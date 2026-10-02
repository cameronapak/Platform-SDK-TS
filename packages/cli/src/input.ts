import { parseArgs, type ParseArgsConfig } from 'node:util';
import { readFile } from 'node:fs/promises';
import { isIP } from 'node:net';
import { Ajv } from 'ajv';
import addFormats from 'ajv-formats';
import type { Operation, Schema } from './operation.js';

export class UsageError extends Error {}

export const globalOptions: NonNullable<ParseArgsConfig['options']> = {
  help: { type: 'boolean' }, version: { type: 'boolean' },
  output: { type: 'string' }, yes: { type: 'boolean' }, 'show-sensitive': { type: 'boolean' },
  'timeout-seconds': { type: 'string' }, 'base-url': { type: 'string' },
  'body-file': { type: 'string' }, 'body-stdin': { type: 'boolean' }, 'auth-mode': { type: 'string' },
};

const validator = new Ajv({ strict: true, allErrors: true });
addFormats.default(validator);
validator.addFormat('int32', { type: 'number', validate: (value: number) => Number.isInteger(value) && value >= -2147483648 && value <= 2147483647 });

export function validate(schema: Schema, value: unknown, label: string): void {
  if (!validator.validate(schema, value)) {
    const error = validator.errors?.[0];
    throw new UsageError(`${label}${error?.instancePath ?? ''}: invalid ${error?.keyword ?? 'value'}. See command help.`);
  }
}

function scalar(value: string, schema: Schema): unknown {
  if (schema.anyOf) {
    for (const candidate of schema.anyOf) {
      try {
        const parsed = scalar(value, candidate);
        if (validator.validate(candidate, parsed)) return parsed;
      } catch (error) {
        if (!(error instanceof UsageError)) throw error;
      }
    }
    throw new UsageError('Invalid union input. See command help.');
  }
  if (schema.type === 'integer') {
    if (!/^-?(0|[1-9]\d*)$/.test(value) || !Number.isSafeInteger(Number(value))) throw new UsageError('Expected a safe integer.');
    return Number(value);
  }
  if (schema.type === 'boolean') {
    if (!['true', 'false'].includes(value)) throw new UsageError('Expected true or false.');
    return value === 'true';
  }
  return value;
}

export function parseOptions(args: string[], operation?: Operation) {
  const options = { ...globalOptions };
  for (const input of operation?.inputs ?? []) {
    if (!input.secretEnv) options[input.flag] = { type: 'string', multiple: input.schema.type === 'array' };
  }
  try {
    const parsed = parseArgs({ args, options, tokens: true, allowPositionals: false, strict: true });
    const seen = new Set<string>();
    for (const token of parsed.tokens ?? []) {
      if (token.kind !== 'option') continue;
      if (seen.has(token.name) && !options[token.name].multiple) throw new UsageError(`Duplicate --${token.name}.`);
      seen.add(token.name);
    }
    return parsed.values;
  } catch (error) {
    if (error instanceof UsageError) throw error;
    throw new UsageError('Invalid options. Use --help; credential values belong in environment inputs, not arguments.');
  }
}

export function requestInputs(operation: Operation, values: ReturnType<typeof parseOptions>) {
  const request: Record<string, unknown> = {};
  const headers: Record<string, string> = {};
  for (const input of operation.inputs) {
    if (input.secretEnv) continue;
    const value = values[input.flag];
    if (value === undefined) {
      if (input.required) throw new UsageError(`Required --${input.flag}.`);
      continue;
    }
    let parsed;
    try {
      parsed = input.schema.type === 'array'
        ? (value as string[]).map((item) => scalar(item, input.schema.items!))
        : scalar(value as string, input.schema);
    } catch (error) {
      if (!(error instanceof UsageError)) throw error;
      throw new UsageError(`--${input.flag}: ${error.message}`);
    }
    validate(input.schema, parsed, `--${input.flag}`);
    if (input.location === 'header') headers[input.name] = parsed as string;
    else request[input.name] = parsed;
  }
  return { request, headers };
}

export function destination(value: unknown): string | undefined {
  if (value === undefined) return undefined;
  let url;
  try { url = new URL(value as string); } catch { throw new UsageError('Invalid --base-url.'); }
  const loopback = url.hostname === 'localhost' || url.hostname === '[::1]' || (isIP(url.hostname) === 4 && url.hostname.startsWith('127.'));
  if (url.username || url.password || url.search || url.hash || (url.protocol !== 'https:' && !(url.protocol === 'http:' && loopback))) {
    throw new UsageError('--base-url must use HTTPS or loopback HTTP, without credentials, query, or fragment.');
  }
  return url.href;
}

export async function readBody(operation: Operation, values: ReturnType<typeof parseOptions>, signal: AbortSignal) {
  const file = values['body-file'];
  const stdin = values['body-stdin'];
  if (!operation.body) {
    if (file || stdin) throw new UsageError('This operation has no request body.');
    return {};
  }
  if (Boolean(file) === Boolean(stdin)) throw new UsageError('Select exactly one of --body-file or --body-stdin.');
  let text = '';
  const abort = () => process.stdin.destroy(new Error('Input interrupted'));
  try {
    signal.throwIfAborted();
    if (file) text = await readFile(file as string, { encoding: 'utf8', signal });
    else {
      signal.addEventListener('abort', abort, { once: true });
      for await (const chunk of process.stdin) text += chunk.toString();
    }
  } catch { throw new UsageError('Cannot read the JSON body input.'); }
  finally { signal.removeEventListener('abort', abort); }
  let body;
  try { body = JSON.parse(text); } catch { throw new UsageError('Body input must contain valid JSON.'); }
  validate(operation.body, body, 'body');
  return body as Record<string, unknown>;
}

export function credentials(operation: Operation, values: ReturnType<typeof parseOptions>, request: Record<string, unknown>) {
  const appKey = process.env.YOUVERSION_APP_KEY;
  const accessToken = process.env.YOUVERSION_ACCESS_TOKEN;
  const exchangeToken = process.env.YOUVERSION_DATA_EXCHANGE_TOKEN;
  if (!appKey?.trim()) throw new UsageError('Set YOUVERSION_APP_KEY in the environment.');
  let auth = operation.auth;
  if (auth === 'approval') {
    const mode = values['auth-mode'];
    if (mode !== undefined && mode !== 'exchange-token' && mode !== 'oauth') throw new UsageError('--auth-mode must be exchange-token or oauth.');
    if (!mode && Boolean(accessToken) === Boolean(exchangeToken)) throw new UsageError('Approval requires an unambiguous --auth-mode exchange-token or oauth selection.');
    auth = mode === 'oauth' || (!mode && accessToken) ? 'oauth' : 'exchange-token';
  } else if (values['auth-mode'] !== undefined) throw new UsageError('--auth-mode is only available for approval-post.');
  if (auth === 'oauth' && !accessToken?.trim()) throw new UsageError('Set YOUVERSION_ACCESS_TOKEN in the environment.');
  if (auth === 'exchange-token' && !exchangeToken?.trim()) throw new UsageError('Set YOUVERSION_DATA_EXCHANGE_TOKEN in the environment.');
  for (const input of operation.inputs) {
    if (!input.secretEnv || input.location === 'header' || (input.name === 'token' && auth !== 'exchange-token')) continue;
    const value = process.env[input.secretEnv];
    if (value) {
      validate(input.schema, value, input.secretEnv);
      request[input.name] = value;
    }
  }
  return { yvpAppKey: appKey, ...(auth === 'oauth' ? { token: accessToken } : { auth: false as const }) };
}
