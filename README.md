# YouVersion Platform TypeScript SDK

Typed, ESM-first client for the YouVersion Platform API. The SDK is generated from the bundled OpenAPI specification with [Cloudflare Forge](https://github.com/cloudflare/forge).

## Requirements

- Node.js 22 or newer

## Install

```sh
pnpm add @youversion/platform-sdk
```

## Use

App-key endpoints only require your YouVersion Platform app key:

```ts
import { YouVersionPlatformClient } from '@youversion/platform-sdk';

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

The generator is pinned to a tested Forge commit because Forge's standalone transformer is not yet published to npm. It applies two compatibility adaptations after Forge runs:

- Replace Forge's current Cloudflare-specific public names and response-envelope runtime with YouVersion equivalents.
- Keep OAuth optional for app-key-only endpoints while using bearer authentication when `token` is supplied.

Generated source is committed under `src/generated`. The operation-to-method mapping is exported as `@youversion/platform-sdk/sdk-map.json`.
