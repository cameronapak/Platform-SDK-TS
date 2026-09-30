from __future__ import annotations

import asyncio
from collections import deque
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import importlib.resources
import inspect
import json
import os
from pathlib import Path
import sys
import threading
import time
import unittest
from urllib.parse import parse_qs, urlsplit

import httpx
import cameronapak_platform_sdk as sdk
from cameronapak_platform_sdk.core.api_error import ApiError

REPOSITORY = Path(os.environ["SDK_REPOSITORY"]).resolve()


class Handler(BaseHTTPRequestHandler):
    def do_GET(self): self.handle_request()
    def do_POST(self): self.handle_request()
    def do_DELETE(self): self.handle_request()

    def handle_request(self):
        raw = self.rfile.read(int(self.headers.get("content-length", "0")))
        url = urlsplit(self.path)
        self.server.requests.append({
            "method": self.command, "path": url.path,
            "query": parse_qs(url.query, keep_blank_values=True),
            "headers": {key.lower(): value for key, value in self.headers.items()},
            "body": json.loads(raw) if raw else None,
        })
        response = self.server.responses.popleft() if self.server.responses else {"status": 500, "text": "unexpected request"}
        time.sleep(response.get("delay", 0))
        body = json.dumps(response["body"]).encode() if "body" in response else response.get("text", "").encode()
        self.send_response(response["status"])
        headers = {"content-type": "application/json" if "body" in response else "text/plain", **response.get("headers", {})}
        for name, value in headers.items():
            self.send_header(name, value)
        self.send_header("content-length", str(len(body)))
        self.end_headers()
        try:
            self.wfile.write(body)
        except (BrokenPipeError, ConnectionResetError):
            pass  # The timeout test deliberately closes its connection.

    def log_message(self, *_args):
        pass


@contextmanager
def recording_server(responses):
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    server.responses = deque(responses)
    server.requests = []
    thread = threading.Thread(target=lambda: server.serve_forever(poll_interval=0.01))
    thread.start()
    try:
        yield server, f"http://127.0.0.1:{server.server_port}"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


async def invoke(call, *args, **arguments):
    result = call(*args, **arguments)
    return await result if inspect.isawaitable(result) else result


class InstalledConsumerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.cases = json.loads((REPOSITORY / "test/conformance/cases.json").read_text())
        cls.mapping = json.loads(importlib.resources.files(sdk).joinpath("sdk-map.json").read_text())

    def test_installed_inventory(self):
        location = Path(inspect.getfile(sdk)).resolve()
        self.assertTrue(location.is_relative_to(Path(sys.prefix)), location)
        self.assertFalse(location.is_relative_to(REPOSITORY), location)
        self.assertTrue(importlib.resources.files(sdk).joinpath("py.typed").is_file())
        spec = json.loads((REPOSITORY / "openapi/openapi.json").read_text())
        operations = {
            operation["operationId"]: (method.upper(), path)
            for path, item in spec["paths"].items() for method, operation in item.items()
            if method in {"get", "post", "delete", "put", "patch", "head", "options", "trace"}
        }
        self.assertEqual(33, len(operations))
        self.assertEqual(set(operations), set(self.mapping))
        self.assertEqual(set(operations), {case["operationId"] for case in self.cases})
        for name, (method, path) in operations.items():
            self.assertEqual((method, path), (self.mapping[name]["httpMethod"], self.mapping[name]["path"]))

    async def check_cases(self, asynchronous):
        for case in self.cases:
            with self.subTest(surface="async" if asynchronous else "sync", case=case["name"]):
                with recording_server([case["response"]]) as (server, base_url):
                    transport = httpx.AsyncClient(follow_redirects=True, trust_env=False) if asynchronous else httpx.Client(follow_redirects=True, trust_env=False)
                    config = case["client"]
                    client_type = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                    client = client_type(base_url=base_url, yvp_app_key=config["appKey"], token=config.get("token"), headers=config.get("headers"), max_retries=0, httpx_client=transport)
                    try:
                        mapping = self.mapping[case["operationId"]]
                        owner = getattr(client, mapping["accessor"][0])
                        status = case["response"]["status"]
                        if status == 303:
                            owner = owner.with_raw_response
                        call = getattr(owner, mapping["method"])
                        arguments = {mapping["parameters"][name]: value for name, value in case["parameters"].items()}
                        if "highlight" in arguments:
                            from cameronapak_platform_sdk.highlights import V1HighlightsCollectionPostRequestHighlight
                            arguments["highlight"] = V1HighlightsCollectionPostRequestHighlight(**arguments["highlight"])
                        if case.get("requestHeaders"):
                            arguments["request_options"] = {"additional_headers": case["requestHeaders"]}
                        if status >= 300 and status != 303:
                            with self.assertRaises(ApiError) as raised:
                                await invoke(call, **arguments)
                            self.assertEqual(status, raised.exception.status_code)
                            self.assertEqual(case["response"].get("body", case["response"].get("text")), raised.exception.body)
                        else:
                            result = await invoke(call, **arguments)
                            if status == 303:
                                self.assertEqual(303, result.status_code)
                                self.assertEqual(case["response"]["headers"]["location"], result.headers["location"])
                                result = result.data
                            if hasattr(result, "model_dump"):
                                result = result.model_dump(by_alias=True, exclude_unset=True, mode="json")
                            self.assertEqual(case["response"].get("body", case["response"].get("text")), result)
                        self.assertEqual(1, len(server.requests), "callback redirect or unexpected retry")
                        actual = server.requests[0]
                        expected = case["expect"]
                        for field in ["method", "path", "query"]:
                            self.assertEqual(expected[field], actual[field], field)
                        for key, value in expected["headers"].items():
                            self.assertEqual(value, actual["headers"].get(key.lower()), key)
                        self.assertEqual(expected.get("body"), actual["body"])
                    finally:
                        if asynchronous:
                            await transport.aclose()
                        else:
                            transport.close()

    def test_shared_cases_sync_and_async(self):
        asyncio.run(self.check_cases(False))
        asyncio.run(self.check_cases(True))

    def test_retry_limits_and_timeout_on_both_surfaces(self):
        async def check(asynchronous):
            responses = [
                {"status": 429, "headers": {"Retry-After": "0.001"}, "body": {"message": "busy"}},
                {"status": 204},
                {"status": 503, "headers": {"Retry-After": "0.001"}, "text": "unavailable"},
                {"status": 503, "text": "still unavailable"},
                {"status": 204, "delay": 0.15},
            ]
            with recording_server(responses) as (server, base_url):
                transport = httpx.AsyncClient(trust_env=False) if asynchronous else httpx.Client(trust_env=False)
                cls = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                client = cls(base_url=base_url, yvp_app_key="synthetic-key", token="synthetic-token", max_retries=3, httpx_client=transport)
                call = client.highlights.v1highlights_resource_delete
                try:
                    self.assertIsNone(await invoke(call, "MAT.1.1", bible_id=1, request_options={"max_retries": 1}))
                    with self.assertRaises(ApiError) as raised:
                        await invoke(call, "MAT.1.2", bible_id=1, request_options={"max_retries": 1})
                    self.assertEqual(503, raised.exception.status_code)
                    self.assertEqual("still unavailable", raised.exception.body)
                    self.assertEqual(4, len(server.requests))
                    with self.assertRaises(httpx.TimeoutException):
                        await invoke(call, "MAT.1.3", bible_id=1, request_options={"timeout": 0.02, "max_retries": 0})
                    self.assertEqual(5, len(server.requests))
                finally:
                    if asynchronous: await transport.aclose()
                    else: transport.close()
        asyncio.run(check(False))
        asyncio.run(check(True))

    def test_token_supplier_selection_and_suppression(self):
        async def check(asynchronous, use_async_supplier=False):
            calls = []
            unavailable = True

            def sync_supplier():
                calls.append("sync")
                if unavailable or use_async_supplier:
                    raise RuntimeError("sync token unavailable")
                return "sync-access-token"

            async def async_supplier():
                calls.append("async")
                if unavailable:
                    raise RuntimeError("async token unavailable")
                return "async-access-token"

            responses = [
                {"status": 503, "headers": {"Retry-After": "0"}},
                {"status": 200, "text": "approval page"},
                {"status": 303, "headers": {"location": "/callback"}},
                {"status": 303, "headers": {"location": "/callback"}},
                {"status": 200, "text": "@font-face {}"},
                {"status": 204},
            ]
            with recording_server(responses) as (server, base_url):
                transport = httpx.AsyncClient(follow_redirects=True, trust_env=False) if asynchronous else httpx.Client(follow_redirects=True, trust_env=False)
                cls = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                options = {"async_token": async_supplier} if use_async_supplier else {}
                client = cls(base_url=base_url, yvp_app_key="supplier-app-key", token=sync_supplier, max_retries=1, httpx_client=transport, **options)
                try:
                    self.assertEqual("approval page", await invoke(client.data_exchange.approval_get, token="exchange-get"))
                    response = await invoke(client.data_exchange.with_raw_response.approval_post, token="exchange-post")
                    self.assertEqual(303, response.status_code)
                    self.assertEqual([], calls, "exchange approval must not acquire OAuth, including retries")
                    self.assertEqual(3, len(server.requests))
                    for request in server.requests:
                        self.assertNotIn("authorization", request["headers"])

                    selected = "async" if use_async_supplier else "sync"
                    with self.assertRaisesRegex(RuntimeError, f"{selected} token unavailable"):
                        await invoke(client.highlights.v1highlights_resource_delete, "MAT.1.1", bible_id=1)
                    self.assertEqual([selected], calls)
                    self.assertEqual(3, len(server.requests), "authenticated calls must propagate token failures")
                    calls.clear()
                    unavailable = False

                    response = await invoke(client.data_exchange.with_raw_response.approval_post)
                    self.assertEqual(303, response.status_code)
                    self.assertEqual([selected], calls)
                    self.assertEqual("@font-face {}", await invoke(client.fonts.v1fonts_stylesheet_get, 42))
                    self.assertEqual([selected, selected], calls, "stylesheet app-key lookup must not acquire an extra token")
                    self.assertEqual({"app_key": ["supplier-app-key"]}, server.requests[4]["query"])
                    self.assertIsNone(await invoke(client.highlights.v1highlights_resource_delete, "MAT.1.1", bible_id=1))
                    self.assertEqual([selected] * 3, calls)
                    for request in server.requests[3:]:
                        self.assertEqual(f"Bearer {selected}-access-token", request["headers"]["authorization"])
                finally:
                    if asynchronous: await transport.aclose()
                    else: transport.close()

        for asynchronous, use_async_supplier in [(False, False), (True, False), (True, True)]:
            with self.subTest(asynchronous=asynchronous, async_supplier=use_async_supplier):
                asyncio.run(check(asynchronous, use_async_supplier))

    def test_non_approval_redirects_keep_transport_policy(self):
        async def check(asynchronous):
            responses = [
                {"status": 302, "headers": {"location": "/redirected-stylesheet"}},
                {"status": 200, "text": "redirected CSS"},
            ]
            with recording_server(responses) as (server, base_url):
                transport = httpx.AsyncClient(follow_redirects=True, trust_env=False) if asynchronous else httpx.Client(follow_redirects=True, trust_env=False)
                cls = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                client = cls(base_url=base_url, yvp_app_key="redirect-app-key", max_retries=0, httpx_client=transport)
                try:
                    self.assertEqual("redirected CSS", await invoke(client.fonts.v1fonts_stylesheet_get, 42))
                    self.assertEqual(["/v1/fonts/42/stylesheet", "/redirected-stylesheet"], [request["path"] for request in server.requests])
                finally:
                    if asynchronous: await transport.aclose()
                    else: transport.close()
        asyncio.run(check(False))
        asyncio.run(check(True))

    def test_empty_error_body_is_not_success(self):
        async def check(asynchronous):
            with recording_server([{"status": 403}]) as (server, base_url):
                transport = httpx.AsyncClient(trust_env=False) if asynchronous else httpx.Client(trust_env=False)
                cls = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                client = cls(base_url=base_url, yvp_app_key="synthetic-key", max_retries=0, httpx_client=transport)
                try:
                    with self.assertRaises(ApiError) as raised:
                        await invoke(client.bibles.collection_get, language_ranges=["en"])
                    self.assertEqual(403, raised.exception.status_code)
                    self.assertEqual("", raised.exception.body)
                finally:
                    if asynchronous: await transport.aclose()
                    else: transport.close()
        asyncio.run(check(False))
        asyncio.run(check(True))

    def test_logging_redaction_and_async_token_supplier(self):
        class Capture:
            def __init__(self): self.entries = []
            def debug(self, message, **kwargs): self.entries.append((message, kwargs))
            info = debug
            warn = debug
            error = debug

        async def check(asynchronous):
            logger = Capture()
            with recording_server([{"status": 200, "text": "@font-face {}"}, {"status": 403, "body": {"message": "denied"}}]) as (server, base_url):
                transport = httpx.AsyncClient(trust_env=False) if asynchronous else httpx.Client(trust_env=False)
                cls = sdk.AsyncPlatformClient if asynchronous else sdk.PlatformClient
                options = {"token": "synthetic-access-token"}
                if asynchronous:
                    async def supplier(): return "synthetic-access-token"
                    options = {"async_token": supplier}
                client = cls(base_url=base_url, yvp_app_key="synthetic-app-secret", max_retries=0, httpx_client=transport, logging={"level": "debug", "silent": False, "logger": logger}, **options)
                try:
                    self.assertEqual("@font-face {}", await invoke(client.fonts.v1fonts_stylesheet_get, 42))
                    with self.assertRaises(ApiError) as raised:
                        await invoke(client.languages.v1languages_collection_get)
                    self.assertEqual("Bearer synthetic-access-token", server.requests[1]["headers"]["authorization"])
                    self.assertTrue(logger.entries)
                    rendered = json.dumps(logger.entries) + str(raised.exception)
                    self.assertNotIn("synthetic-app-secret", rendered)
                    self.assertNotIn("synthetic-access-token", rendered)
                finally:
                    if asynchronous: await transport.aclose()
                    else: transport.close()
        asyncio.run(check(False))
        asyncio.run(check(True))


if __name__ == "__main__":
    unittest.main()
