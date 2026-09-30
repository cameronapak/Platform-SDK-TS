# Go Platform SDK experiment

This is Cameron Pak's personal Cloudflare Forge experiment, not a YouVersion project or an official or supported YouVersion SDK. It has no versioned release, and its API may change without notice.

## Consume locally

Go 1.24 or newer is required. The module is `github.com/cameronapak/Platform-SDK-TS/packages/go`. In your consumer module, use a local replacement pointing to your checkout:

```sh
go mod edit -require=github.com/cameronapak/Platform-SDK-TS/packages/go@v0.0.0
go mod edit -replace=github.com/cameronapak/Platform-SDK-TS/packages/go=/absolute/path/to/Platform-SDK-TS/packages/go
go mod tidy
```

Add your imports before running `go mod tidy`. No repository tag or public module release is needed.

## Use the client

App-key-only operations do not need an access token. Reuse a caller-owned HTTP client to configure transport and timeout behavior:

```go
package example

import (
    "context"
    "net/http"
    "os"
    "time"

    platform "github.com/cameronapak/Platform-SDK-TS/packages/go"
    "github.com/cameronapak/Platform-SDK-TS/packages/go/client"
    "github.com/cameronapak/Platform-SDK-TS/packages/go/option"
)

func readBibles(ctx context.Context) (*platform.BiblesCollectionGetResponse, error) {
    transport := &http.Client{Timeout: 30 * time.Second}
    sdk := client.NewPlatformClient(
        option.WithYvpAppKey(os.Getenv("YOUVERSION_APP_KEY")),
        option.WithHTTPClient(transport),
    )
    return sdk.Bibles.CollectionGet(ctx, &platform.BiblesCollectionGetRequest{
        LanguageRangesArray: []*string{platform.String("en")},
        PageSize: platform.BiblesCollectionGetRequestPageSize("25").Ptr(),
    })
}
```

Pass `option.WithToken(accessToken)` for user operations. `option.WithTokenFunc` acquires a token at request time and propagates supplier failures. A per-request token or token supplier overrides the client credential without acquiring the overridden supplier.

Every operation takes a `context.Context`. Context cancellation interrupts in-flight HTTP calls and retry waits. Use `context.WithTimeout` for an overall deadline, including retries. Without an injected HTTP client or context deadline, the generator uses `http.DefaultClient`, which has no timeout.

## Control requests and inspect redirects

Client headers override generated locale defaults, and request headers override client headers case-insensitively:

```go
response, err := sdk.Languages.V1LanguagesCollectionGet(ctx,
    &platform.V1LanguagesCollectionGetRequest{},
    option.WithHTTPHeader(http.Header{"Accept-Language": {"es-MX"}}),
    option.WithoutRetries(),
)
```

The native retry policy defaults to two total attempts for HTTP 408, 429, and 5xx responses. `option.WithMaxAttempts(n)` sets total attempts, not retries. `option.WithoutRetries()` makes exactly one attempt, including when configured on the client. A positive per-request maximum overrides the client retry policy. Transport errors are returned without SDK retries. Keep retry ownership with the application when it already retries requests.

Approval operations do not follow redirects when using the default or an injected `*http.Client`. The SDK preserves the caller's transport and redirect policy for other endpoints. If you supply another implementation of `core.HTTPClient`, its `Do` method must return redirects without following them; the SDK cannot control redirects hidden inside a custom implementation. Prefer a custom `http.RoundTripper` inside `*http.Client` when you need custom transport behavior.

```go
response, err := sdk.DataExchange.WithRawResponse.ApprovalPost(ctx,
    &platform.DataExchangeApprovalPostRequest{Token: platform.String(exchangeToken)},
)
if err != nil {
    return err
}
callback := response.Header.Get("Location")
```

Approval GET and token-bearing approval POST suppress bearer authentication and do not acquire an access token. Tokenless approval POST uses the configured access token. The font stylesheet operation puts the app key in the `app_key` query parameter.

## Responses, errors, and pagination

Methods return typed API bodies directly, without Cloudflare envelope handling. Optional empty responses return `nil`; delete and approval-submit methods return an error only. `WithRawResponse` also exposes status and headers, without transferring HTTP response-body cleanup to the caller.

Use `errors.As(err, &apiError)` with `var apiError *core.APIError` to inspect `StatusCode`, `Header`, and `Body`. `Body` retains the JSON value or text. Transport errors preserve `errors.Is` and `errors.As`, with known query credentials redacted from their URL. There is no SDK request logging. Custom transports and token suppliers own the confidentiality of their logs and errors; API response bodies and callback locations can also contain sensitive data.

Unknown response properties are available through `GetExtraProperties()`. Go's generated JSON marshaling can omit unknown fields and optional nulls; it is not a lossless copy of the wire response.

Pagination is manual. Pass a returned `NextPageToken` into the next request's `PageToken`, preserving the other filters. Mixed numeric/wildcard page sizes use named string types, such as `platform.BiblesCollectionGetRequestPageSize("25").Ptr()` or `platform.BiblesCollectionGetRequestPageSizeAll.Ptr()`. Numeric-only page sizes use `*int`.

## Regenerate and verify

From the repository root:

```sh
pnpm generate:go
pnpm check:go-generated
pnpm check:go
```

Generation needs Node.js 22+, Go, and a running Docker daemon. Root `fern/` pins CLI `5.112.0` and Go generator `1.47.2` to the tested Forge revision. These pins generated successfully locally without Fern login or a token; newer Fern versions may have different access requirements. Cold generation downloads the CLI, generator image, and Go dependencies.

Preparation edits a temporary copy of the authoritative OpenAPI document. `adapt.mjs` applies counted compatibility adaptations, and `operation-map.mjs` verifies every operation's generated method, request parameters, HTTP method, and path before writing `sdk-map.json`. Regeneration formats the output and checks module metadata. Two existing Fern warnings concern explicit Authorization headers that duplicate security schemes; auth and approval behavior are covered by consumer tests.

Checks copy the module into a temporary directory, resolve it from a separate consumer module with `GOWORK=off`, build and vet it, and execute all shared wire cases plus Go-specific runtime tests under the race detector. CI checks Go 1.24 and 1.27, with generation in a separate job. Tests use loopback servers and synthetic credentials, not the live YouVersion Platform API. No workflow publishes or tags this module.
