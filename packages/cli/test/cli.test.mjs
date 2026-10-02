import assert from 'node:assert/strict';
import { execFile, spawn } from 'node:child_process';
import { createServer } from 'node:http';
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { promisify } from 'node:util';
import test from 'node:test';
import sdkMap from '../../../src/generated/sdk-map.json' with { type: 'json' };
import cases from '../../../test/conformance/cases.json' with { type: 'json' };

const exec = promisify(execFile);
const root = resolve('.');

test('installed Platform CLI', async (t) => {
  const temporary = await mkdtemp(join(tmpdir(), 'platform-cli-'));
  t.after(() => rm(temporary, { recursive: true, force: true }));
  const consumer = join(temporary, 'consumer');
  await mkdir(consumer);
  const pack = async (cwd) => {
    const { stdout } = await exec('pnpm', ['--config.ignore-scripts=true', 'pack', '--json', '--pack-destination', temporary], { cwd });
    const packed = JSON.parse(stdout);
    return packed.filename ?? packed[0].filename;
  };
  const sdk = await pack(root);
  const cli = await pack(join(root, 'packages/cli'));
  await exec('npm', ['init', '-y'], { cwd: consumer });
  await exec('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', sdk, cli], { cwd: consumer });
  const bin = join(consumer, 'node_modules/.bin/yvp');
  const run = async (args, env = {}) => {
    const clean = { PATH: process.env.PATH, HOME: temporary, ...env };
    try {
      return { ...(await exec(bin, args, { cwd: consumer, env: clean, timeout: 5000 })), code: 0 };
    } catch (error) {
      return { stdout: error.stdout, stderr: error.stderr, code: error.code };
    }
  };
  const fixture = async (reply) => {
    const calls = [];
    const server = createServer(async (request, response) => {
      const chunks = [];
      for await (const chunk of request) chunks.push(chunk);
      calls.push({ method: request.method, url: request.url, headers: request.headers, body: Buffer.concat(chunks).toString() });
      if (typeof reply === 'function') return reply(request, response);
      response.writeHead(reply.status, reply.headers ?? {});
      response.end('body' in reply ? JSON.stringify(reply.body) : reply.text);
    });
    await new Promise((done) => server.listen(0, '127.0.0.1', done));
    t.after(() => { server.closeAllConnections(); server.close(); });
    return { calls, url: `http://127.0.0.1:${server.address().port}` };
  };

  await t.test('discovers every operation and displays help without credentials', async () => {
    const result = await run(['catalog']);
    assert.equal(result.code, 0, result.stderr);
    const catalog = JSON.parse(result.stdout);
    assert.deepEqual(catalog.map((entry) => entry.operationId).sort(), Object.keys(sdkMap).sort());
    assert.equal(catalog.filter((entry) => entry.kind === 'write').length, 4);
    for (const entry of catalog) {
      const help = await run([...entry.command, '--help']);
      assert.equal(help.code, 0, help.stderr);
      assert.ok(help.stdout.includes(entry.operationId));
      assert.ok(help.stdout.includes(sdkMap[entry.operationId].path));
    }
    assert.equal((await run(['--version'])).stdout.trim(), '0.1.0');
    assert.match((await run(['--help'])).stdout, /unofficial/i);
  });

  await t.test('help explains inputs, consent, credentials, output, and exit semantics', async () => {
    const help = await run(['highlights', 'collection-post', '--help']);
    for (const term of ['--body-file', '--body-stdin', 'request_id', 'highlight', 'color', '--yes',
      '--show-sensitive', 'YOUVERSION_ACCESS_TOKEN', '--timeout-seconds', '--output', '130', '143']) {
      assert.ok(help.stdout.includes(term), `Missing help: ${term}`);
    }
    const query = await run(['bibles', 'collection-get', '--help']);
    assert.match(query.stdout, /--language-ranges.*required.*repeat/i);
    assert.match(query.stdout, /--page-size/);
  });

  await t.test('preserves every shared wire case through the installed binary', async (t) => {
    const catalog = JSON.parse((await run(['catalog'])).stdout);
    for (const entry of cases) await t.test(entry.name, async () => {
      const { calls, url } = await fixture(entry.response);
      const args = [...catalog.find((operation) => operation.operationId === entry.operationId).command,
        '--base-url', url, '--yes', '--show-sensitive'];
      const env = { YOUVERSION_APP_KEY: entry.client.appKey, YOUVERSION_ACCESS_TOKEN: entry.client.token };
      for (const [name, value] of Object.entries(entry.parameters)) {
        if (name in (entry.expect.body ?? {})) continue;
        if (name === 'token') { env.YOUVERSION_DATA_EXCHANGE_TOKEN = value; continue; }
        if (name === 'x-yvp-app-key') { env.YOUVERSION_DATA_EXCHANGE_APP_KEY = value; continue; }
        const flag = name.replace(/\[\]$/, '').replaceAll('_', '-');
        for (const item of Array.isArray(value) ? value : [value]) args.push(`--${flag}`, String(item));
      }
      if (entry.operationId === 'data_exchange.approval_post') {
        args.push('--auth-mode', entry.parameters.token ? 'exchange-token' : 'oauth');
      }
      if (entry.expect.body) {
        const file = join(consumer, 'fixture-body.json');
        await writeFile(file, JSON.stringify(entry.expect.body));
        args.push('--body-file', file);
      }
      const locale = Object.entries({ ...entry.client.headers, ...entry.requestHeaders })
        .filter(([name]) => name.toLowerCase() === 'accept-language').at(-1)?.[1];
      if (locale) args.push('--accept-language', locale);
      const result = await run(args, env);
      assert.equal(result.code, entry.response.status >= 400 ? 1 : 0, result.stderr);
      assert.equal(calls.length, 1);
      const call = calls[0];
      const actual = new URL(call.url, url);
      assert.equal(call.method, entry.expect.method);
      assert.equal(actual.pathname, entry.expect.path);
      const query = Object.fromEntries([...new Set(actual.searchParams.keys())].map((key) => [key, actual.searchParams.getAll(key)]));
      assert.deepEqual(query, entry.expect.query);
      for (const [name, value] of Object.entries(entry.expect.headers)) assert.equal(call.headers[name], value ?? undefined, name);
      if (entry.expect.body) assert.deepEqual(JSON.parse(call.body), entry.expect.body);
      else assert.equal(call.body, '');
      if (result.code === 1) {
        assert.equal(result.stdout, '');
        assert.equal(JSON.parse(result.stderr).status, entry.response.status);
      } else {
        const envelope = JSON.parse(result.stdout);
        assert.equal(envelope.status, entry.response.status);
        assert.equal(envelope.kind, entry.response.status === 204 || entry.response.status === 303 ? 'empty' : 'body' in entry.response ? 'json' : 'text');
        assert.deepEqual(envelope.data, entry.response.body ?? entry.response.text ?? null);
        if (entry.response.status === 303) assert.equal(envelope.location, entry.response.headers.location);
      }
    });
  });

  await t.test('serializes repeated arrays and wildcard without leaking unrelated OAuth', async () => {
    const calls = [];
    const server = createServer((request, response) => {
      calls.push({ url: request.url, headers: request.headers });
      response.setHeader('content-type', 'application/json');
      response.end(JSON.stringify({ data: [{ id: 111, name: 'Fixture Bible' }] }));
    });
    await new Promise((done) => server.listen(0, '127.0.0.1', done));
    t.after(() => { server.closeAllConnections(); server.close(); });
    const result = await run(['bibles', 'collection-get', '--language-ranges', 'en', '--language-ranges', 'fr',
      '--page-size', '*', '--fields', 'id', '--fields', 'name', '--base-url', `http://127.0.0.1:${server.address().port}`],
    { YOUVERSION_APP_KEY: 'fake-app-key', YOUVERSION_ACCESS_TOKEN: 'unrelated-oauth' });
    assert.equal(result.code, 0, result.stderr);
    assert.deepEqual(JSON.parse(result.stdout), {
      operationId: 'bibles.collection_get', status: 200, kind: 'json', data: { data: [{ id: 111, name: 'Fixture Bible' }] },
    });
    assert.equal(calls.length, 1);
    const url = new URL(calls[0].url, 'http://127.0.0.1');
    assert.deepEqual(url.searchParams.getAll('language_ranges[]'), ['en', 'fr']);
    assert.deepEqual(url.searchParams.getAll('fields[]'), ['id', 'name']);
    assert.equal(url.searchParams.get('page_size'), '*');
    assert.equal(calls[0].headers['x-yvp-app-key'], 'fake-app-key');
    assert.equal(calls[0].headers.authorization, undefined);
  });

  await t.test('token issuance requires both action and disclosure consent before dispatch', async () => {
    const { calls, url } = await fixture({ status: 201, body: { token: 'new-exchange-17', token_type: 'data_exchange', expires_in: 300 } });
    const file = join(consumer, 'permissions.json');
    await writeFile(file, JSON.stringify({ requested_permissions: ['highlights'] }));
    const args = ['data-exchange', 'token-post', '--body-file', file, '--base-url', url];
    const env = { YOUVERSION_APP_KEY: 'fake-token-app-key', YOUVERSION_ACCESS_TOKEN: 'fake-token-access-token' };
    for (const consent of [[], ['--yes'], ['--show-sensitive']]) {
      const refused = await run([...args, ...consent], env);
      assert.equal(refused.code, 2);
      assert.equal(refused.stdout, '');
      assert.equal(calls.length, 0);
    }
    const result = await run([...args, '--yes', '--show-sensitive'], env);
    assert.equal(result.code, 0, result.stderr);
    assert.deepEqual(JSON.parse(result.stdout).data, { token: 'new-exchange-17', token_type: 'data_exchange', expires_in: 300 });
    assert.equal(calls.length, 1);
    assert.equal(calls[0].method, 'POST');
    assert.equal(calls[0].headers.authorization, 'Bearer fake-token-access-token');
    assert.deepEqual(JSON.parse(calls[0].body), { requested_permissions: ['highlights'] });
  });

  await t.test('guards configured credentials in JSON keys, values, text, and callback URLs', async () => {
    const appKey = 'fixture-key+with/escapes="';
    const oauth = 'fixture-access-token';
    const exchange = 'fixture-exchange-token';
    const env = { YOUVERSION_APP_KEY: appKey, YOUVERSION_ACCESS_TOKEN: oauth, YOUVERSION_DATA_EXCHANGE_TOKEN: exchange };
    const { url } = await fixture({ status: 200, body: {
      [appKey]: appKey, nested: [oauth, encodeURIComponent(appKey), JSON.stringify(appKey).slice(1, -1)],
    } });
    const result = await run(['fonts', 'collection-get', '--base-url', url, '--show-sensitive'], env);
    assert.equal(result.code, 0, result.stderr);
    assert.deepEqual(JSON.parse(result.stdout).data, { '[REDACTED]': '[REDACTED]', nested: ['[REDACTED]', '[REDACTED]', '[REDACTED]'] });
    for (const secret of [appKey, oauth, exchange, encodeURIComponent(appKey)]) assert.equal((result.stdout + result.stderr).includes(secret), false);

    const css = await fixture({ status: 200, text: `/* ${oauth} */\nbody { color: red; }\n` });
    const text = await run(['fonts', 'stylesheet-get', '--font-id', '17', '--output', 'text', '--base-url', css.url], env);
    assert.equal(text.code, 0, text.stderr);
    assert.equal(text.stdout, '/* [REDACTED] */\nbody { color: red; }\n');
    const empty = await fixture({ status: 204 });
    const noContent = await run(['fonts', 'stylesheet-get', '--font-id', '17', '--base-url', empty.url], env);
    assert.equal(noContent.code, 0, noContent.stderr);
    assert.equal(JSON.parse(noContent.stdout).kind, 'empty');
    assert.equal(JSON.parse(noContent.stdout).data, null);

    const redirect = await fixture({ status: 303, headers: { location: `http://127.0.0.1/callback?token=${exchange}&app_key=${encodeURIComponent(appKey)}` }, text: '<html>redirecting</html>' });
    const args = ['data-exchange', 'approval-post', '--auth-mode', 'exchange-token', '--yes', '--base-url', redirect.url];
    const hidden = await run(args, env);
    assert.equal(hidden.code, 0, hidden.stderr);
    assert.equal(JSON.parse(hidden.stdout).location, null);
    assert.equal(JSON.parse(hidden.stdout).redacted, true);
    const shown = await run([...args, '--show-sensitive'], env);
    assert.equal(shown.code, 0, shown.stderr);
    assert.equal(JSON.parse(shown.stdout).location, 'http://127.0.0.1/callback?token=[REDACTED]&app_key=[REDACTED]');
    assert.equal(redirect.calls.length, 2, 'callback must not be followed');
    assert.equal(redirect.calls[0].headers.authorization, undefined);
    const punctuation = await run(['fonts', 'collection-get', '--base-url', url], { YOUVERSION_APP_KEY: ':' });
    assert.equal(punctuation.code, 0, punctuation.stderr);
    assert.equal(JSON.parse(punctuation.stdout).status, 200, 'guarding must not replace JSON syntax');
  });

  await t.test('execution deadline covers stalled body consumption and leaves mutation outcome unknown', async () => {
    const { calls, url } = await fixture((_request, response) => {
      response.writeHead(200, { 'content-type': 'application/json' });
      response.write('{"data":');
    });
    const file = join(consumer, 'highlight.json');
    const body = { request_id: '10000000-0000-4000-8000-000000000017', highlight: { bible_id: 111, passage_id: 'JHN.3.16', color: '11aa44' } };
    await writeFile(file, JSON.stringify(body));
    const started = Date.now();
    const result = await run(['highlights', 'collection-post', '--body-file', file, '--yes', '--show-sensitive',
      '--base-url', url, '--timeout-seconds', '0.1'], { YOUVERSION_APP_KEY: 'fake-deadline-app', YOUVERSION_ACCESS_TOKEN: 'fake-deadline-oauth' });
    assert.equal(result.code, 1, result.stderr);
    assert.equal(result.stdout, '');
    const diagnostic = JSON.parse(result.stderr);
    assert.match(diagnostic.error, /deadline/i);
    assert.equal(diagnostic.outcome, 'unknown');
    assert.equal(diagnostic.status, undefined, 'synthetic 499 is not an API status');
    assert.ok(Date.now() - started < 2500);
    assert.equal(calls.length, 1, 'neither SDK nor CLI retries');
    assert.deepEqual(JSON.parse(calls[0].body), body);
  });

  await t.test('invalid inputs and ambiguous credentials never dispatch', async () => {
    const { calls, url } = await fixture({ status: 200, body: {} });
    const env = { YOUVERSION_APP_KEY: 'validation-app', YOUVERSION_ACCESS_TOKEN: 'validation-oauth', YOUVERSION_DATA_EXCHANGE_TOKEN: 'validation-exchange' };
    const file = join(consumer, 'invalid-body.json');
    const body = { request_id: '10000000-0000-4000-8000-000000000017', highlight: { bible_id: 111, passage_id: 'JHN.3.16', color: '11aa44' } };
    await writeFile(file, JSON.stringify({ ...body, highlight: { ...body.highlight, extra: true } }));
    const invalid = [
      ['bibles', 'collection-get'],
      ['bibles', 'collection-get', '--language-ranges', 'en', '--all-available', 'yes'],
      ['bibles', 'collection-get', '--language-ranges', 'en', '--page-size', '1.2'],
      ['bibles', 'collection-get', '--language-ranges', 'en', '--page-size', '9007199254740992'],
      ['bibles', 'resource-get', '--bible-id-path', '1', '--bible-id-path', '2'],
      ['fonts', 'collection-get', '--token', 'must-not-echo'],
      ['fonts', 'collection-get', '--body-file', file],
      ['fonts', 'collection-get', '--timeout-seconds', 'Infinity'],
      ['fonts', 'collection-get', '--timeout-seconds', '0'],
      ['fonts', 'collection-get', '--timeout-seconds', '301'],
      ['fonts', 'collection-get', '--base-url', 'http://example.com'],
      ['fonts', 'collection-get', '--base-url', 'http://127.example.test'],
      ['fonts', 'collection-get', '--base-url', 'https://user:pass@example.com'],
      ['fonts', 'collection-get', '--base-url', 'https://example.com/?query=1'],
      ['fonts', 'collection-get', '--output', 'text'],
      ['data-exchange', 'approval-get', '--output', 'text'],
      ['data-exchange', 'approval-post', '--yes'],
      ['highlights', 'collection-post', '--yes', '--body-file', file],
      ['highlights', 'collection-post', '--yes', '--body-file', file, '--body-stdin'],
    ];
    for (const args of invalid) {
      const result = await run(args.includes('--base-url') ? args : [...args, '--base-url', url], env);
      assert.equal(result.code, 2, `${args}: ${result.stderr}`);
      assert.equal(result.stdout, '');
      assert.equal(result.stderr.includes('must-not-echo'), false);
      assert.equal(calls.length, 0);
    }
    for (const [args, missingEnv] of [
      [['fonts', 'collection-get'], {}],
      [['highlights', 'collection-get'], { YOUVERSION_APP_KEY: 'validation-app' }],
      [['data-exchange', 'approval-get'], { YOUVERSION_APP_KEY: 'validation-app', YOUVERSION_ACCESS_TOKEN: 'validation-oauth' }],
      [['data-exchange', 'approval-post', '--yes', '--auth-mode', 'oauth'], { YOUVERSION_APP_KEY: 'validation-app', YOUVERSION_DATA_EXCHANGE_TOKEN: 'validation-exchange' }],
    ]) {
      assert.equal((await run([...args, '--base-url', url], missingEnv)).code, 2);
      assert.equal(calls.length, 0);
    }
    for (const invalidBody of [{ ...body, extra: true }, { ...body, request_id: 'not-a-uuid' },
      { highlight: body.highlight }, { ...body, highlight: { ...body.highlight, bible_id: 2147483648 } },
      { ...body, highlight: { ...body.highlight, color: 'ABCDEF' } }, []]) {
      await writeFile(file, JSON.stringify(invalidBody));
      const result = await run(['highlights', 'collection-post', '--yes', '--body-file', file, '--base-url', url], env);
      assert.equal(result.code, 2, result.stderr);
      assert.equal(calls.length, 0);
      assert.equal(result.stderr.includes('not-a-uuid'), false);
    }
  });

  await t.test('sensitive reads are withheld independently of write consent', async () => {
    const { url } = await fixture({ status: 200, body: { data: [{ passage_id: 'JHN.3.16', color: '11aa44' }] } });
    const args = ['highlights', 'collection-get', '--bible-id', '111', '--passage-id', 'JHN.3.16', '--base-url', url];
    const env = { YOUVERSION_APP_KEY: 'read-app', YOUVERSION_ACCESS_TOKEN: 'read-oauth' };
    const hidden = JSON.parse((await run([...args, '--yes'], env)).stdout);
    assert.equal(hidden.data, null);
    assert.equal(hidden.redacted, true);
    const shown = JSON.parse((await run([...args, '--show-sensitive'], env)).stdout);
    assert.deepEqual(shown.data, { data: [{ passage_id: 'JHN.3.16', color: '11aa44' }] });
  });

  await t.test('malformed JSON and server errors fail safely without retries', async () => {
    for (const status of [200, 500]) {
      const { calls, url } = await fixture({ status, headers: { 'content-type': 'application/json' }, text: 'invalid-secret-body' });
      const result = await run(['fonts', 'collection-get', '--base-url', url], { YOUVERSION_APP_KEY: 'error-app' });
      assert.equal(result.code, 1);
      assert.equal(result.stdout, '');
      assert.equal(JSON.parse(result.stderr).status, status);
      assert.equal(result.stderr.includes('invalid-secret-body'), false);
      assert.equal(calls.length, 1);
    }
    const { url } = await fixture({ status: 404, text: 'unsafe error prose' });
    const result = await run(['fonts', 'stylesheet-get', '--font-id', '17', '--base-url', url, '--output', 'text'], { YOUVERSION_APP_KEY: 'text-error-app' });
    assert.equal(result.code, 1);
    assert.match(result.stderr, /404/);
    assert.match(result.stderr, /YouVersionPlatformError/);
    assert.equal(result.stderr.includes('unsafe error prose'), false);
  });

  await t.test('stdin bodies preserve caller IDs and interruption has phase-aware outcomes', async () => {
    const fileBody = { request_id: '10000000-0000-4000-8000-000000000021', highlight: { bible_id: 206, passage_id: 'PSA.23.4', color: '11aa44' } };
    const env = { PATH: process.env.PATH, HOME: temporary, YOUVERSION_APP_KEY: 'stdin-app', YOUVERSION_ACCESS_TOKEN: 'stdin-oauth' };
    const launch = (args) => {
      const child = spawn(bin, args, { cwd: consumer, env });
      let stdout = '', stderr = '';
      child.stdout.on('data', (chunk) => { stdout += chunk; });
      child.stderr.on('data', (chunk) => { stderr += chunk; });
      const done = new Promise((resolve) => child.on('close', (code, signal) => resolve({ code, signal, stdout, stderr })));
      return { child, done };
    };
    const success = await fixture({ status: 201, body: { data: [] } });
    const args = ['highlights', 'collection-post', '--body-stdin', '--yes', '--base-url', success.url];
    const input = launch(args);
    input.child.stdin.end(JSON.stringify(fileBody));
    assert.equal((await input.done).code, 0);
    assert.deepEqual(JSON.parse(success.calls[0].body), fileBody);
    const declined = launch(args.filter((arg) => arg !== '--yes'));
    assert.equal((await declined.done).code, 2, 'must refuse before reading stdin');
    assert.equal(success.calls.length, 1);
    for (const [signal, code] of [['SIGINT', 130], ['SIGTERM', 143]]) {
      const started = Date.now();
      const pendingInput = launch(args);
      await new Promise((resolve) => setTimeout(resolve, 300));
      pendingInput.child.kill(signal);
      const cancelledInput = await pendingInput.done;
      assert.equal(cancelledInput.code, code, cancelledInput.stderr);
      assert.equal(JSON.parse(cancelledInput.stderr).outcome, undefined);
      assert.equal(success.calls.length, 1);

      let called;
      const dispatched = new Promise((resolve) => { called = resolve; });
      const stalled = await fixture((_request, response) => { response.writeHead(200); response.write('{'); called(); });
      const active = launch([...args.slice(0, -1), stalled.url]);
      active.child.stdin.end(JSON.stringify(fileBody));
      await dispatched;
      active.child.kill(signal);
      const cancelled = await active.done;
      assert.equal(cancelled.code, code, cancelled.stderr);
      assert.equal(cancelled.stdout, '');
      assert.equal(JSON.parse(cancelled.stderr).outcome, 'unknown');
      assert.equal(JSON.parse(cancelled.stderr).status, undefined);
      assert.equal(stalled.calls.length, 1);
      assert.ok(Date.now() - started < 2500, 'interruption must not wait for an SDK timer');
    }
  });

  await t.test('interactive writes default to refusal and require an affirmative answer', async () => {
    const { calls, url } = await fixture({ status: 204 });
    const args = ['highlights', 'resource-delete', '--bible-id', '206', '--passage-id-path', 'PSA.23.4', '--base-url', url];
    const command = [bin, ...args].map((arg) => `'${arg.replaceAll("'", "'\\''")}'`).join(' ');
    const interactive = (answer) => new Promise((resolve, reject) => {
      const child = spawn('script', ['-q', '-e', '-E', 'never', '-c', command, '/dev/null'], {
        cwd: consumer, env: { PATH: process.env.PATH, HOME: temporary,
          YOUVERSION_APP_KEY: 'interactive-app', YOUVERSION_ACCESS_TOKEN: 'interactive-oauth' },
      });
      let transcript = '', answered = false;
      child.on('error', reject);
      child.stdout.on('data', (chunk) => {
        transcript += chunk;
        if (!answered && transcript.includes('[y/N]')) { answered = true; child.stdin.write(`${answer}\n`); }
      });
      child.on('close', (code) => resolve({ code, transcript }));
    });
    const refused = await interactive('');
    assert.equal(refused.code, 2, refused.transcript);
    assert.match(refused.transcript, /Write declined/);
    assert.equal(calls.length, 0);
    const interrupted = await interactive('\x03');
    assert.equal(interrupted.code, 130, interrupted.transcript);
    assert.equal(calls.length, 0);
    const closed = await interactive('\x04');
    assert.equal(closed.code, 2, closed.transcript);
    assert.match(closed.transcript, /Write declined/);
    assert.equal(calls.length, 0);
    const accepted = await interactive('yes');
    assert.equal(accepted.code, 0, accepted.transcript);
    assert.equal(calls.length, 1);
    assert.equal(calls[0].method, 'DELETE');
    assert.equal(calls[0].url, '/v1/highlights/PSA.23.4?bible_id=206');
    assert.match(accepted.transcript, /"redacted":true/);
  });

  await t.test('output delivery failure is distinct from unknown mutation outcome', async () => {
    const { calls, url } = await fixture({ status: 201, body: { data: 'x'.repeat(100000) } });
    const file = join(consumer, 'delivery.json');
    await writeFile(file, JSON.stringify({ requested_permissions: ['highlights'] }));
    const child = spawn(bin, ['data-exchange', 'token-post', '--body-file', file, '--yes', '--show-sensitive', '--base-url', url], {
      cwd: consumer, env: { PATH: process.env.PATH, HOME: temporary, YOUVERSION_APP_KEY: 'delivery-app', YOUVERSION_ACCESS_TOKEN: 'delivery-oauth' },
    });
    child.stdout.destroy();
    let stderr = '';
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    const code = await new Promise((resolve) => child.on('close', resolve));
    assert.equal(code, 1, stderr);
    assert.match(JSON.parse(stderr).error, /Output delivery failed; the result was received/);
    assert.equal(JSON.parse(stderr).outcome, undefined);
    assert.equal(calls.length, 1);
  });
});
