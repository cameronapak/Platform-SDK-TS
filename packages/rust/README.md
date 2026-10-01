# Rust Platform SDK experiment

This is Cameron Pak's personal Cloudflare Forge experiment, not a YouVersion project or an official or supported YouVersion SDK. It is unpublished, and its API may change without notice.

## Consume locally

Use a Cargo path dependency. The crate uses Rust edition 2021, Tokio, reqwest 0.12, and Serde. Checks run on Rust 1.99.0; no older minimum Rust version is promised.

```toml
[dependencies]
cameronapak_platform_sdk = { path = "/absolute/path/to/Unofficial-YouVersion-Platform-SDKs/packages/rust" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

`pnpm check:rust` also creates and validates a local `.crate` artifact under `.cache/rust-target/package`. The crate has `publish = false`; no registry release is needed.

## Use the client

App-key-only operations do not need an access token. The generated `api_key` setting represents your app key and sends `X-YVP-App-Key`:

```rust
use cameronapak_platform_sdk::*;
use std::time::Duration;

async fn read_bibles() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClientBuilder::default()
        .api_key(std::env::var("YOUVERSION_APP_KEY")?)
        .timeout(Duration::from_secs(15))
        .build()?;
    let request = CollectionGetQueryRequest {
        language_ranges_array: vec![Some("en".into())],
        page_size: Some(BiblesCollectionGetRequestPageSize::Numeric(25)),
        ..Default::default()
    };
    if let Some(page) = client.bibles.collection_get(&request, None).await? {
        let next_page_token = page.next_page_token;
    }
    Ok(())
}
```

Pass `.token(access_token)` for user operations. Client and per-request options retain Fern's generated names and builder patterns. Pagination is manual: pass `next_page_token` as `page_token`, preserving the original filters. Numeric/wildcard page sizes use `Numeric(25)` or `All`; deserialization and `FromStr` validate the 1 through 99 range and `"*"`.

Search text is sent as a literal query value, including colons, commas, and quotes. String path identifiers are encoded as one segment. Whole-segment `.` and `..` return `ApiError::Configuration` before any request is sent; dotted passage IDs remain valid.

## Configure transport and requests

Use `.transport(move || reqwest::Client::builder()...)` to configure proxy, TLS roots, or other reqwest settings. Add reqwest 0.12 to your application dependencies when naming its types. The SDK builds regular and approval transports from the same factory, applies its timeout and user agent, and disables redirects only on the approval transport. It does not mutate caller-owned clients. The default TLS backend is rustls with certificate verification enabled.

Configure API credentials through SDK authentication settings and headers through SDK client/request options. Do not inject API credentials through reqwest default headers. The SDK cannot inspect or remove those defaults; reqwest can add them after SDK suppression. That configuration is unsupported and can send credentials on approval requests. TLS client identities and proxy authentication remain transport settings.

Client timeout defaults to 60 seconds per attempt. `RequestOptions::new().timeout_seconds(n)` overrides it for one call. For an overall deadline, including retry waits, wrap the call in `tokio::time::timeout`. Dropping or aborting the request future cancels pending HTTP work and retry waits.

Caller locale headers override generated English defaults. Per-request headers override client headers case-insensitively:

```rust
let options = RequestOptions::new()
    .additional_header("Accept-Language", "es-MX")
    .max_retries(0);
let languages = client.languages.v1languages_collection_get(
    &V1LanguagesCollectionGetQueryRequest::default(), Some(options),
).await?;
```

Native retries default to three retries after the initial attempt. They cover network errors, HTTP 408, 429, and 5xx responses. Per-request `max_retries` overrides the client, including zero. Backoff starts at 100 milliseconds and doubles, saturating at 30 seconds to prevent overflow. `Retry-After` is not implemented. If your application already retries, disable SDK retries. Use zero retries for single-use approval submissions when repeating an uncertain request is unsafe.

The native hidden `RequestExecutor` integration remains available, but delegates authentication, headers, retries, TLS, and redirect control entirely to its implementer. Prefer the transport factory for normal SDK use.

## Inspect approvals, responses, and errors

Approval GET and token-bearing approval POST suppress Authorization supplied through SDK authentication and SDK client/request headers, and do not acquire an OAuth token. Suppression also recognizes tokens supplied through additional query parameters. Tokenless approval POST uses the configured access token. The font stylesheet sends your app key in the `app_key` query parameter.

```rust
let response = client.data_exchange.approval_post_with_raw_response(
    &ApprovalPostQueryRequest::builder().token(exchange_token).build()?,
    Some(RequestOptions::new().max_retries(0)),
).await?;
assert_eq!(response.status_code, 303);
let callback = response.headers.get("location");
```

Approval raw-response methods expose status, headers, and the parsed body without following callbacks. Approval GET returns `Option<String>` because its successful responses include HTML and an empty redirect. Other methods return typed API bodies directly, with no Cloudflare envelope handling. Required text responses preserve an empty string; optional empty responses return `None`. Operations with no declared response body return `()`, discarding any successful response text while preserving raw metadata.

`ApiError::status_code()` and `body()` expose non-success HTTP status and the original JSON or text error body. Network errors retain reqwest's classification but omit URLs from formatted errors to avoid exposing query credentials. There is no SDK request logging. Custom executors, response bodies, and callback locations remain the caller's confidentiality responsibility.

Serde ignores unknown response properties. Typed models do not promise lossless JSON reserialization, including nullable fields. Declared fields remain typed and are checked through consumer usage and response assertions against the OpenAPI schema.

## Regenerate and verify

```sh
pnpm generate:rust
pnpm check:rust-generated
pnpm check:rust
```

Generation requires Node.js 22+, Rust with rustfmt, and a running Docker daemon. Checks also require Clippy, OpenSSL, and tar. Root `fern/` retains Forge's Fern CLI `5.112.0` and Rust generator `0.42.1`. HTTPS uses the generator's supported `extraDependencies` setting; contract gaps use counted adaptations in `adapt.mjs`. The authoritative OpenAPI stays unchanged.

Three adaptations address failures in the pinned generator and runtime:

- Search: Fern's `structured_query` helper can turn literal text such as `John3:16` into unrelated query parameters. Search operations use a literal string instead.
- Responses: the default parser rejects successful empty required text and tries to parse undeclared response bodies as JSON. The adaptation preserves empty text and discards bodies for unit responses, including approval redirects, without losing status or headers.
- Paths: URL parsing can normalize whole-segment `.` and `..` into a different request path. The adaptation encodes each string identifier as one segment and rejects dot segments before sending.

Generation validates each operation's method, path, and parameters before producing `sdk-map.json` and consumer dispatch. Dependency locks are checked in and reused during regeneration. Checks package the crate, extract it into a temporary directory, resolve it from a separate consumer, run all shared wire cases, and exercise TLS, redirects, authentication, retries, cancellation, timeouts, and typed builders. Consumer examples compile as documentation tests; generated Fern utility doctests remain disabled by its default. Tests use loopback HTTP/HTTPS and synthetic credentials, never the live YouVersion Platform API.

Clippy checks the library and all consumer targets. Lint and documentation warnings from generated code remain visible. Full-crate Clippy fails on Fern's numeric utility test fixtures (`approx_constant`); those fixtures remain unchanged and run under `cargo test` instead.
