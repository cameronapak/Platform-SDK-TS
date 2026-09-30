# Python Platform SDK experiment

This is Cameron Pak's personal Cloudflare Forge experiment, not a YouVersion project or an official or supported YouVersion SDK. It is unpublished, and its API may change without notice.

## Build and install locally

Python 3.11 or newer and [uv](https://docs.astral.sh/uv/) are required. From the repository root:

```sh
uv build packages/python --out-dir packages/python/dist
uv venv .venv
uv pip install packages/python/dist/cameronapak_platform_sdk-0.1.0-py3-none-any.whl
```

The distribution is `cameronapak-platform-sdk`; the import is `cameronapak_platform_sdk`. The package uses HTTPX and Pydantic 2. Responses preserve the API body as native typed models, without Cloudflare-specific envelope handling. Successful empty responses return `None`.

## Use the synchronous client

App-key-only operations do not need an OAuth access token. Inject an HTTPX client to manage connection cleanup explicitly:

```python
import os
import httpx
from cameronapak_platform_sdk import PlatformClient

with httpx.Client() as transport:
    client = PlatformClient(
        yvp_app_key=os.environ["YOUVERSION_APP_KEY"],
        httpx_client=transport,
    )
    page = client.bibles.collection_get(language_ranges=["en"], page_size=25)
    if page is not None and page.next_page_token is not None:
        next_page = client.bibles.collection_get(
            language_ranges=["en"], page_size=25, page_token=page.next_page_token,
        )
```

Pagination is manual. Preserve the original filters when passing a `next_page_token` into the next request's `page_token` argument. Pass `token` when an operation acts on user data; the async client also accepts an `async_token` callable.

## Use the asynchronous client

The async client uses native HTTPX async I/O:

```python
import asyncio
import os
import httpx
from cameronapak_platform_sdk import AsyncPlatformClient

async def main():
    async with httpx.AsyncClient() as transport:
        client = AsyncPlatformClient(
            yvp_app_key=os.environ["YOUVERSION_APP_KEY"],
            httpx_client=transport,
            timeout=10,
            max_retries=2,
        )
        languages = await client.languages.v1languages_collection_get()

asyncio.run(main())
```

Injected transports remain caller-owned. The generated SDK does not expose client context managers; use the HTTPX context managers shown above. A custom transport's timeout remains in effect unless you explicitly configure an SDK timeout.

## Control requests and inspect redirects

Client headers override the generated English locale default, and per-request headers override client headers:

```python
result = client.search_topics.v1search_topics_collection_get(
    query="hope",
    language_ranges=["en"],
    request_options={
        "timeout": 2.0,
        "max_retries": 1,
        "additional_headers": {"Accept-Language": "es"},
    },
)
```

The SDK defaults to two retries and, for its own transport, a 60-second timeout. HTTPX timeout exceptions remain native; API errors expose `status_code`, `headers`, and `body` through `cameronapak_platform_sdk.core.api_error.ApiError`. Retry timing remains the generator's native policy rather than matching TypeScript backoff exactly.

Approval endpoints do not follow callback redirects, even with a redirect-enabled injected transport. Inspect the raw response:

```python
response = client.data_exchange.with_raw_response.approval_post(token="exchange-token")
assert response.status_code == 303
callback = response.headers["location"]
```

Approval GET and token-bearing approval POST omit the SDK's bearer credential. A tokenless approval POST uses the configured access token. The font stylesheet operation sends the app key as the `app_key` query parameter.

## Regenerate and verify

From the repository root:

```sh
pnpm generate:python
pnpm check:python-generated
pnpm check:python
```

Generation needs Node.js 22+, Git, and a running Docker daemon. Root `fern/` configuration pins the CLI and Python generator to the versions used by the tested Forge revision. The authoritative OpenAPI document stays unchanged; shared preparation repairs query examples in a temporary copy. `adapt.mjs` applies narrowly asserted Python compatibility changes, and `operation-map.py` verifies every generated operation against OpenAPI before writing `sdk-map.json`.

Checks build a wheel and sdist, compare wheel contents rebuilt from the sdist, install the wheel non-editably in a clean consumer environment, execute shared wire cases on both clients, and type-check consumer calls. TypeScript consumes the same cases. Automated API requests use loopback servers and synthetic credentials, not the live YouVersion Platform API. CI checks Python 3.11 and 3.14 independently of TypeScript. No workflow publishes the package.
