# YouVersion Platform TypeScript SDK

> [!IMPORTANT]
> This repository is an unofficial experiment. It is not an official or supported YouVersion SDK, it is not published as a package, and its API may change without notice.

This experiment generates a typed, ESM-first client for the YouVersion Platform API from the bundled OpenAPI specification using [Cloudflare Forge](https://github.com/cloudflare/forge).

## Requirements

- Node.js 22 or newer

## Run locally

```sh
git clone https://github.com/cameronapak/Platform-SDK-TS.git
cd Platform-SDK-TS
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

## Regenerate

Generation requires Node.js 22 or newer, pnpm, Git, and a running Docker daemon.

```sh
pnpm generate
pnpm check
```

The generator is pinned to a tested Forge commit and builds the standalone transformer from source. It applies compatibility adaptations after Forge runs:

- Replace Forge's current Cloudflare-specific public names and response-envelope runtime with YouVersion equivalents.
- Keep OAuth optional for app-key-only endpoints while using bearer authentication when `token` is supplied.
- Generate the React Query add-on from the adapted SDK inventory and OpenAPI operation metadata.

Generated source is committed under `src/generated`. The operation-to-method mapping is available at `src/generated/sdk-map.json`.
