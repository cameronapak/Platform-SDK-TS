import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const target = join(root, 'packages/cli');
const output = join(target, 'src/generated.ts');
const approval = join(target, 'contract.sha256');
const check = process.argv.includes('--check');
const approve = process.argv.includes('--approve-contract');
const fail = (message) => { throw new Error(`CLI generation failed: ${message}`); };
if (check && approve) fail('--check cannot approve a contract');
if (process.argv.slice(2).some((arg) => !['--check', '--approve-contract'].includes(arg))) fail('unknown argument');

const require = createRequire(join(target, 'package.json'));
const Ajv = require('ajv');
const addFormats = require('ajv-formats');
const validator = new Ajv({ strict: true, allErrors: true });
addFormats(validator);
validator.addFormat('int32', { type: 'number', validate: (value) => Number.isInteger(value) && value >= -2147483648 && value <= 2147483647 });

const openapi = JSON.parse(readFileSync(join(root, 'openapi/openapi.json'), 'utf8'));
const sdkMap = JSON.parse(readFileSync(join(root, 'src/generated/sdk-map.json'), 'utf8'));
const methods = new Set(['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace']);
const globals = new Set(['help', 'version', 'output', 'yes', 'show-sensitive', 'timeout-seconds', 'base-url', 'body-file', 'body-stdin', 'auth-mode']);
// Exceptions only. Endpoint discovery, names, types, and dispatch remain generated.
const policy = {
  'data_exchange.approval_get': { auth: 'exchange-token', sensitive: true, redirect: true },
  'data_exchange.approval_post': { auth: 'approval', sensitive: true, redirect: true },
  'data_exchange.token_post': { requiresDisclosure: true, sensitive: true },
};
const secrets = { token: 'YOUVERSION_DATA_EXCHANGE_TOKEN', 'x-yvp-app-key': 'YOUVERSION_DATA_EXCHANGE_APP_KEY', Authorization: 'YOUVERSION_ACCESS_TOKEN' };
const annotations = new Set(['description', 'example', 'default', 'title']);
const keywords = new Set(['type', 'format', 'properties', 'required', 'items', 'anyOf', 'enum', 'pattern', 'minimum', 'maximum', 'minLength', 'maxLength', 'minItems', 'maxItems', 'additionalProperties']);
const kebab = (value) => value.replace(/([a-z0-9])([A-Z])/g, '$1-$2').replaceAll('_', '-').replace(/\[\]$/, '').toLowerCase();
const pascal = (value) => value.split(/[^A-Za-z0-9]+/).map((word) => word[0].toUpperCase() + word.slice(1)).join('');

function dereference(value) {
  if (value.$ref == null) return value;
  if (!value.$ref.startsWith('#/')) fail(`external reference ${value.$ref}`);
  return value.$ref.slice(2).split('/').reduce((owner, key) => owner?.[key.replaceAll('~1', '/').replaceAll('~0', '~')], openapi) ?? fail(`unresolved ${value.$ref}`);
}

function schema(value) {
  value = dereference(value);
  const result = value.description === undefined ? {} : { description: value.description };
  for (const [key, child] of Object.entries(value)) {
    if (annotations.has(key)) continue;
    if (!keywords.has(key)) fail(`unsupported schema keyword ${key}`);
    result[key] = key === 'properties'
      ? Object.fromEntries(Object.entries(child).map(([name, definition]) => [name, schema(definition)]))
      : key === 'items' ? schema(child)
      : key === 'anyOf' ? child.map(schema) : child;
  }
  if (result.type === 'object') result.additionalProperties = false;
  if (result.type === 'integer') {
    result.minimum = Math.max(result.minimum ?? Number.MIN_SAFE_INTEGER, Number.MIN_SAFE_INTEGER);
    result.maximum = Math.min(result.maximum ?? Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER);
  }
  validator.compile(result);
  return result;
}

const contracts = new Map();
for (const [path, item] of Object.entries(openapi.paths)) {
  for (const [method, operation] of Object.entries(item)) {
    if (!methods.has(method)) continue;
    const id = operation.operationId;
    if (!id || contracts.has(id)) fail(`missing or duplicate operationId at ${path}`);
    const parameters = new Map();
    for (const candidate of [...(item.parameters ?? []), ...(operation.parameters ?? [])]) {
      const parameter = dereference(candidate);
      parameters.set(`${parameter.in}:${parameter.name}`, parameter);
    }
    contracts.set(id, { ...operation, path, httpMethod: method.toUpperCase(), parameters: [...parameters.values()] });
  }
}
if (JSON.stringify([...contracts.keys()].sort()) !== JSON.stringify(Object.keys(sdkMap).sort())) fail('OpenAPI and SDK inventories differ');
for (const id of Object.keys(policy)) if (!contracts.has(id)) fail(`stale policy ${id}`);

const bindingSources = {};
const operations = [];
const calls = [];
for (const id of Object.keys(sdkMap).sort()) {
  const entry = sdkMap[id];
  const contract = contracts.get(id);
  if (entry.path !== contract.path || entry.httpMethod !== contract.httpMethod) fail(`${id}: method/path mismatch`);
  if (entry.accessor.length !== 1) fail(`${id}: unsupported accessor`);
  const accessor = entry.accessor[0];
  const clientPath = join(root, 'src/generated/api/resources', accessor, 'client/Client.ts');
  const client = ts.createSourceFile(clientPath, readFileSync(clientPath, 'utf8'), ts.ScriptTarget.Latest, true);
  const member = client.statements.filter(ts.isClassDeclaration).flatMap((node) => node.members)
    .find((node) => ts.isMethodDeclaration(node) && node.name.getText(client) === entry.method);
  if (!member) fail(`${id}: missing SDK method`);
  const signature = member.parameters.map((parameter) => parameter.getText(client));
  const hasRequest = signature[0]?.startsWith('request:');
  if (hasRequest !== Boolean(entry.requestType) || member.parameters.length !== (hasRequest ? 2 : 1)) fail(`${id}: unsupported SDK signature`);
  bindingSources[id] = { signature };

  const inputs = contract.parameters.map((parameter) => {
    if (!['path', 'query', 'header'].includes(parameter.in)) fail(`${id}: unsupported parameter location`);
    const secretEnv = secrets[parameter.name];
    if (parameter.in === 'header' && parameter.name !== 'Accept-Language' && !secretEnv) fail(`${id}: unclassified header ${parameter.name}`);
    if (/token|authorization|app.?key/i.test(parameter.name) && !secretEnv && parameter.name !== 'page_token') fail(`${id}: unclassified credential input`);
    return { name: parameter.name, flag: kebab(parameter.name), location: parameter.in,
      required: parameter.required === true, description: parameter.description ?? '', schema: schema(parameter.schema),
      ...(secretEnv ? { secretEnv } : {}) };
  });
  let body;
  if (contract.requestBody) {
    const definition = dereference(contract.requestBody);
    if (!definition.required || Object.keys(definition.content).join() !== 'application/json') fail(`${id}: unsupported body`);
    body = schema(definition.content['application/json'].schema);
    if (body.type !== 'object') fail(`${id}: body must be an object`);
  }
  const expectedFields = [...inputs.filter((input) => input.location !== 'header').map((input) => input.name), ...Object.keys(body?.properties ?? {})].sort();
  if (entry.requestType) {
    const requestPath = join(dirname(clientPath), 'requests', `${entry.requestType}.ts`);
    const requestSource = readFileSync(requestPath, 'utf8');
    // Include nested namespace declarations, not only top-level property references.
    bindingSources[id].requestSource = requestSource;
    const requestAst = ts.createSourceFile(requestPath, requestSource, ts.ScriptTarget.Latest, true);
    const declaration = requestAst.statements.find((node) => ts.isInterfaceDeclaration(node) && node.name.text === entry.requestType);
    if (!declaration || declaration.heritageClauses) fail(`${id}: unsupported request declaration`);
    const fields = declaration.members.map((node) => {
      if (!ts.isPropertySignature(node) || !node.type || !node.name) fail(`${id}: unsupported request member`);
      return { name: ts.isStringLiteral(node.name) ? node.name.text : node.name.getText(requestAst), required: !node.questionToken, type: node.type.getText(requestAst) };
    });
    if (JSON.stringify(fields.map((field) => field.name).sort()) !== JSON.stringify(expectedFields)) fail(`${id}: request properties mismatch`);
    const required = [...inputs.filter((input) => input.required && input.location !== 'header').map((input) => input.name), ...(body?.required ?? [])].sort();
    if (JSON.stringify(fields.filter((field) => field.required).map((field) => field.name).sort()) !== JSON.stringify(required)) fail(`${id}: request requiredness mismatch`);
    bindingSources[id].fields = fields;
  } else if (expectedFields.length) fail(`${id}: request metadata missing`);

  let action = entry.method.replace(/^v1(?=[A-Z])/, '');
  const resource = pascal(accessor);
  const position = action.indexOf(resource);
  if (position !== -1) action = action.slice(position + resource.length);
  const command = [kebab(accessor), kebab(action || entry.method)];
  const flags = inputs.filter((input) => !input.secretEnv).map((input) => input.flag);
  if (new Set(flags).size !== flags.length || flags.some((flag) => globals.has(flag))) fail(`${id}: flag collision`);
  const security = contract.security ?? openapi.security ?? [];
  if (security.some((requirement) => Object.keys(requirement).some((name) => !['yvpAppKey', 'yvpAppKeyQuery', 'OAuth2'].includes(name)))) fail(`${id}: unknown auth scheme`);
  const oauth = security.some((requirement) => 'OAuth2' in requirement) || inputs.some((input) => input.name === 'Authorization' && input.required);
  const success = Object.entries(contract.responses).filter(([status]) => /^[23]\d\d$/.test(status));
  const content = [...new Set(success.flatMap(([, response]) => Object.keys(dereference(response).content ?? {})))];
  if (content.length > 1 || content.some((type) => !['application/json', 'text/css', 'text/html'].includes(type))) fail(`${id}: unsupported success representation`);
  const metadata = { operationId: id, command, httpMethod: entry.httpMethod, path: entry.path,
    description: [contract.summary, contract.description].filter(Boolean).join('\n'),
    kind: ['GET', 'HEAD', 'OPTIONS'].includes(entry.httpMethod) ? 'read' : 'write',
    auth: oauth ? 'oauth' : 'app-key', sensitive: oauth, requiresDisclosure: false,
    response: content.length === 0 ? 'empty' : content[0] === 'application/json' ? 'json' : 'text',
    redirect: false, inputs, ...(body ? { body } : {}), ...policy[id] };
  operations.push(metadata);
  const methodType = `YouVersionPlatformClient[${JSON.stringify(accessor)}][${JSON.stringify(entry.method)}]`;
  const request = hasRequest ? `{ ${bindingSources[id].fields.map((field) => {
    const name = JSON.stringify(field.name);
    return `${name}: request[${name}] as NonNullable<Parameters<${methodType}>[0]>[${name}]`;
  }).join(', ')} }, ` : '';
  calls.push(`  { ...metadata[${operations.length - 1}], execute: (client, request, options) => client.${accessor}.${entry.method}(${request}options).withRawResponse() },`);
}
if (new Set(operations.map((operation) => operation.command.join(' '))).size !== operations.length) fail('command collision');
const digest = createHash('sha256').update(JSON.stringify({ openapi, sdkMap, bindingSources, policy })).digest('hex');
if (approve) writeFileSync(approval, `${digest}\n`);
if (!existsSync(approval) || readFileSync(approval, 'utf8').trim() !== digest) fail('contract changed; review its source diff, then run generate:cli --approve-contract');
const source = `// Generated by scripts/generate-cli.mjs. Do not edit.\nimport type { YouVersionPlatformClient } from '@cameronapak/platform-sdk';\nimport type { Operation } from './operation.js';\n\nconst metadata: Omit<Operation, 'execute'>[] = ${JSON.stringify(operations, null, 2)};\n\nexport const operations: Operation[] = [\n${calls.join('\n')}\n];\n`;
if (check) {
  if (!existsSync(output) || readFileSync(output, 'utf8') !== source) fail('generated commands are stale; run generate:cli');
  console.log(`CLI generation is current (${operations.length} operations)`);
} else {
  mkdirSync(dirname(output), { recursive: true });
  writeFileSync(output, source);
  console.log(`Generated CLI (${operations.length} operations)`);
}
