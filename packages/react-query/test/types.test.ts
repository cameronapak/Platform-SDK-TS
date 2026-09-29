import { YouVersionPlatformClient, type YouVersionPlatform } from '@cameronapak/platform-sdk';
import { QueryClient } from '@tanstack/react-query';

import { createPlatformQueries } from '../dist/index.js';

declare const client: YouVersionPlatformClient;
declare const queryClient: QueryClient;

const platform = createPlatformQueries({ client, cacheScope: 'user:1|locale:en' });
const bibleRequest: YouVersionPlatform.BiblesCollectionGetRequest = {
  'language_ranges[]': ['en', 'es'],
};
const bibleOptions = platform.bibles.collectionGet.queryOptions(bibleRequest);
const cachedBibles = queryClient.getQueryData(bibleOptions.queryKey);
const expectedCachedBibles:
  | YouVersionPlatform.BiblesCollectionGetResponse
  | null
  | undefined = cachedBibles;
const standaloneBibleKey = platform.bibles.collectionGet.queryKey(bibleRequest);
const directlyCachedBibles = queryClient.getQueryData(standaloneBibleKey);
const expectedDirectlyCachedBibles:
  | YouVersionPlatform.BiblesCollectionGetResponse
  | null
  | undefined = directlyCachedBibles;
queryClient.setQueryData(standaloneBibleKey, (previous) => {
  const typedPrevious:
    | YouVersionPlatform.BiblesCollectionGetResponse
    | null
    | undefined = previous;
  return typedPrevious ?? null;
});
queryClient.getQueriesData({ queryKey: standaloneBibleKey, exact: true });

const selected = platform.bibles.collectionGet.useQuery(bibleRequest, {
  select: (response) => response?.total_size ?? 0,
});
const selectedData: number | undefined = selected.data;

const withInitialData = platform.bibles.collectionGet.useQuery(bibleRequest, {
  initialData: { data: [], total_size: 0 },
});
const definedData: YouVersionPlatform.BiblesCollectionGetResponse | null = withInitialData.data;

platform.fonts.collectionGet.queryOptions();
platform.languages.collectionGet.queryOptions();
platform.languages.collectionGet.queryOptions(undefined, { staleTime: 1_000 });

platform.highlights.collectionPost.mutationOptions({
  onMutate: (variables) => ({ requestId: variables.request_id }),
  onError: (_error, _variables, onMutateResult) => {
    const requestId: string | undefined = onMutateResult?.requestId;
    return requestId;
  },
});

// @ts-expect-error language_ranges[] remains required even when a query is disabled.
platform.bibles.collectionGet.queryOptions({}, { enabled: false });

// @ts-expect-error generated cache identity cannot be overridden.
platform.bibles.collectionGet.queryOptions(bibleRequest, { queryKey: ['other'] });

// @ts-expect-error generated query execution cannot be overridden.
platform.bibles.collectionGet.queryOptions(bibleRequest, { queryFn: async () => null });

// @ts-expect-error custom query hashing can defeat generated cache identity.
platform.bibles.collectionGet.queryOptions(bibleRequest, { queryKeyHashFn: () => 'same' });

// @ts-expect-error arbitrary SDK headers are intentionally unavailable.
platform.bibles.collectionGet.queryOptions(bibleRequest, { sdk: { headers: { foo: 'bar' } } });

// @ts-expect-error redirect approval operations are deliberately excluded.
platform.dataExchange.approvalGet;

void expectedCachedBibles;
void expectedDirectlyCachedBibles;
void selectedData;
void definedData;
