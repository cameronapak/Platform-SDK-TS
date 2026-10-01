# Unofficial YouVersion Platform SDKs

> [!IMPORTANT]
> This is Cameron Pak's personal project exploring Cloudflare Forge. Cameron contracts for YouVersion, but this project is not a YouVersion project or an official or supported YouVersion SDK. It is not published as a package, and its API may change without notice.

This repository explores language SDK generation for the YouVersion Platform API from the bundled OpenAPI specification using [Cloudflare Forge](https://github.com/cloudflare/forge). It contains TypeScript, Python, Go, and Rust Platform SDKs and a TypeScript React Query SDK add-on.

## Requirements

- TypeScript: Node.js 22 or newer and pnpm.
- Python: Python 3.11 or newer and uv. See the [Python setup and usage](packages/python/README.md).
- Go: Go 1.24 or newer. See the [Go setup and usage](packages/go/README.md).
- Rust: Rust 1.99.0 for the verified toolchain. See the [Rust setup and usage](packages/rust/README.md).

## Run locally

```sh
git clone https://github.com/cameronapak/Unofficial-YouVersion-Platform-SDKs.git
cd Unofficial-YouVersion-Platform-SDKs
pnpm install
pnpm build
```

## Use

App-key endpoints only require your YouVersion Platform app key:

```ts
import { YouVersionPlatformClient } from '@cameronapak/platform-sdk';

const youVersion = new YouVersionPlatformClient({
  yvpAppKey: process.env.YOUVERSION_APP_KEY!,
});

const bibles = await youVersion.bibles.collectionGet({
  'language_ranges[]': ['en'],
  page_size: 25,
});
```

Pass an OAuth access token when an operation acts on a user's data:

```ts
const youVersion = new YouVersionPlatformClient({
  yvpAppKey: process.env.YOUVERSION_APP_KEY!,
  token: process.env.YOUVERSION_ACCESS_TOKEN!,
});

const highlights = await youVersion.highlights.v1HighlightsCollectionGet({
  bible_id: 3034,
  passage_id: 'JHN.3.16',
});
```

Data-exchange approval endpoints return redirects without following them. Read the callback URL from the raw response:

```ts
const { rawResponse } = await youVersion.dataExchange
  .approvalPost({ token: exchangeToken })
  .withRawResponse();

const callbackUrl = rawResponse.headers.get('location');
```

The client also accepts `baseUrl`, `timeoutInSeconds`, `maxRetries`, custom headers, and a custom `fetch` implementation.

## Use with TanStack React Query

The sibling `@cameronapak/platform-sdk-react-query` package generates query keys, reusable options, and thin hooks from the same operation inventory as the SDK.

```ts
import { createPlatformQueries } from '@cameronapak/platform-sdk-react-query';

const platform = createPlatformQueries({
  client: youVersion,
  // Non-secret identity and representation context. Never use credentials here.
  cacheScope: `user:${userId}|locale:en|environment:production`,
});

const query = platform.bibles.collectionGet.useQuery({
  'language_ranges[]': ['en'],
  page_size: 25,
});
```

Use the same options factory for server prefetching and the hook. Resource filters support broad, scope-safe invalidation:

```ts
await queryClient.prefetchQuery(
  platform.bibles.collectionGet.queryOptions({ 'language_ranges[]': ['en'] }),
);

await queryClient.invalidateQueries(platform.highlights.queryFilters());
```

See [`packages/react-query/README.md`](packages/react-query/README.md) for retry, empty-response, SSR, and mutation-state boundaries.

## Use Python

The unpublished `cameronapak-platform-sdk` distribution exports `PlatformClient` and `AsyncPlatformClient` from `cameronapak_platform_sdk`. Build a local wheel:

```sh
uv build packages/python --out-dir packages/python/dist
```

See the [Python README](packages/python/README.md) for installation, sync and async clients, transport cleanup, and manual pagination.

## Use Go

The Go module lives under `packages/go` and exports `client.NewPlatformClient`, typed request and response models, and request options. Use a local `go.mod` replacement to consume it without a versioned release. See the [Go README](packages/go/README.md) for setup, context cancellation, transport configuration, approval redirects, and manual pagination.

## Use Rust

The unpublished crate lives under `packages/rust` and exports `PlatformClient`, `ApiClientBuilder`, typed models, and request options. Use a Cargo path dependency to consume it locally. See the [Rust README](packages/rust/README.md) for Tokio, HTTPS and transport configuration, approval redirects, cancellation, and manual pagination.

## Regenerate and verify

Generation requires Node.js 22 or newer, pnpm, Git, and a running Docker daemon.

```sh
pnpm generate
pnpm check
pnpm generate:python
pnpm check:python-generated
pnpm check:python
pnpm generate:go
pnpm check:go-generated
pnpm check:go
pnpm generate:rust
pnpm check:rust-generated
pnpm check:rust
```

Go generation also requires Go; Rust generation requires Rust and rustfmt. In Amp orbs, `.agents/setup` installs Go, Rust, and locked dependencies. Start Docker for generation with `amp orb service start sdk-docker --command 'sudo dockerd --storage-driver=vfs --group user'`; stop it when finished with `amp orb service stop sdk-docker`.

The generator is pinned to a tested Forge commit and builds the standalone transformer from source. It applies compatibility adaptations after Forge runs:

- Replace Forge's current Cloudflare-specific public names and response-envelope runtime with YouVersion equivalents.
- Keep OAuth optional for app-key-only endpoints while using bearer authentication when `token` is supplied.
- Generate the React Query add-on from the adapted SDK inventory and OpenAPI operation metadata.

TypeScript generated source is tracked under `src/generated`, with its operation-to-method mapping at `src/generated/sdk-map.json`. Python generated source and its mapping are tracked under `packages/python/src/cameronapak_platform_sdk`; Go's are under `packages/go`; Rust's are under `packages/rust`. Root `fern/` pins Python, Go, and Rust generation; shared preparation uses a temporary copy of the authoritative OpenAPI document.

All four language SDKs consume the [shared wire conformance cases](test/conformance/README.md), covering every OpenAPI operation with local HTTP requests. Python runs the cases on both client surfaces; Go checks a clean consumer module under the race detector; Rust checks an extracted crate artifact from a separate consumer. Language validation runs independently in CI; nothing publishes the packages or calls the live YouVersion Platform API.
