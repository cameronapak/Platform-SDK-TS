import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import test from 'node:test';

import openapi from '../openapi/openapi.json' with { type: 'json' };
import sourceSdkMap from '../src/generated/sdk-map.json' with { type: 'json' };
import cases from './conformance/cases.json' with { type: 'json' };

const execFileAsync = promisify(execFile);
const HTTP_METHODS = new Set(['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace']);

function openApiOperations() {
  const operations = new Map();
  for (const [path, item] of Object.entries(openapi.paths)) {
    for (const [method, operation] of Object.entries(item)) {
      if (HTTP_METHODS.has(method)) operations.set(operation.operationId, { method: method.toUpperCase(), path });
    }
  }
  return operations;
}

test('case inventory covers every OpenAPI operation independently of the SDK map', () => {
  const operations = openApiOperations();
  assert.equal(operations.size, 33);
  assert.deepEqual(new Set(cases.map((item) => item.operationId)), new Set(operations.keys()));
  for (const item of cases) {
    const operation = operations.get(item.operationId);
    assert.equal(item.expect.method, operation.method, `${item.name}: method`);
    const expectedPath = operation.path.replaceAll(/\{([^}]+)\}/g, (_, name) => encodeURIComponent(String(item.parameters[name])));
    assert.equal(item.expect.path, expectedPath, `${item.name}: path`);
  }
});

test('installed npm artifact conforms to shared wire cases', async (t) => {
  const temporary = await mkdtemp(join(tmpdir(), 'platform-sdk-conformance-'));
  t.after(() => rm(temporary, { recursive: true, force: true }));

  const packDirectory = join(temporary, 'pack');
  const consumerDirectory = join(temporary, 'consumer');
  await execFileAsync('mkdir', ['-p', packDirectory, consumerDirectory]);
  // Test entrypoints build first. Do not rebuild dist while sibling tests import it.
  const { stdout } = await execFileAsync('npm', ['pack', '--ignore-scripts', '--pack-destination', packDirectory], { cwd: resolve('.') });
  const tarballName = stdout.trim().split('\n').at(-1);
  const tarball = resolve(packDirectory, tarballName);
  await execFileAsync('npm', ['init', '-y'], { cwd: consumerDirectory });
  await execFileAsync('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', tarball], { cwd: consumerDirectory });

  const packageRoot = join(consumerDirectory, 'node_modules', '@cameronapak', 'platform-sdk');
  const sdk = await import(pathToFileURL(join(packageRoot, 'dist', 'index.js')));
  const installedMap = JSON.parse(await readFile(join(packageRoot, 'dist', 'generated', 'sdk-map.json'), 'utf8'));
  assert.deepEqual(installedMap, sourceSdkMap);

  for (const item of cases) {
    await t.test(item.name, async () => {
      const calls = [];
      const server = createServer(async (request, response) => {
        const chunks = [];
        for await (const chunk of request) chunks.push(chunk);
        calls.push({ method: request.method, url: request.url, headers: request.headers, body: Buffer.concat(chunks).toString() });
        response.writeHead(item.response.status, item.response.headers ?? {});
        if ('body' in item.response) response.end(JSON.stringify(item.response.body));
        else response.end(item.response.text ?? undefined);
      });
      await new Promise((resolveListen) => server.listen(0, '127.0.0.1', resolveListen));
      t.after(() => server.close());
      const { port } = server.address();
      const client = new sdk.YouVersionPlatformClient({
        yvpAppKey: item.client.appKey,
        token: item.client.token,
        headers: item.client.headers,
        maxRetries: item.client.maxRetries ?? 0,
        baseUrl: `http://127.0.0.1:${port}`,
      });
      const mapEntry = installedMap[item.operationId];
      let owner = client;
      for (const accessor of mapEntry.accessor) owner = owner[accessor];
      const requestOptions = item.requestHeaders ? { headers: item.requestHeaders } : undefined;
      const invocation = owner[mapEntry.method](item.parameters, requestOptions);
      const expectsRedirect = item.response.status === 303;
      const expectsError = item.response.status >= 300 && !expectsRedirect;
      let returned;
      if (expectsError) {
        await assert.rejects(invocation, (error) => {
          assert.equal(error.statusCode, item.response.status);
          const expectedBody = 'body' in item.response ? item.response.body : item.response.text;
          assert.deepEqual(error.body, expectedBody);
          return true;
        });
      } else if (expectsRedirect) {
        const result = await invocation.withRawResponse();
        assert.equal(result.rawResponse.status, 303);
        assert.equal(result.rawResponse.headers.get('location'), item.response.headers.location);
        returned = result.data ?? null;
      } else {
        returned = (await invocation) ?? null;
      }
      if (!expectsError) {
        const expectedReturn = 'body' in item.response ? item.response.body : ('text' in item.response ? item.response.text : null);
        assert.deepEqual(returned, expectedReturn);
      }

      assert.equal(calls.length, 1, `${item.name}: redirect callback or retry was unexpectedly requested`);
      const call = calls[0];
      const url = new URL(call.url, `http://127.0.0.1:${port}`);
      assert.equal(call.method, item.expect.method);
      assert.equal(url.pathname, item.expect.path);
      const actualQuery = {};
      for (const key of new Set(url.searchParams.keys())) actualQuery[key] = url.searchParams.getAll(key);
      assert.deepEqual(actualQuery, item.expect.query);
      for (const [name, expected] of Object.entries(item.expect.headers)) {
        assert.equal(call.headers[name.toLowerCase()] ?? null, expected, `${item.name}: header ${name}`);
      }
      if ('body' in item.expect) assert.deepEqual(JSON.parse(call.body), item.expect.body);
      else assert.equal(call.body, '');
      assert.notEqual(url.pathname, '/callback');
    });
  }
});
