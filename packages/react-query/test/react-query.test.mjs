import assert from 'node:assert/strict';
import test from 'node:test';

import { YouVersionPlatformClient } from '@cameronapak/platform-sdk';
import {
  MutationObserver,
  QueryClient,
  QueryClientProvider,
  dehydrate,
  hydrate,
} from '@tanstack/react-query';
import React, { act } from 'react';
import TestRenderer from 'react-test-renderer';

import { createPlatformQueries, reactQueryOperationManifest } from '../dist/index.js';

globalThis.IS_REACT_ACT_ENVIRONMENT = true;

function createClient(fetch, options = {}) {
  return new YouVersionPlatformClient({
    yvpAppKey: 'test-app-key',
    baseUrl: 'https://example.test',
    maxRetries: 2,
    fetch,
    ...options,
  });
}

function createQueryClient() {
  return new QueryClient({ defaultOptions: { queries: { retry: false } } });
}

test('classifies every SDK operation and excludes redirect actions', () => {
  const operations = Object.values(reactQueryOperationManifest);
  assert.equal(operations.filter(({ kind }) => kind === 'query').length, 28);
  assert.equal(operations.filter(({ kind }) => kind === 'mutation').length, 3);
  assert.equal(operations.filter(({ kind }) => kind === 'excluded').length, 2);
  assert.equal(reactQueryOperationManifest['v1.apps.permissions.collection_get'].requiresOAuth, true);

  const platform = createPlatformQueries({
    client: createClient(async () => assert.fail('fetch should not run')),
    cacheScope: 'user:1|locale:en|environment:test',
  });
  assert.equal('approvalGet' in platform.dataExchange, false);
  assert.equal('approvalPost' in platform.dataExchange, false);
  assert.throws(
    () => createPlatformQueries({ client: createClient(async () => assert.fail()), cacheScope: '  ' }),
    /cacheScope must be a non-empty/,
  );
});

test('uses parameter-sensitive snapshots and scope-isolated resource filters', async () => {
  const calls = [];
  const client = createClient(async (url, init) => {
    calls.push({ url: String(url), init });
    return Response.json({ data: [], total_size: 0 });
  });
  const english = createPlatformQueries({ client, cacheScope: 'user:1|locale:en' });
  const spanish = createPlatformQueries({ client, cacheScope: 'user:1|locale:es' });
  const request = { 'language_ranges[]': ['en', 'es'], page_size: 17, page_token: 'page-a' };
  const options = english.bibles.collectionGet.queryOptions(request);

  request['language_ranges[]'][0] = 'fr';
  request.page_token = 'page-b';

  const queryClient = createQueryClient();
  await queryClient.fetchQuery(options);
  const url = new URL(calls[0].url);
  assert.deepEqual(url.searchParams.getAll('language_ranges[]'), ['en', 'es']);
  assert.equal(url.searchParams.get('page_size'), '17');
  assert.equal(url.searchParams.get('page_token'), 'page-a');
  assert.equal(new Headers(calls[0].init.headers).get('x-yvp-app-key'), 'test-app-key');

  const englishKey = options.queryKey;
  const spanishKey = spanish.bibles.collectionGet.queryKey({
    'language_ranges[]': ['en', 'es'],
    page_size: 17,
    page_token: 'page-a',
  });
  const fontKey = english.fonts.collectionGet.queryKey();
  queryClient.setQueryData(spanishKey, { data: [{ id: 1 }] });
  queryClient.setQueryData(fontKey, { data: [{ id: 2 }] });
  queryClient.removeQueries(english.bibles.queryFilters());

  assert.equal(queryClient.getQueryData(englishKey), undefined);
  assert.deepEqual(queryClient.getQueryData(spanishKey), { data: [{ id: 1 }] });
  assert.deepEqual(queryClient.getQueryData(fontKey), { data: [{ id: 2 }] });
  assert.equal(JSON.stringify(englishKey).includes('test-app-key'), false);

  const locked = english.bibles.collectionGet.queryOptions(
    { 'language_ranges[]': ['en'] },
    {
      queryKey: ['wrong'],
      queryFn: async () => ({ wrong: true }),
      queryHash: 'wrong',
      queryKeyHashFn: () => 'wrong',
    },
  );
  assert.equal(locked.queryKey[0], '@cameronapak/platform-sdk-react-query');
  assert.equal(locked.queryHash, undefined);
  assert.equal(locked.queryKeyHashFn, undefined);
});

test('converts modeled empty query success to null', async () => {
  const platform = createPlatformQueries({
    client: createClient(async () => new Response(null, { status: 204 })),
    cacheScope: 'public|environment:test',
  });
  const data = await createQueryClient().fetchQuery(
    platform.bibles.collectionGet.queryOptions({ 'language_ranges[]': ['en'] }),
  );
  assert.equal(data, null);
});

test('forwards cancellation and lets TanStack own retries', async () => {
  let cancellationSignal;
  const cancellationClient = createClient(
    async (_url, init) =>
      await new Promise((resolve) => {
        cancellationSignal = init.signal;
        init.signal.addEventListener('abort', () => resolve(Response.json({ cancelled: true })), {
          once: true,
        });
      }),
  );
  const cancellationPlatform = createPlatformQueries({
    client: cancellationClient,
    cacheScope: 'public|environment:test',
  });
  const queryClient = createQueryClient();
  const pending = queryClient.fetchQuery(
    cancellationPlatform.fonts.resourceGet.queryOptions({ font_id: 42 }),
  );
  await new Promise((resolve) => setTimeout(resolve, 0));
  await queryClient.cancelQueries(cancellationPlatform.fonts.resourceGet.queryFilters());
  await assert.rejects(pending);
  assert.equal(cancellationSignal.aborted, true);

  let attempts = 0;
  const retryPlatform = createPlatformQueries({
    client: createClient(async () => {
      attempts += 1;
      return Response.json({ error: 'unavailable' }, { status: 503 });
    }),
    cacheScope: 'public|environment:test',
  });
  await assert.rejects(
    createQueryClient().fetchQuery(
      retryPlatform.fonts.resourceGet.queryOptions(
        { font_id: 42 },
        { retry: 2, retryDelay: 0 },
      ),
    ),
  );
  assert.equal(attempts, 3, 'SDK retries must not multiply TanStack attempts');
});

test('executes typed mutations without automatic invalidation', async () => {
  let call;
  const responseBody = {
    data: { bible_id: 3034, passage_id: 'JHN.3.16', color: '44aa44' },
  };
  const platform = createPlatformQueries({
    client: createClient(
      async (url, init) => {
        call = { url: String(url), init };
        return Response.json(responseBody, { status: 201 });
      },
      { token: 'user-access-token' },
    ),
    cacheScope: 'user:1|locale:en|environment:test',
  });
  const queryClient = createQueryClient();
  const observer = new MutationObserver(queryClient, platform.highlights.collectionPost.mutationOptions());
  const result = await observer.mutate({
    request_id: '2f8c5d2e-6f6c-4b4f-9b36-8c1b5a17d3f9',
    highlight: { bible_id: 3034, passage_id: 'JHN.3.16', color: '44aa44' },
  });

  assert.deepEqual(result, responseBody);
  assert.equal(new URL(call.url).pathname, '/v1/highlights');
  assert.equal(call.init.method, 'POST');
  assert.equal(new Headers(call.init.headers).get('authorization'), 'Bearer user-access-token');
  assert.deepEqual(JSON.parse(call.init.body), {
    request_id: '2f8c5d2e-6f6c-4b4f-9b36-8c1b5a17d3f9',
    highlight: { bible_id: 3034, passage_id: 'JHN.3.16', color: '44aa44' },
  });
});

test('reuses options for SSR hydration and exposes a working hook', async () => {
  let fetches = 0;
  const responseBody = { data: [{ id: 1, name: 'Platform Sans' }] };
  const platform = createPlatformQueries({
    client: createClient(async () => {
      fetches += 1;
      return Response.json(responseBody);
    }),
    cacheScope: 'public|locale:en|environment:test',
  });
  const options = platform.fonts.collectionGet.queryOptions({ staleTime: 60_000 });
  const server = createQueryClient();
  await server.prefetchQuery(options);
  const browser = createQueryClient();
  hydrate(browser, dehydrate(server));
  assert.deepEqual(browser.getQueryData(options.queryKey), responseBody);

  let hookData;
  function Consumer() {
    hookData = platform.fonts.collectionGet.useQuery({ staleTime: 60_000 }).data;
    return null;
  }

  let renderer;
  await act(async () => {
    renderer = TestRenderer.create(
      React.createElement(
        QueryClientProvider,
        { client: browser },
        React.createElement(Consumer),
      ),
    );
  });
  assert.deepEqual(hookData, responseBody);
  assert.equal(fetches, 1);
  await act(async () => renderer.unmount());
});
