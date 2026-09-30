import { execFileSync } from 'node:child_process';
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  renameSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareOpenapi, replaceExactly } from './generation.mjs';

const FORGE_REPOSITORY = 'https://github.com/cloudflare/forge.git';
const FORGE_COMMIT = '00b8ede867530f8891fe4124b2f5f20ee8d0b05e';
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const spec = join(root, 'openapi', 'openapi.json');
const forge = resolve(process.env.FORGE_DIR ?? join(root, '.cache', 'forge'));
const forgeSpec = join(root, '.cache', 'openapi.fern.json');
const output = join(root, '.forge-output');
const generated = join(root, 'src', 'generated');

function run(command, args, cwd = root) {
  execFileSync(command, args, { cwd, stdio: 'inherit' });
}

function adaptMethod(source, method, transform, file) {
  const startMarker = `    private async __${method}(`;
  const start = source.indexOf(startMarker);
  if (start === -1) throw new Error(`Missing ${method} in ${file}`);
  const nextMethod = source.indexOf('\n    /**', start);
  const end = nextMethod === -1 ? source.length : nextMethod;
  return source.slice(0, start) + transform(source.slice(start, end)) + source.slice(end);
}

function adaptGeneratedSdk(directory) {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      adaptGeneratedSdk(path);
    } else if (entry.name.endsWith('.ts')) {
      let source = readFileSync(path, 'utf8').replaceAll('CloudflareApi', 'YouVersionPlatform');
      if (entry.name === 'getResponseBody.ts') {
        source = replaceExactly(
          source,
          "import { unwrapCloudflareEnvelope } from './unwrapCloudflareEnvelope.js';\n",
          '',
          1,
          path,
        );
        source = replaceExactly(
          source,
          '    return unwrapCloudflareEnvelope(responseBody, responseType);',
          '    return responseBody;',
          1,
          path,
        );
      } else if (entry.name === 'BaseClient.ts') {
        source = replaceExactly(
          source,
          '    normalized.authProvider ??= new BearerAuthProvider(normalizedWithNoOpAuthProvider);',
          '    normalized.authProvider ??= BearerAuthProvider.canCreate(normalized)\n' +
            '        ? new BearerAuthProvider(normalizedWithNoOpAuthProvider)\n' +
            '        : new core.NoOpAuthProvider();',
          1,
          path,
        );
      } else if (path.endsWith('/core/fetcher/Fetcher.ts')) {
        source = replaceExactly(
          source,
          'const SENSITIVE_HEADERS = new Set([\n    "authorization",',
          'const SENSITIVE_HEADERS = new Set([\n    "authorization",\n    "x-yvp-app-key",',
          1,
          path,
        );
        source = replaceExactly(
          source,
          'const SENSITIVE_QUERY_PARAMS = new Set([\n    "api_key",',
          'const SENSITIVE_QUERY_PARAMS = new Set([\n    "app_key",\n    "api_key",',
          1,
          path,
        );
        source = replaceExactly(
          source,
          '        withCredentials?: boolean;\n        abortSignal?: AbortSignal;',
          '        withCredentials?: boolean;\n        redirect?: RequestRedirect;\n        abortSignal?: AbortSignal;',
          1,
          path,
        );
        source = replaceExactly(
          source,
          '                    args.withCredentials,\n                    args.duplex,',
          '                    args.withCredentials,\n                    args.redirect,\n                    args.duplex,',
          1,
          path,
        );
      } else if (path.endsWith('/core/fetcher/makeRequest.ts')) {
        source = replaceExactly(
          source,
          '    withCredentials?: boolean,\n    duplex?: "half",',
          '    withCredentials?: boolean,\n    redirect?: RequestRedirect,\n    duplex?: "half",',
          1,
          path,
        );
        source = replaceExactly(
          source,
          '        credentials: withCredentials ? "include" : undefined,\n        // @ts-ignore',
          '        credentials: withCredentials ? "include" : undefined,\n        redirect,\n        // @ts-ignore',
          1,
          path,
        );
      } else if (path.endsWith('/core/fetcher/makePassthroughRequest.ts')) {
        source = replaceExactly(
          source,
          '                effectiveInit?.credentials === "include",\n                undefined, // duplex',
          '                effectiveInit?.credentials === "include",\n' +
            '                effectiveInit?.redirect,\n' +
            '                undefined, // duplex',
          1,
          path,
        );
      } else if (path.endsWith('/api/resources/dataExchange/client/Client.ts')) {
        source = adaptMethod(
          source,
          'approvalGet',
          (methodSource) =>
            replaceExactly(
              methodSource,
              '            method: "GET",\n            headers: _headers,',
              '            method: "GET",\n            headers: _headers,\n            redirect: "manual",',
              1,
              path,
            ),
          path,
        );
        source = adaptMethod(
          source,
          'approvalPost',
          (methodSource) => {
            let adapted = replaceExactly(
              methodSource,
              '        };\n        let _headers: core.Fetcher.Args["headers"] = mergeHeaders(\n            this._options?.headers,',
              '        };\n        const _authRequest: core.AuthRequest =\n            token == null ? await this._options.authProvider.getAuthRequest() : { headers: {} };\n        let _headers: core.Fetcher.Args["headers"] = mergeHeaders(\n            _authRequest.headers,\n            this._options?.headers,',
              1,
              path,
            );
            adapted = replaceExactly(
              adapted,
              '            method: "POST",\n            headers: _headers,',
              '            method: "POST",\n            headers: _headers,\n            redirect: "manual",',
              1,
              path,
            );
            return adapted;
          },
          path,
        );
      } else if (path.endsWith('/api/resources/fonts/client/Client.ts')) {
        source = adaptMethod(
          source,
          'v1FontsStylesheetGet',
          (methodSource) =>
            replaceExactly(
              methodSource,
              '            queryString: core.url.queryBuilder().mergeAdditional(requestOptions?.queryParams).build(),',
              '            queryString: core.url\n' +
                '                .queryBuilder()\n' +
                '                .add("app_key", await core.Supplier.get(requestOptions?.yvpAppKey ?? this._options?.yvpAppKey))\n' +
                '                .mergeAdditional(requestOptions?.queryParams)\n' +
                '                .build(),',
              1,
              path,
            ),
          path,
        );
      } else if (
        path.endsWith('/api/resources/languages/client/Client.ts') ||
        path.endsWith('/api/resources/organizations/client/Client.ts')
      ) {
        source = replaceExactly(
          source,
          '            this._options?.headers,\n            mergeOnlyDefinedHeaders({\n                "Accept-Language": "en",\n                "X-YVP-App-Key":',
          '            mergeOnlyDefinedHeaders({ "Accept-Language": "en" }),\n' +
            '            this._options?.headers,\n' +
            '            mergeOnlyDefinedHeaders({\n                "X-YVP-App-Key":',
          2,
          path,
        );
      } else if (
        path.endsWith('/BiblesCollectionGetRequest.ts') ||
        path.endsWith('/V1SearchQueriesCollectionGetRequest.ts') ||
        path.endsWith('/V1SearchUnifiedCollectionGetRequest.ts') ||
        path.endsWith('/V1SearchTopicsCollectionGetRequest.ts')
      ) {
        source = replaceExactly(
          source,
          '    "language_ranges[]"?: string | string[];',
          '    "language_ranges[]": string | string[];',
          1,
          path,
        );
      } else if (
        path.endsWith('/api/resources/bibles/client/Client.ts') ||
        path.endsWith('/api/resources/searchQueries/client/Client.ts')
      ) {
        source = replaceExactly(source, 'Request = {},', 'Request,', 2, path);
      }
      writeFileSync(path, source);
    }
  }

  const apiError = join(directory, 'errors', 'CloudflareApiError.ts');
  const timeoutError = join(directory, 'errors', 'CloudflareApiTimeoutError.ts');
  if (existsSync(apiError)) {
    renameSync(apiError, join(directory, 'errors', 'YouVersionPlatformError.ts'));
  }
  if (existsSync(timeoutError)) {
    renameSync(timeoutError, join(directory, 'errors', 'YouVersionPlatformTimeoutError.ts'));
  }
  rmSync(join(directory, 'core', 'fetcher', 'unwrapCloudflareEnvelope.ts'), { force: true });
  rmSync(join(directory, '.fernignore'), { force: true });
}

if (!existsSync(join(forge, '.git'))) {
  mkdirSync(dirname(forge), { recursive: true });
  run('git', ['clone', FORGE_REPOSITORY, forge]);
}

run('git', ['fetch', '--depth', '1', 'origin', FORGE_COMMIT], forge);
run('git', ['checkout', '--detach', FORGE_COMMIT], forge);
run('pnpm', ['install', '--frozen-lockfile'], forge);

// The public Forge checkout omits the Cloudflare API inputs that its package
// build expects. These files only seed package metadata; generation still uses
// the YouVersion spec passed to the Forge CLI below.
mkdirSync(join(forge, 'packages', 'cloudflare-fern-config', 'fern'), { recursive: true });
cpSync(spec, join(forge, 'openapi.json'));
cpSync(spec, join(forge, 'packages', 'cloudflare-fern-config', 'fern', 'openapi.json'));

run('pnpm', ['--filter', '@cloudflare/forge-transformer-sdk-ts', 'build'], forge);
prepareOpenapi(spec, forgeSpec);
rmSync(output, { recursive: true, force: true });
run(
  'node',
  [join(forge, 'packages', 'cloudflare-forge-transformer-sdk-ts', 'dist', 'cli.js'), forgeSpec, '--out', output],
  root,
);

rmSync(generated, { recursive: true, force: true });
cpSync(join(output, 'sdk'), generated, { recursive: true });
adaptGeneratedSdk(generated);
run('node', [join(root, 'scripts', 'generate-react-query.mjs')], root);
console.log(`Generated YouVersion Platform SDK at ${generated}`);
