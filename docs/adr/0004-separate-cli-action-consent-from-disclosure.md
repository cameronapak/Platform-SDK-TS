---
status: accepted
---

# Separate CLI action consent from disclosure

Use an API-shaped Platform CLI that authorizes requests separately from disclosure of their results. This avoids treating a read as public, treating write confirmation as permission to print credentials, or treating a successful approval redirect as proof that permissions were granted; these rules intentionally trade lossless output and convenience for explicit disclosure boundaries.

## Commands and inputs

Derive resource/action names from SDK accessors and methods using the existing React Query naming pattern, converted to kebab case. Initial examples are `yvp bibles collection-get`, `yvp bibles books-resource-get`, `yvp data-exchange token-post`, and `yvp highlights collection-post`; do not add handwritten workflow aliases. Help identifies operationId, HTTP method/path, required inputs, types, and read/write classification.

Use named path/query flags. Remove `[]` from flag names while preserving wire names; repeated flags produce arrays without comma splitting. Parse booleans as explicit `true` or `false`, and preserve integer-or-wildcard inputs. Generate `--accept-language` for declared locale headers and preserve SDK defaults when omitted. Keep API `--format` distinct from CLI `--output`.

JSON bodies use exactly one of `--body-file` or `--body-stdin`. Reject unknown flags, duplicate scalar inputs, body options on bodyless operations, and undeclared body fields at every object depth. The body restriction is deliberately stricter than open OpenAPI object schemas; do not alter the authoritative document. Do not silently insert examples, generate request IDs, or allow body fields to overwrite path, query, or credential inputs.

## Credentials and authentication

Secret values come only from documented environment inputs: `YOUVERSION_APP_KEY`, `YOUVERSION_ACCESS_TOKEN`, `YOUVERSION_DATA_EXCHANGE_TOKEN`, and optional distinct query-context `YOUVERSION_DATA_EXCHANGE_APP_KEY`. Allow a nonsecret app-id input, but no credential-valued flags, config persistence, login command, arbitrary headers, or query passthrough. Initially require an app key for every execution to satisfy SDK construction, including approval operations whose OpenAPI security does not require one.

Construct an operation-scoped SDK client. Disable OAuth with `auth: false` for ordinary app-key operations and exchange-token approval; supply it only where the contract requires it, including declared Authorization headers. Approval GET requires a data exchange token. Approval POST infers token versus OAuth mode only when exactly one credential is locally present; require `--auth-mode exchange-token|oauth` when both exist, and reject a selected mode whose credential is missing. Local presence does not verify validity or expiry. Preserve SDK wire placement, including stylesheet app-key query authentication.

## Consent and output

- Token issuance, approval POST, highlight upsert, and highlight deletion require write consent. Prompt only with interactive stdin and stderr, defaulting to refusal; automation and body-stdin writes require `--yes`. Validate before prompting. Help, catalog, and version do not consume stdin, validate credentials, construct a client, or send requests.
- `--yes` never implies `--show-sensitive`. Highlights, user permissions, approval HTML, and full callback Location are sensitive results. Without disclosure selection, their default output is metadata-only; revealing request status is intentional. Token issuance requires `--show-sensitive` before dispatch so its one-time result is not silently discarded.
- Default stdout is a JSON envelope containing `operationId`, `status`, `kind` (`json`, `text`, or `empty`), and `data`, with optional `location` and `redacted`. Empty or withheld data is `null`; redaction is explicit. Withheld Location is `null` without exposing extracted callback pieces.
- `--output text` emits guarded raw text only for ordinary text operations, not approvals or JSON/empty operations. Approval always retains the JSON envelope so status and Location remain representable. No automatic passage-content extraction.
- Never print configured credential values, even with disclosure selection or when unused by that operation. Project allowed output before guarding JSON keys/values, text, Location, and diagnostics; cover literal, URI-encoded, and JSON-escaped forms. Do not promise recognition of arbitrary unknown transformations. Disable SDK logging; never dump raw SDK messages, bodies, stacks, or arbitrary server prose. Validation errors identify the field/rule without echoing its value. Human diagnostics escape terminal controls.

Successful declared approval redirects exit zero, whether the callback represents granted, cancelled, or error; zero means request completion, not permission grant. Expose guarded Location only with disclosure selection, never follow the callback, execute HTML, or open a browser. Other redirects retain SDK behavior; a validated custom base URL is a trusted initial destination, not a redirect-chain network sandbox. Allow HTTPS and loopback HTTP, rejecting URL credentials, query, and fragment.

## Execution and failures

Default to a 30-second execution deadline, configurable with finite positive `--timeout-seconds` up to 300. Start after input validation and confirmation; keep an AbortController active through body consumption and forward its signal to the SDK. Human input is outside that deadline. Set SDK retries to zero and add no CLI retries or retry flag; one SDK invocation need not mean one HTTP exchange when ordinary redirects occur.

Keep stdout empty on failures. Send structured stderr in JSON mode and actionable human diagnostics in text mode, preserving allowlisted SDK error class and real HTTP status without unsafe messages. Exit 1 for SDK, transport, deadline, or output-delivery failure; 2 for input, credential selection, or confirmation refusal; 130 for SIGINT; and 143 for SIGTERM. Do not report synthetic SDK abort status 499 as an API response.

Before dispatch, interruption means no CLI request was dispatched. After dispatch without a definitive response, describe mutation outcome as unknown, not rolled back or safe to retry. If a result was received but output failed, distinguish delivery failure from unknown server outcome; one-time token delivery cannot be guaranteed.
