---
status: accepted
---

# Generate the Platform CLI over the existing SDK

For [issue #18](https://github.com/cameronapak/Unofficial-YouVersion-Platform-SDKs/issues/18), use a separate `@cameronapak/platform-cli` package with a `yvp` executable, generated from the TypeScript Platform SDK inventory and authoritative OpenAPI. Forge's pinned source provides SDK metadata and transformer extension points, but no reusable CLI emitter; a local generator following the React Query generation pattern avoids a new pipeline dependency, a second handwritten endpoint inventory, and a competing HTTP runtime.

## Generation boundary

- Join every OpenAPI operation against `src/generated/sdk-map.json`; currently there are 33. Do not use the request-type union as the inventory because it omits two requestless operations.
- Generate statically typed SDK calls and runtime argument/help metadata. OpenAPI owns input constraints, descriptions, and declared authentication; generated SDK signatures own invocation shape. Permitted SDK type widening is not a mismatch.
- Support all current operations for their declared inputs. Future exclusions must remain discoverable with reasons. Approval submission does not imply support for browser-form actions absent from OpenAPI.
- Keep a sparse operation-specific policy for credential sourcing, disclosure, and exceptional side effects. Check an explicitly reviewed fingerprint of OpenAPI, SDK binding shapes, and policy so source changes cannot silently gain a safe classification. Ordinary regeneration does not approve a changed fingerprint; this is not remote-service drift detection.
- Fail deterministically on inventory, method/path, binding, unsupported-schema, and command/flag collision drift. Keep the authoritative OpenAPI and shared generator pins unchanged.

Use Node.js 22 or newer, TypeScript, and generated option definitions with Node's argument parser. Use pinned Ajv 8 and compatible `ajv-formats` for input validation, without coercion, default insertion, or property removal. Normalize only explicitly supported OpenAPI constructs, validate formats including UUID and int32, reject unsafe integers, and fail generation on unsupported validation keywords. Preserve prose-only constraints in help without building a second implementation of server validation.

## Verification and delivery boundary

Add separate CLI generation/check scripts and independent CI validation. Pack both the SDK and CLI, install their artifacts into a clean consumer, and execute the installed binary with fake environment credentials and loopback fixtures. Cover every command and shared wire case, then falsify confirmation, disclosure, typed-input, authentication-selection, locale, error, stalled-body deadline, interruption, and unfollowed-approval-callback behavior independently of generated expectations.

The tool remains experimental, unofficial, local, and unpublished. No live API tests, browser authentication UI, credential persistence, package publication, or deployment are part of this proposal.

## Approved SDK prerequisite

A fake-response reproduction confirmed that a JSON operation returning malformed HTTP 200 JSON resolves with an internal parse-failure object as successful data. Cam approved a narrow assertion-backed generation adaptation that routes parse failure through the existing SDK error channel; do not identify failures by API payload shape. Malformed HTTP error JSON remains an error with its real response status and raw metadata.

Installed-consumer regressions must preserve actual HTTP status/raw metadata, valid JSON shaped like an error, text and empty responses, approval redirects with incidental non-JSON bodies, and headerless non-2xx responses using the same parser. If this requires broad runtime replacement, reconsider the scope rather than proceeding. Keep the existing runtime and shared generator pins.

## Evidence

- [Pinned Forge SDK transformer](https://github.com/cloudflare/forge/blob/00b8ede867530f8891fe4124b2f5f20ee8d0b05e/packages/cloudflare-forge-transformer-sdk-ts/scripts/generate-from-openapi.ts) and [inventory contract](https://github.com/cloudflare/forge/blob/00b8ede867530f8891fe4124b2f5f20ee8d0b05e/packages/cloudflare-forge-sdk-ts/scripts/generate-sdk-map.ts).
- Existing local generation pattern: `scripts/generate-react-query.mjs`.
- Parse-failure boundary: `src/generated/core/fetcher/getResponseBody.ts`, `Fetcher.ts`, and `src/generated/errors/handleNonStatusCodeError.ts`.
- [Design interview with Oracle](https://ampcode.com/threads/T-01a0f9a8-73e4-7342-8748-9d467a84a746).
