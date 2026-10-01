# PHP Platform SDK specification

Implements [issue #5](https://github.com/cameronapak/Unofficial-YouVersion-Platform-SDKs/issues/5).
This SDK is experimental, unofficial, local, and unpublished.

## Generation boundary

- Use Forge's existing baseline, revision `00b8ede867530f8891fe4124b2f5f20ee8d0b05e`, Fern CLI `5.112.0`, and PHP generator `2.11.1`.
- Generate from a working copy of the bundled YouVersion Platform OpenAPI. Do not change the authoritative contract for generator convenience.
- Preserve Fern's PHP models, client organization, PSR transport, exception hierarchy, and retry timing. Use supported configuration before adapting output.
- Count compatibility replacements and reject unexpected generator output. Compare the complete generated file inventory and contents during regeneration checks.
- If correctness requires replacing the runtime or broadly rewriting endpoints, stop and report the failing cases and alternatives.

## Public package

- Composer package: `cameronapak/platform-sdk`. Namespace: `Cameronapak\PlatformSdk`. Root client: `PlatformClient`.
- Preserve native generated request and response types. Use explicit page tokens, not a handwritten pagination framework.
- Preserve direct JSON and text responses, declared empty responses, and useful HTTP exceptions with status, body, and headers.
- Expose approval status and `Location` through native PSR response objects if the pinned generator lacks a supported raw-response facility. Do not use a mutable last-response property.
- Guzzle is the first verified consumer transport. Keep transport injection and document which transports support SDK timeout overrides.

## Wire contract

- App-key-only endpoints work without OAuth. User operations send the app key and bearer credential together.
- The font stylesheet receives the app key in its query string.
- Approval GET suppresses bearer authentication. Approval POST suppresses bearer authentication when an exchange token is present and uses configured OAuth otherwise. Suppression happens before acquiring credentials and after merging SDK-owned headers.
- Approval responses never follow callbacks. Callers can inspect 303 status and `Location`, including when a timeout is configured or the redirect has a body.
- Array query parameters use repeated wire keys, without PHP numeric indices. Booleans use textual wire values. Path and query values preserve reserved characters safely.
- Locale precedence is case-insensitive: request override, then client override, then the generated default.
- Retry and timeout overrides retain native runtime semantics. Permanent HTTP errors must not become retryable merely because a timeout is configured.
- SDK-produced exception messages and string conversion must not expose credentials. Explicit access to raw response data remains available. Credentials injected by opaque custom transports are outside SDK suppression guarantees.

## Acceptance checks

1. Generate untouched output and inspect the actual constructor, transport, Composer requirements, and response APIs.
2. Install a local archive into a clean Composer consumer using an artifact repository. Verify metadata, autoloading, artifact contents, and that classes resolve from the installed artifact rather than the checkout.
3. First prove app-key access, bearer access, all approval flows, observable redirects with timeout overrides, and permanent errors. Only then build the full operation suite.
4. Execute the 39 shared wire cases covering all 33 OpenAPI operations. Match observed methods and paths against independent fixture expectations, not generated mappings. Verify declared typed response fields.
5. Add focused regressions for literal search punctuation, reserved path/query characters, empty required text, optional empty responses, body-bearing redirects, retry status selection, timeout overrides, and credential-safe failures.
6. Run syntax checks, Composer validation, narrow consumer static analysis, deterministic formatting, artifact checks, and generated-source drift checks. Test the supported PHP floor and newest compatible stable PHP in consumer CI, with one generation job.
7. Pin maintainer and consumer dependencies. Refresh only the SDK archive entry in the temporary consumer lock and assert that other dependency entries do not change. A library lock does not constrain downstream users.

Use loopback mock servers and fake credentials. No automated test calls the live YouVersion Platform API. Publication, framework integrations, shared generator upgrades, and unrelated SDK cleanup are out of scope.

## Sources

- [Forge PHP pin](https://github.com/cloudflare/forge/blob/00b8ede867530f8891fe4124b2f5f20ee8d0b05e/packages/cloudflare-fern-config/fern/generators.yml)
- [Fern 2.11.1 configuration](https://github.com/fern-api/fern/blob/2cf8f5ddb1404a23fdb433d5299cd9f97d1be324/generators/php/codegen/src/custom-config/BasePhpCustomConfigSchema.ts)
- [Fern timeout and retry transport](https://github.com/fern-api/fern/blob/2cf8f5ddb1404a23fdb433d5299cd9f97d1be324/generators/php/base/src/asIs/Client/RetryDecoratingClient.Template.php)
- [Guzzle PSR transport semantics](https://github.com/guzzle/guzzle/blob/7.10.0/src/Client.php)
