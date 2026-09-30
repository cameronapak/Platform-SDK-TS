# Wire conformance cases

`cases.json` is language-neutral and is consumed by the TypeScript and Python installed-package tests. Its root is an array. Each item has:

```text
{ name, operationId, client, parameters, requestHeaders?, response, expect }
```

`parameters` uses the original OpenAPI wire names, including names such as `fields[]`, and flattened JSON-body properties. Expected query values are arrays of strings. Expected headers are a case-insensitive subset; `null` means the header must be absent. `expect.body` is included only when a JSON request body is expected. Responses use either `body` for JSON or `text` for text, and successful empty or redirect results normalize to `null` across languages.

Cases intentionally use fake credentials and loopback-only callback URLs. Shared cases test request/response wire behavior, not retry timing or transport failures.
