import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const openapiPath = join(root, 'openapi', 'openapi.json');
const sdkMapPath = join(root, 'src', 'generated', 'sdk-map.json');
const outputPath = join(root, 'packages', 'react-query', 'src', 'generated.ts');
const check = process.argv.includes('--check');

const HTTP_METHODS = new Set(['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace']);
const EXCLUSIONS = new Map([
  ['data_exchange.approval_get', 'Browser approval action with a manual redirect'],
  ['data_exchange.approval_post', 'Browser approval action with a manual redirect'],
]);
const CREDENTIAL_PARAMETER_NAMES = new Set([
  'app_key',
  'x-yvp-app-key',
  'access_token',
  'authorization',
  'token',
]);

function fail(message) {
  throw new Error(`React Query generation failed: ${message}`);
}

function lowerFirst(value) {
  return value.length === 0 ? value : value[0].toLowerCase() + value.slice(1);
}

function upperFirst(value) {
  return value.length === 0 ? value : value[0].toUpperCase() + value.slice(1);
}

function pascalCase(value) {
  return value
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean)
    .map(upperFirst)
    .join('');
}

function property(value) {
  return /^[A-Za-z_$][A-Za-z0-9_$]*$/.test(value) ? value : JSON.stringify(value);
}

function publicOperationName(accessor, method) {
  let candidate = method.replace(/^v1(?=[A-Z])/, '');
  const resourceName = pascalCase(accessor);
  const resourceIndex = candidate.indexOf(resourceName);
  if (resourceIndex !== -1) {
    candidate = candidate.slice(resourceIndex + resourceName.length);
  }
  return lowerFirst(candidate || method);
}

function readOpenapiOperations(openapi) {
  const operations = new Map();
  for (const [path, pathItem] of Object.entries(openapi.paths)) {
    for (const [method, operation] of Object.entries(pathItem)) {
      if (!HTTP_METHODS.has(method)) continue;
      if (operation.operationId == null) fail(`${method.toUpperCase()} ${path} has no operationId`);
      if (operations.has(operation.operationId)) fail(`duplicate operationId ${operation.operationId}`);

      const parameters = [...(pathItem.parameters ?? []), ...(operation.parameters ?? [])]
        .map((parameter) => {
          if (parameter.$ref == null) return parameter;
          const name = parameter.$ref.split('/').at(-1);
          return openapi.components?.parameters?.[name] ?? fail(`unresolved parameter ${parameter.$ref}`);
        })
        .map((parameter) => parameter.name);
      const security = operation.security ?? openapi.security ?? [];

      operations.set(operation.operationId, {
        httpMethod: method.toUpperCase(),
        path,
        parameters,
        emptySuccess: operation.responses?.['204'] != null,
        requiresOAuth: security.some((requirement) =>
          Object.prototype.hasOwnProperty.call(requirement, 'OAuth2'),
        ),
      });
    }
  }
  return operations;
}

function requestMode(accessor, method, hasRequestType) {
  const clientPath = join(
    root,
    'src',
    'generated',
    'api',
    'resources',
    accessor,
    'client',
    'Client.ts',
  );
  const source = readFileSync(clientPath, 'utf8');
  const escapedMethod = method.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const match = source.match(new RegExp(`public ${escapedMethod}\\(\\n([\\s\\S]*?)\\n    \\):`));
  if (match == null) fail(`could not read signature for ${accessor}.${method}`);
  const parameters = match[1].trim();

  if (parameters.startsWith('requestOptions?')) {
    if (hasRequestType) fail(`${accessor}.${method} has request metadata but no request argument`);
    return 'none';
  }
  if (!parameters.startsWith('request:')) fail(`unexpected signature for ${accessor}.${method}`);
  if (!hasRequestType) fail(`${accessor}.${method} has a request argument but no request metadata`);
  return /^request:[\s\S]*?= \{\},/m.test(parameters) ? 'optional' : 'required';
}

function operationKind(operationId, httpMethod) {
  if (EXCLUSIONS.has(operationId)) return 'excluded';
  if (httpMethod === 'GET') return 'query';
  if (httpMethod === 'POST' || httpMethod === 'DELETE') return 'mutation';
  fail(`${operationId} uses unsupported HTTP method ${httpMethod}`);
}

function buildOperations(openapi, sdkMap) {
  const openapiOperations = readOpenapiOperations(openapi);
  const sdkIds = Object.keys(sdkMap).sort();
  const openapiIds = [...openapiOperations.keys()].sort();
  if (JSON.stringify(sdkIds) !== JSON.stringify(openapiIds)) {
    fail('OpenAPI and sdk-map operation sets differ');
  }

  const operations = sdkIds.map((operationId) => {
    const sdk = sdkMap[operationId];
    const contract = openapiOperations.get(operationId);
    if (sdk.httpMethod !== contract.httpMethod || sdk.path !== contract.path) {
      fail(`${operationId} method or path differs between OpenAPI and sdk-map`);
    }
    if (!Array.isArray(sdk.accessor) || sdk.accessor.length !== 1) {
      fail(`${operationId} must have exactly one client accessor`);
    }

    const accessor = sdk.accessor[0];
    const kind = operationKind(operationId, sdk.httpMethod);
    const mode = requestMode(accessor, sdk.method, sdk.requestType != null);
    if (kind === 'mutation' && mode !== 'required') {
      fail(`${operationId} has an unsupported ${mode} mutation request`);
    }
    if (
      kind === 'query' &&
      contract.parameters.some((name) => CREDENTIAL_PARAMETER_NAMES.has(name.toLowerCase()))
    ) {
      fail(`${operationId} has a credential-bearing query parameter and needs an explicit policy`);
    }

    return {
      operationId,
      accessor,
      method: sdk.method,
      publicName: publicOperationName(accessor, sdk.method),
      kind,
      requestMode: mode,
      ...contract,
      exclusionReason: EXCLUSIONS.get(operationId),
    };
  });

  for (const operationId of EXCLUSIONS.keys()) {
    if (!openapiOperations.has(operationId)) fail(`stale exclusion ${operationId}`);
  }

  for (const [accessor, grouped] of Map.groupBy(operations, (operation) => operation.accessor)) {
    const names = grouped
      .filter((operation) => operation.kind !== 'excluded')
      .map((operation) => operation.publicName);
    if (new Set(names).size !== names.length) fail(`${accessor} has colliding public operation names`);
  }

  return operations;
}

function methodType(operation) {
  return `YouVersionPlatformClient[${JSON.stringify(operation.accessor)}][${JSON.stringify(operation.method)}]`;
}

function bindingTypeSource(operation) {
  const method = methodType(operation);
  const result = operation.emptySuccess
    ? `EmptyQueryResult<MethodResult<${method}>>`
    : `MethodResult<${method}>`;
  if (operation.kind === 'query') {
    if (operation.requestMode === 'none') return `RequestlessQueryBinding<${result}>`;
    const binding =
      operation.requestMode === 'optional' ? 'OptionalQueryBinding' : 'RequiredQueryBinding';
    return `${binding}<MethodRequest<${method}>, ${result}>`;
  }
  return `MutationBinding<MethodRequest<${method}>, MethodResult<${method}>>`;
}

function bindingSource(operation) {
  const variable = `${operation.accessor}${upperFirst(operation.publicName)}`;
  const common = [
    `    cacheScope,`,
    `    resource: ${JSON.stringify(operation.accessor)},`,
    `    operationId: ${JSON.stringify(operation.operationId)},`,
  ];

  if (operation.kind === 'query') {
    common.push(`    emptyAsNull: ${operation.emptySuccess},`);
    if (operation.requestMode === 'none') {
      common.push(
        `    execute: (requestOptions) => client.${operation.accessor}.${operation.method}(requestOptions),`,
      );
      return `  const ${variable} = createRequestlessQueryBinding({\n${common.join('\n')}\n  });`;
    }
    common.push(
      `    execute: (`,
      `      request: NonNullable<Parameters<${methodType(operation)}>[0]>,`,
      `      requestOptions,`,
      `    ) => client.${operation.accessor}.${operation.method}(request, requestOptions),`,
    );
    const factory =
      operation.requestMode === 'optional' ? 'createOptionalQueryBinding' : 'createRequiredQueryBinding';
    return `  const ${variable} = ${factory}({\n${common.join('\n')}\n  });`;
  }

  common.push(
    `    execute: (`,
    `      request: NonNullable<Parameters<${methodType(operation)}>[0]>,`,
    `      requestOptions,`,
    `    ) => client.${operation.accessor}.${operation.method}(request, requestOptions),`,
  );
  return `  const ${variable} = createMutationBinding({\n${common.join('\n')}\n  });`;
}

function manifestSource(operations) {
  const entries = operations.map((operation) => {
    const value = {
      kind: operation.kind,
      httpMethod: operation.httpMethod,
      path: operation.path,
      accessor: operation.accessor,
      method: operation.method,
      publicName: operation.publicName,
      requestMode: operation.requestMode,
      emptySuccess: operation.emptySuccess,
      requiresOAuth: operation.requiresOAuth,
      ...(operation.exclusionReason == null ? {} : { exclusionReason: operation.exclusionReason }),
    };
    return `  ${JSON.stringify(operation.operationId)}: ${JSON.stringify(value)},`;
  });
  return `export const reactQueryOperationManifest = {\n${entries.join('\n')}\n} as const;`;
}

function resourceSource(accessor, operations) {
  const active = operations.filter((operation) => operation.kind !== 'excluded');
  const properties = [];
  if (active.some((operation) => operation.kind === 'query')) {
    properties.push(`      queryFilters: () => resourceQueryFilters(cacheScope, ${JSON.stringify(accessor)}),`);
  }
  if (active.some((operation) => operation.kind === 'mutation')) {
    properties.push(
      `      mutationFilters: () => resourceMutationFilters(cacheScope, ${JSON.stringify(accessor)}),`,
    );
  }
  for (const operation of active) {
    properties.push(
      `      ${property(operation.publicName)}: ${operation.accessor}${upperFirst(operation.publicName)},`,
    );
  }
  return `    ${property(accessor)}: {\n${properties.join('\n')}\n    },`;
}

function publicResourceTypeSource(accessor, operations) {
  const active = operations.filter((operation) => operation.kind !== 'excluded');
  const properties = [];
  if (active.some((operation) => operation.kind === 'query')) {
    properties.push('    queryFilters(): QueryFilters;');
  }
  if (active.some((operation) => operation.kind === 'mutation')) {
    properties.push('    mutationFilters(): MutationFilters;');
  }
  for (const operation of active) {
    properties.push(`    ${property(operation.publicName)}: ${bindingTypeSource(operation)};`);
  }
  return `  ${property(accessor)}: {\n${properties.join('\n')}\n  };`;
}

function generate(operations) {
  const active = operations.filter((operation) => operation.kind !== 'excluded');
  const resources = [...Map.groupBy(active, (operation) => operation.accessor).entries()].sort(([a], [b]) =>
    a.localeCompare(b),
  );
  return `// This file is generated by scripts/generate-react-query.mjs.\n\nimport type { YouVersionPlatformClient } from '@cameronapak/platform-sdk';\nimport type { MutationFilters, QueryFilters } from '@tanstack/react-query';\nimport {\n  createMutationBinding,\n  createOptionalQueryBinding,\n  createRequestlessQueryBinding,\n  createRequiredQueryBinding,\n  resourceMutationFilters,\n  resourceQueryFilters,\n  rootMutationFilters,\n  rootQueryFilters,\n  validateCacheScope,\n  type MutationBinding,\n  type OptionalQueryBinding,\n  type RequestlessQueryBinding,\n  type RequiredQueryBinding,\n} from './runtime.js';\n\ntype ClientMethod = (...args: never[]) => unknown;\ntype MethodRequest<TMethod extends ClientMethod> = NonNullable<Parameters<TMethod>[0]>;\ntype MethodResult<TMethod extends ClientMethod> = Awaited<ReturnType<TMethod>>;\ntype EmptyQueryResult<TResult> = Exclude<TResult, undefined> | null;\n\nexport interface CreatePlatformQueriesOptions {\n  client: YouVersionPlatformClient;\n  /** Non-secret identity and representation boundary for every cache entry. */\n  cacheScope: string;\n}\n\nexport interface PlatformQueries {\n  queryFilters(): QueryFilters;\n  mutationFilters(): MutationFilters;\n${resources.map(([accessor, grouped]) => publicResourceTypeSource(accessor, grouped)).join('\n')}\n}\n\n${manifestSource(operations)}\n\nexport function createPlatformQueries(options: CreatePlatformQueriesOptions): PlatformQueries {\n  const client = options.client;\n  const cacheScope = validateCacheScope(options.cacheScope);\n\n${active.map(bindingSource).join('\n\n')}\n\n  return {\n    queryFilters: () => rootQueryFilters(cacheScope),\n    mutationFilters: () => rootMutationFilters(cacheScope),\n${resources.map(([accessor, grouped]) => resourceSource(accessor, grouped)).join('\n')}\n  };\n}\n`;
}

const openapi = JSON.parse(readFileSync(openapiPath, 'utf8'));
const sdkMap = JSON.parse(readFileSync(sdkMapPath, 'utf8'));
const source = generate(buildOperations(openapi, sdkMap));

if (check) {
  if (!existsSync(outputPath) || readFileSync(outputPath, 'utf8') !== source) {
    fail(`${outputPath} is stale; run pnpm generate:react-query`);
  }
  console.log('React Query bindings are current');
} else {
  mkdirSync(dirname(outputPath), { recursive: true });
  writeFileSync(outputPath, source);
  console.log(`Generated React Query bindings at ${outputPath}`);
}
