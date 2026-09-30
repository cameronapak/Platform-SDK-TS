"""Match generated request methods to the authoritative OpenAPI inventory."""

import ast
import json
from pathlib import Path
import re
import sys


def request_path(node):
    if isinstance(node, ast.Constant):
        return "/" + node.value
    if isinstance(node, ast.JoinedStr):
        result = "/"
        for part in node.values:
            if isinstance(part, ast.Constant):
                result += part.value
            elif isinstance(part, ast.FormattedValue) and isinstance(part.value, ast.Call):
                assert isinstance(part.value.func, ast.Name) and part.value.func.id == "encode_path_param"
                result += "{" + part.value.args[0].id + "}"
            else:
                raise ValueError(f"Unexpected generated path: {ast.dump(part)}")
        return result
    raise ValueError(f"Unexpected generated path: {ast.dump(node)}")


def main():
    source, spec_file = map(Path, sys.argv[1:])
    spec = json.loads(spec_file.read_text())
    requests = {}
    for file in sorted(source.glob("*/raw_client.py")):
        for cls in ast.parse(file.read_text()).body:
            if not isinstance(cls, ast.ClassDef) or cls.name.startswith("Async"):
                continue
            for method in cls.body:
                if not isinstance(method, ast.FunctionDef):
                    continue
                for node in ast.walk(method):
                    if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "request":
                        verb = next(item.value.value for item in node.keywords if item.arg == "method")
                        key = (verb, request_path(node.args[0]))
                        if key in requests:
                            raise ValueError(f"Duplicate generated operation: {key}")
                        arguments = {arg.arg for arg in [*method.args.args, *method.args.kwonlyargs]}
                        requests[key] = (file.parent.name, method.name, arguments)

    mapping = {}
    for path, item in spec["paths"].items():
        for verb, operation in item.items():
            if verb not in {"get", "put", "post", "delete", "options", "head", "patch", "trace"}:
                continue
            resource, method, arguments = requests.pop((verb.upper(), path))
            parameters = {}
            for parameter in [*item.get("parameters", []), *operation.get("parameters", [])]:
                if "$ref" in parameter:
                    parameter = spec["components"]["parameters"][parameter["$ref"].split("/")[-1]]
                if parameter["in"] == "header":
                    continue
                name = re.sub(r"[^a-zA-Z0-9_]+", "_", parameter["name"]).strip("_")
                if name not in arguments:
                    raise ValueError(f"Missing generated argument {name} for {operation['operationId']}")
                parameters[parameter["name"]] = name
            body = operation.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {})
            for name in body.get("properties", {}):
                if name not in arguments:
                    raise ValueError(f"Missing generated body argument {name}")
                parameters[name] = name
            mapping[operation["operationId"]] = {
                "accessor": [resource], "method": method,
                "httpMethod": verb.upper(), "path": path, "parameters": parameters,
            }
    if requests:
        raise ValueError(f"Unexpected generated operations: {requests.keys()}")
    (source / "sdk-map.json").write_text(json.dumps(mapping, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
