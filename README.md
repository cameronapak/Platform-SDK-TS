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
import { YouVersionPlatformClient } from './dist/index.js';

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

## Regenerate

Generation requires Node.js 22 or newer, pnpm, Git, and a running Docker daemon.

```sh
pnpm generate
pnpm check
```

The generator is pinned to a tested Forge commit and builds the standalone transformer from source. It applies two compatibility adaptations after Forge runs:

- Replace Forge's current Cloudflare-specific public names and response-envelope runtime with YouVersion equivalents.
- Keep OAuth optional for app-key-only endpoints while using bearer authentication when `token` is supplied.

Generated source is committed under `src/generated`. The operation-to-method mapping is available at `src/generated/sdk-map.json`.
