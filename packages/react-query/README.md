# Platform SDK React Query add-on

> [!IMPORTANT]
> This package is part of Cameron Pak's personal Cloudflare Forge experiment. It is not a YouVersion project or an official or supported YouVersion SDK and is not published.

This package generates TanStack React Query bindings for `@cameronapak/platform-sdk`. See the repository README for setup and usage.

## Bind a client

Create one binding for each non-secret cache identity and representation context. Include every client-level detail that can change a response, such as the user or tenant, app identity, upstream environment, and locale. Credential refresh for the same identity does not require a new scope.

```ts
import { YouVersionPlatformClient } from '@cameronapak/platform-sdk';
import { createPlatformQueries } from '@cameronapak/platform-sdk-react-query';

const client = new YouVersionPlatformClient({
  yvpAppKey: process.env.YOUVERSION_APP_KEY!,
  token: process.env.YOUVERSION_ACCESS_TOKEN,
  headers: { 'Accept-Language': 'en' },
});

const platform = createPlatformQueries({
  client,
  cacheScope: `user:${userId}|locale:en|environment:production`,
});
```

The cache scope must be a non-empty string. Never put an app key, access token, or data exchange token in it. Scope separation prevents cache collisions; it is not an authorization boundary. Applications own cache cleanup when identities change.

## Query and prefetch

Every query exposes `queryKey`, `queryOptions`, `queryFilters`, and `useQuery`. Options factories work without React rendering, so server prefetching and browser hooks use identical keys and functions.

```ts
const request = {
  'language_ranges[]': ['en'],
  page_size: 25,
};

await queryClient.prefetchQuery(
  platform.bibles.collectionGet.queryOptions(request, {
    staleTime: 60_000,
  }),
);

const result = platform.bibles.collectionGet.useQuery(request, {
  staleTime: 60_000,
  select: (response) => response?.data ?? [],
});
```

For SSR, create the SDK client, binding, and TanStack `QueryClient` per request. Dehydrate only TanStack state; recreate the binding in the browser with the same cache scope and request data.

Successful empty query responses return `null` because TanStack Query does not permit query functions to resolve to `undefined`. Other SDK results are unchanged.

## Mutations and invalidation

Mutations expose `mutationKey`, `mutationOptions`, `mutationFilters`, and `useMutation`. Generated code does not guess which queries a mutation should invalidate.

```ts
const mutation = platform.highlights.collectionPost.useMutation({
  onSuccess: () =>
    queryClient.invalidateQueries(platform.highlights.queryFilters()),
});
```

TanStack can retain mutation variables and results in memory, Devtools, or persisted state. Applications own persistence policy and cleanup for data exchange tokens and other sensitive mutation data.

## Retry and cancellation ownership

The add-on forwards TanStack Query's `AbortSignal` to query requests and sets SDK retries to zero for queries and mutations. TanStack owns retry and backoff policy by default.

An explicit SDK escape hatch is available when its `Retry-After` handling is required:

```ts
platform.bibles.collectionGet.queryOptions(request, {
  retry: false,
  sdk: { maxRetries: 2, timeoutInSeconds: 30 },
});
```

Disable TanStack retries when enabling SDK retries to avoid multiplying attempts. Arbitrary SDK headers, query parameters, body parameters, credentials, and abort signals are intentionally unavailable through operation options; configure response-affecting values on the bound SDK client and represent them in the cache scope.

Data-exchange approval GET and POST operations are browser redirect actions and have no generated bindings. Pagination remains ordinary page-token queries; the add-on does not generate infinite or Suspense hooks.
