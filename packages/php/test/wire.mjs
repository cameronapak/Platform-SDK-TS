import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { createServer as createHttpsServer } from 'node:https';
import { spawn, execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { createInterface } from 'node:readline';
import { join } from 'node:path';
import spec from '../../../openapi/openapi.json' with { type: 'json' };
import cases from '../../../test/conformance/cases.json' with { type: 'json' };

function resolveSchema(schema) {
  return schema?.$ref ? resolveSchema(schema.$ref.split('/').slice(1).reduce((value, part) => value[part], spec)) : schema;
}

// Expectations come from OpenAPI and fixture values, not generated model fields.
function assertFields(actual, expected, schema) {
  schema = resolveSchema(schema);
  if (expected === null || typeof expected !== 'object') return assert.deepEqual(actual, expected);
  if (Array.isArray(expected)) {
    assert.equal(actual.length, expected.length);
    expected.forEach((value, i) => assertFields(actual[i], value, schema?.items));
    return;
  }
  for (const [key, value] of Object.entries(expected)) {
    const field = schema?.properties?.[key] ?? (typeof schema?.additionalProperties === 'object' ? schema.additionalProperties : undefined);
    if (field) assertFields(actual[key] ?? null, value, field);
  }
}

export async function checkWire(consumer, probe = false) {
  const operations = new Map();
  for (const [path, item] of Object.entries(spec.paths)) {
    for (const [verb, operation] of Object.entries(item)) {
      if (operation.operationId) operations.set(operation.operationId, { ...operation, path, method: verb.toUpperCase() });
    }
  }
  assert.equal(operations.size, 33);
  assert.deepEqual(new Set(cases.map((item) => item.operationId)), new Set(operations.keys()));
  const php = spawn('php', [join(consumer, 'run.php')], { stdio: ['pipe', 'pipe', 'inherit'] });
  const exited = new Promise((resolve) => php.once('close', resolve));
  const lines = createInterface({ input: php.stdout })[Symbol.asyncIterator]();
  let passed = 0;
  const failures = [];
  async function exercise(item, additions = {}) {
    const calls = [];
    const handler = async (request, response) => {
      const chunks = [];
      for await (const chunk of request) chunks.push(chunk);
      calls.push({ method: request.method, url: request.url, headers: request.headers, body: Buffer.concat(chunks).toString() });
      if (request.url.startsWith('/callback')) return response.end('callback must not be requested');
      if (additions.delay) await new Promise((resolve) => setTimeout(resolve, additions.delay));
      response.writeHead(item.response.status, item.response.headers ?? {});
      response.end('body' in item.response ? JSON.stringify(item.response.body) : item.response.text);
    };
    const server = additions.tls ? createHttpsServer({ key: readFileSync(join(consumer, 'key.pem')), cert: readFileSync(join(consumer, 'cert.pem')) }, handler) : createServer(handler);
    await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
    const baseUrl = `${additions.tls ? 'https' : 'http'}://127.0.0.1:${server.address().port}`;
    const redirect = item.response.status === 303;
    if (redirect) item.response.headers.location = `${baseUrl}/callback?result=granted`;
    try {
      php.stdin.write(JSON.stringify({ ...item, baseUrl, options: { headers: item.requestHeaders ?? {} }, raw: redirect, ...additions }) + '\n');
      const line = await lines.next();
      assert.equal(line.done, false, 'PHP consumer exited before returning a result');
      const returned = JSON.parse(line.value);
      if (additions.timeoutError) {
        assert.equal(returned.error, 'Cameronapak\\PlatformSdk\\Exceptions\\CameronapakException', JSON.stringify(returned));
        assert.equal(returned.message, 'HTTP transport failed');
      } else if (item.response.status >= 400) {
        assert.equal(returned.status, item.response.status, JSON.stringify(returned));
        assert.deepEqual(returned.body, 'body' in item.response ? JSON.stringify(item.response.body) : item.response.text);
        for (const [name, value] of Object.entries(item.response.headers ?? {})) {
          assert.equal(returned.headers?.[name]?.[0], value);
        }
      } else {
        assert.equal(returned.error, undefined, JSON.stringify(returned));
        if (redirect || additions.raw) {
          assert.equal(returned.result.status, item.response.status);
          if (redirect) assert.equal(returned.result.headers.location[0], item.response.headers.location);
          assert.equal(returned.result.text, item.response.text ?? '');
        } else if ('body' in item.response) {
          const schema = operations.get(item.operationId).responses[String(item.response.status)].content['application/json'].schema;
          assertFields(returned.result, item.response.body, schema);
        } else {
          assert.deepEqual(returned.result, item.response.text ?? null);
        }
      }
      assert.equal(calls.length, 1, 'Unexpected retry or redirect callback request');
      const call = calls[0];
      const url = new URL(call.url, baseUrl);
      const operation = operations.get(item.operationId);
      assert.equal(item.expect.method, operation.method);
      assert.equal(item.expect.path, operation.path.replaceAll(/\{([^}]+)\}/g, (_, name) => encodeURIComponent(item.parameters[name])));
      assert.equal(call.method, item.expect.method);
      assert.equal(url.pathname, item.expect.path);
      const query = Object.fromEntries([...new Set(url.searchParams.keys())].map((name) => [name, url.searchParams.getAll(name)]));
      assert.deepEqual(query, item.expect.query);
      assert(!/\[(?:\d+)\]/.test(decodeURIComponent(url.search)), 'Indexed PHP query array was sent');
      for (const [name, value] of Object.entries(item.expect.headers)) assert.equal(call.headers[name.toLowerCase()] ?? null, value, `header ${name}`);
      if (item.expect.body) assert.deepEqual(JSON.parse(call.body), item.expect.body);
      else assert.equal(call.body, '');
      passed++;
    } catch (error) {
      failures.push(`${item.name}: ${error.message}`);
    } finally {
      server.closeAllConnections();
      await new Promise((resolve) => server.close(resolve));
    }
  }
  try {
    const selected = probe ? cases.filter((item) => ['bibles.resource_get', 'v1.highlights.collection_get', 'data_exchange.approval_get', 'data_exchange.approval_post'].includes(item.operationId)) : cases;
    for (const item of selected) await exercise(structuredClone(item));
    for (const operationId of ['data_exchange.approval_get', 'data_exchange.approval_post']) {
      for (const timeout of ['unset', 'client', 'request']) {
        const item = structuredClone(cases.find((item) => item.operationId === operationId));
        item.name = `${operationId} body-bearing 303 with ${timeout} timeout`;
        item.response = { status: 303, headers: { location: 'replaced-with-loopback' }, text: 'Redirecting' };
        if (timeout === 'client') item.client.timeout = 1;
        await exercise(item, { raw: true, options: timeout === 'request' ? { timeout: 1 } : {} });
      }
    }
    if (!probe) {
      const search = structuredClone(cases.find((item) => item.operationId === 'v1.search_unified.collection_get'));
      search.name = 'literal search punctuation';
      search.parameters.query = 'John3:16 + grace/faith?';
      search.expect.query.query = [search.parameters.query];
      await exercise(search);
      const empty = structuredClone(cases.find((item) => item.operationId === 'v1.fonts.stylesheet_get'));
      empty.name = 'required empty text';
      empty.response.text = '';
      await exercise(empty);
      const wildcard = structuredClone(cases.find((item) => item.operationId === 'bibles.collection_get'));
      wildcard.name = 'wildcard page size';
      wildcard.parameters.page_size = '*';
      wildcard.expect.query.page_size = ['*'];
      await exercise(wildcard);
      for (const override of [false, true]) {
        const slow = structuredClone(cases.find((item) => item.operationId === 'bibles.resource_get'));
        slow.name = `real timeout with request override ${override}`;
        slow.client.timeout = 0.02;
        await exercise(slow, { delay: 150, timeoutError: !override, options: override ? { timeout: 1 } : {} });
      }
      execFileSync('openssl', ['req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-keyout', join(consumer, 'key.pem'), '-out', join(consumer, 'cert.pem'), '-days', '1', '-subj', '/CN=127.0.0.1', '-addext', 'subjectAltName=IP:127.0.0.1', '-addext', 'basicConstraints=critical,CA:FALSE'], { stdio: 'ignore' });
      const https = structuredClone(cases.find((item) => item.operationId === 'bibles.resource_get'));
      https.name = 'HTTPS with caller transport certificate';
      https.client.transport = { verify: join(consumer, 'cert.pem') };
      await exercise(https, { tls: true });
    }
    console.log(`PHP wire cases: ${passed} passed, ${failures.length} failed`);
    if (failures.length) throw new Error(failures.join('\n'));
  } finally {
    php.stdin.end();
    await exited;
  }
}

if (process.argv[1]?.endsWith('/wire.mjs')) await checkWire(process.argv[2], process.argv.includes('--probe'));
