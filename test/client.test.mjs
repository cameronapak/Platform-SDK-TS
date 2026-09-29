import assert from 'node:assert/strict';
import test from 'node:test';

import { YouVersionPlatformClient } from '../dist/index.js';
import openapi from '../openapi/openapi.json' with { type: 'json' };
import sdkMap from '../dist/generated/sdk-map.json' with { type: 'json' };

const HTTP_METHODS = new Set(['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace']);

test('every OpenAPI operation matches a generated client method', () => {
  const client = new YouVersionPlatformClient({ yvpAppKey: 'test-app-key' });
  const operations = new Map();

  for (const [path, pathItem] of Object.entries(openapi.paths)) {
    for (const [method, operation] of Object.entries(pathItem)) {
      if (HTTP_METHODS.has(method)) operations.set(operation.operationId, { method: method.toUpperCase(), path });
    }
  }

  assert.deepEqual(new Set(Object.keys(sdkMap)), new Set(operations.keys()));
  for (const [operationId, entry] of Object.entries(sdkMap)) {
    assert.equal(entry.httpMethod, operations.get(operationId).method, `${operationId} has the wrong HTTP method`);
    assert.equal(entry.path, operations.get(operationId).path, `${operationId} has the wrong path`);
    let owner = client;
    for (const accessor of entry.accessor) owner = owner[accessor];
    assert.equal(typeof owner[entry.method], 'function', `${operationId} is not callable`);
  }
});

test('app-key requests do not require an OAuth token', async () => {
  const calls = [];
  const responseBody = { data: [{ id: 3034, abbreviation: 'BSB' }], total_size: 1 };
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'test-app-key',
    baseUrl: 'https://example.test',
    maxRetries: 0,
    fetch: async (url, init) => {
      calls.push({ url: String(url), init });
      return Response.json(responseBody);
    },
  });

  const response = await client.bibles.collectionGet({
    'language_ranges[]': ['en', 'es'],
    page_size: 25,
  });

  assert.deepEqual(response, responseBody);
  assert.equal(calls.length, 1);
  const request = calls[0];
  const url = new URL(request.url);
  assert.equal(url.pathname, '/v1/bibles');
  assert.deepEqual(url.searchParams.getAll('language_ranges[]'), ['en', 'es']);
  assert.equal(url.searchParams.get('page_size'), '25');
  const headers = new Headers(request.init.headers);
  assert.equal(headers.get('x-yvp-app-key'), 'test-app-key');
  assert.equal(headers.has('authorization'), false);
});

test('font stylesheet sends the app key in the query string', async () => {
  let call;
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'query-app-key',
    baseUrl: 'https://example.test',
    maxRetries: 0,
    fetch: async (url, init) => {
      call = { url: String(url), init };
      return new Response('@font-face {}', { headers: { 'content-type': 'text/css' } });
    },
  });

  const stylesheet = await client.fonts.v1FontsStylesheetGet({ font_id: 42 });

  assert.equal(stylesheet, '@font-face {}');
  const url = new URL(call.url);
  assert.equal(url.pathname, '/v1/fonts/42/stylesheet');
  assert.equal(url.searchParams.get('app_key'), 'query-app-key');
});

test('font stylesheet redacts the query app key from debug logs', async () => {
  const debugEntries = [];
  const logger = {
    debug: (_message, metadata) => debugEntries.push(metadata),
    info: () => {},
    warn: () => {},
    error: () => {},
  };
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'secret-app-key',
    baseUrl: 'https://example.test',
    maxRetries: 0,
    logging: { level: 'debug', logger, silent: false },
    fetch: async () => new Response('@font-face {}'),
  });

  await client.fonts.v1FontsStylesheetGet({ font_id: 42 });

  assert.ok(debugEntries.length > 0);
  assert.equal(JSON.stringify(debugEntries).includes('secret-app-key'), false);
  assert.equal(debugEntries[0].url, 'https://example.test/v1/fonts/42/stylesheet?app_key=[REDACTED]');
});

test('data exchange preserves manual redirects and conditionally sends OAuth', async () => {
  const calls = [];
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'test-app-key',
    token: 'user-access-token',
    baseUrl: 'https://example.test',
    maxRetries: 0,
    fetch: async (url, init) => {
      calls.push({ url: String(url), init });
      return new Response(null, {
        status: 303,
        headers: { location: 'https://callback.test/result?data_exchange_status=granted' },
      });
    },
  });

  const tokenless = await client.dataExchange.approvalPost().withRawResponse();
  assert.equal(tokenless.rawResponse.status, 303);
  assert.equal(
    tokenless.rawResponse.headers.get('location'),
    'https://callback.test/result?data_exchange_status=granted',
  );
  assert.equal(calls[0].init.redirect, 'manual');
  assert.equal(new Headers(calls[0].init.headers).get('authorization'), 'Bearer user-access-token');

  await client.dataExchange.approvalPost({ token: 'exchange-token' });
  assert.equal(calls[1].init.redirect, 'manual');
  assert.equal(new Headers(calls[1].init.headers).has('authorization'), false);

  await client.dataExchange.approvalGet({ token: 'exchange-token' });
  assert.equal(calls[2].init.redirect, 'manual');
  assert.equal(new Headers(calls[2].init.headers).has('authorization'), false);
  assert.equal(calls.length, 3, 'the callback URL must not be fetched');
});

test('caller Accept-Language headers override the generated English default', async () => {
  const languages = [];
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'test-app-key',
    baseUrl: 'https://example.test',
    headers: { 'Accept-Language': 'es' },
    maxRetries: 0,
    fetch: async (_url, init) => {
      languages.push(new Headers(init.headers).get('accept-language'));
      return Response.json({ data: [] });
    },
  });

  await client.languages.v1LanguagesCollectionGet();
  await client.languages.v1LanguagesCollectionGet({}, { headers: { 'Accept-Language': 'fr' } });

  assert.deepEqual(languages, ['es', 'fr']);
});

test('OAuth token and JSON body are sent for authenticated data exchange', async () => {
  let call;
  const responseBody = { token: 'exchange-token', token_type: 'data_exchange', expires_in: 300 };
  const client = new YouVersionPlatformClient({
    yvpAppKey: 'test-app-key',
    token: 'user-access-token',
    baseUrl: 'https://example.test',
    maxRetries: 0,
    fetch: async (url, init) => {
      call = { url: String(url), init };
      return Response.json(responseBody, { status: 201 });
    },
  });

  const response = await client.dataExchange.tokenPost({
    'x-yvp-app-id': '550e8400-e29b-41d4-a716-446655440000',
    requested_permissions: ['highlights'],
  });

  assert.deepEqual(response, responseBody);
  assert.equal(call.init.method, 'POST');
  const url = new URL(call.url);
  assert.equal(url.pathname, '/data-exchange/token');
  assert.equal(url.searchParams.get('x-yvp-app-id'), '550e8400-e29b-41d4-a716-446655440000');
  const headers = new Headers(call.init.headers);
  assert.equal(headers.get('authorization'), 'Bearer user-access-token');
  assert.equal(headers.get('x-yvp-app-key'), 'test-app-key');
  assert.deepEqual(JSON.parse(call.init.body), { requested_permissions: ['highlights'] });
});
