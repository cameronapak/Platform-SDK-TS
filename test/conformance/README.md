# Wire conformance cases

`cases.json` is language-neutral and is consumed by the TypeScript, Python, Go, and Rust consumer tests. Its root is an array. Each item has:

```text
{ name, operationId, client, parameters, requestHeaders?, response, expect }
```

`parameters` uses the original OpenAPI wire names, including names such as `fields[]`, and flattened JSON-body properties. Expected query values are arrays of strings. Expected headers are a case-insensitive subset; `null` means the header must be absent. `expect.body` is included only when a JSON request body is expected. Responses use either `body` for JSON or `text` for text.

The existing empty-body and bodyless redirect fixtures compare results as `null` across languages. This is a fixture convention, not a promise that every empty SDK response becomes `null`. For example, Rust returns `""` for required empty text, `None` for optional empty responses, and `()` for operations with no declared response body.

Go converts numeric fixture values for mixed numeric/wildcard page sizes into its generated named string types. Its response checks inspect typed fields, literal fields, and `GetExtraProperties()`, rather than requiring lossless JSON re-serialization of optional nulls. Redirect tests substitute a local callback location and assert that it is returned without being requested.

Rust uses numeric/wildcard enums for page sizes and Serde's native unknown-property behavior. Its response assertions verify declared fields without requiring lossless reserialization of optional nulls. Its separate consumer compiles calls to every operation and consumes the extracted `.crate` artifact; extra loopback tests cover HTTPS, transport configuration, redirects, retries, timeouts, and cancellation.

Cases intentionally use fake credentials and loopback-only callback URLs. Shared cases test request/response wire behavior, not retry timing or transport failures.
