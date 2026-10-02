#!/usr/bin/env node
import { YouVersionPlatformClient, YouVersionPlatformError } from '@cameronapak/platform-sdk';
import { createInterface } from 'node:readline/promises';
import { operations } from './generated.js';
import { credentials, destination, parseOptions, readBody, requestInputs, UsageError } from './input.js';
import { human, json, write } from './output.js';
import type { Operation, Schema } from './operation.js';

const controller = new AbortController();
let cancellation: 'SIGINT' | 'SIGTERM' | 'deadline' | undefined;
let selected: Operation | undefined;
let dispatched = false;
let received = false;
let rendering = false;
let output = 'json';
const cancel = (reason: typeof cancellation) => {
  if (controller.signal.aborted) return;
  cancellation = reason;
  controller.abort(reason);
};
const interrupt = () => cancel('SIGINT');
const terminate = () => cancel('SIGTERM');
process.once('SIGINT', interrupt);
process.once('SIGTERM', terminate);

function inputType(schema: Schema): string {
  if (schema.anyOf) return schema.anyOf.map(inputType).join(' | ');
  if (schema.enum) return schema.enum.map((value) => JSON.stringify(value)).join(' | ');
  if (schema.type === 'array') return `${inputType(schema.items!)} (repeat flag)`;
  return schema.type === 'boolean' ? 'true | false' : schema.type ?? 'value';
}

function help(operation: Operation): string {
  const inputs = operation.inputs.map((input) => input.secretEnv
    ? `  ${input.secretEnv} (environment, ${input.required ? 'required' : 'optional'})`
    : `  --${input.flag} <${inputType(input.schema)}> [${input.required ? 'required' : 'optional'}${input.schema.type === 'array' ? ', repeat' : ''}]\n    ${human(input.description)}\n    Constraints: ${human(JSON.stringify(input.schema))}`);
  return `yvp ${operation.command.join(' ')}\n${operation.operationId}\n${operation.httpMethod} ${operation.path} [${operation.kind}]\n${human(operation.description)}\n
Inputs:\n${inputs.join('\n')}
${operation.body ? `\nBody: exactly one of --body-file <path> or --body-stdin.\nDeclared fields only, including nested objects. No defaults or request IDs inserted.\n${human(JSON.stringify(operation.body, null, 2))}\n` : ''}
Credentials: YOUVERSION_APP_KEY is required for execution. Authentication: ${operation.auth}.
${operation.auth === 'oauth' ? 'Set YOUVERSION_ACCESS_TOKEN.' : operation.auth === 'exchange-token' ? 'Set YOUVERSION_DATA_EXCHANGE_TOKEN.' : operation.auth === 'approval' ? 'Use YOUVERSION_DATA_EXCHANGE_TOKEN or YOUVERSION_ACCESS_TOKEN. With both, select --auth-mode exchange-token|oauth.' : 'Unrelated OAuth is not sent.'}

Options:
  --yes                   Consent to a write; otherwise TTY confirmation defaults to no.
  --show-sensitive        Disclose sensitive results, never configured credentials.${operation.requiresDisclosure ? ' Required before this operation dispatches.' : ''}
  --output json|text      Default JSON envelope. Raw text only for ordinary text operations.
  --timeout-seconds <n>   Execution deadline after validation/consent, including body read. Default 30, positive and at most 300.
  --base-url <url>        Trusted initial HTTPS or loopback HTTP destination. No URL credentials, query, or fragment.
  --help                  Display help without a request or reading stdin.

Sensitive results: ${operation.sensitive ? 'metadata-only unless --show-sensitive.' : 'ordinary output.'}
No retries. Approval callbacks are never followed; exit 0 means completion, not permission grant.
Exit codes: 0 success; 1 request/deadline/delivery failure; 2 input/refusal; 130 SIGINT; 143 SIGTERM.
Interrupted writes after dispatch can have unknown outcomes. Check before retrying.\n`;
}

async function main(args: string[]) {
  const operation = operations.find((entry) => entry.command[0] === args[0] && entry.command[1] === args[1]);
  if ((args.length === 1 && args[0] === '--version') || (operation && args.slice(2).includes('--version'))) {
    return write(process.stdout, '0.1.0\n', controller.signal);
  }
  if (args.length === 1 && args[0] === 'catalog') return json(process.stdout, operations, controller.signal);
  if (operation && args.slice(2).includes('--help')) {
    return write(process.stdout, help(operation), controller.signal);
  }
  if (args.length === 0 || (args.length === 1 && args[0] === '--help')) {
    return write(process.stdout, `Experimental, unofficial Platform CLI\nUsage: yvp <resource> <action> [options]\nyvp catalog\n${operations.map((entry) => `  ${entry.command.join(' ')} [${entry.kind}]`).join('\n')}\n`, controller.signal);
  }
  if (!operation) throw new UsageError('Unknown command. Use yvp catalog or --help.');
  selected = operation;
  const values = parseOptions(args.slice(2), operation);
  if (values.output !== undefined && !['json', 'text'].includes(values.output as string)) throw new UsageError('--output must be json or text.');
  output = (values.output as string | undefined) ?? 'json';
  if (values.output === 'text' && (operation.response !== 'text' || operation.redirect || operation.sensitive)) {
    throw new UsageError('--output text is only available for ordinary text operations; approval requires JSON output.');
  }
  const seconds = values['timeout-seconds'] === undefined ? 30 : Number(values['timeout-seconds']);
  if (!Number.isFinite(seconds) || seconds <= 0 || seconds > 300) throw new UsageError('--timeout-seconds must be greater than 0 and at most 300.');
  if (operation.kind === 'excluded') throw new UsageError(operation.exclusionReason ?? 'This operation is excluded.');
  const { request, headers } = requestInputs(operation, values);
  const auth = credentials(operation, values, request);
  const baseUrl = destination(values['base-url']);
  if (operation.requiresDisclosure && !values['show-sensitive']) throw new UsageError('Token issuance requires --show-sensitive before dispatch.');
  if (operation.kind === 'write' && values['body-stdin'] && !values.yes) throw new UsageError('--body-stdin writes require --yes before reading stdin.');
  Object.assign(request, await readBody(operation, values, controller.signal));
  if (operation.kind === 'write' && !values.yes) {
    if (!process.stdin.isTTY || !process.stderr.isTTY || values['body-stdin']) throw new UsageError('Writes require --yes in automation or with --body-stdin.');
    const prompt = createInterface({ input: process.stdin, output: process.stderr });
    const inputClosed = () => controller.abort('confirmation-closed');
    prompt.once('SIGINT', interrupt);
    prompt.once('close', inputClosed);
    try {
      const answer = await prompt.question(human(`Execute ${operation.operationId} [write]? [y/N] `), { signal: controller.signal }).catch((error) => {
        if (cancellation) throw error;
        throw new UsageError('Write declined; no request dispatched.');
      });
      if (!/^y(es)?$/i.test(answer.trim())) throw new UsageError('Write declined; no request dispatched.');
    } finally {
      prompt.removeListener('close', inputClosed);
      prompt.close();
    }
  }
  const client = new YouVersionPlatformClient({
    ...auth, maxRetries: 0, baseUrl, headers, timeoutInSeconds: seconds, logging: { silent: true },
  });
  controller.signal.throwIfAborted();
  const timer = setTimeout(() => cancel('deadline'), seconds * 1000);
  let result;
  try {
    dispatched = true;
    result = await operation.execute(client, request, { maxRetries: 0, timeoutInSeconds: seconds, abortSignal: controller.signal });
    received = true;
    controller.signal.throwIfAborted();
  } finally { clearTimeout(timer); }
  const { data, rawResponse } = result;
  const redacted = operation.sensitive && !values['show-sensitive'];
  const empty = rawResponse.status === 204 || data === undefined;
  rendering = true;
  if (values.output === 'text') return write(process.stdout, empty ? '' : data as string, controller.signal);
  await json(process.stdout, { operationId: operation.operationId, status: rawResponse.status,
    kind: empty ? 'empty' : operation.response, data: redacted || empty ? null : data ?? null,
    ...(operation.redirect ? { location: values['show-sensitive'] ? rawResponse.headers.get('location') : null } : {}),
    ...(redacted ? { redacted: true } : {}) }, controller.signal);
}

try { await main(process.argv.slice(2)); } catch (error) {
  process.exitCode = cancellation === 'SIGINT' ? 130 : cancellation === 'SIGTERM' ? 143 : error instanceof UsageError ? 2 : 1;
  const status = !cancellation && error instanceof YouVersionPlatformError ? error.statusCode : undefined;
  const errorClasses = ['YouVersionPlatformError', 'YouVersionPlatformTimeoutError', 'BadRequestError', 'UnauthorizedError', 'NotFoundError', 'UnprocessableEntityError'];
  const errorClass = error instanceof Error && errorClasses.includes(error.constructor.name) ? error.constructor.name : undefined;
  const message = cancellation === 'deadline' ? 'Execution deadline exceeded.' : cancellation ? received
    ? 'Output delivery interrupted; the result was received.' : 'Command interrupted.'
    : error instanceof UsageError ? error.message : rendering ? 'Output delivery failed; the result was received.'
    : 'Platform SDK request failed. Check credentials, permissions, and command inputs.';
  const unknown = selected?.kind === 'write' && dispatched && !received && (cancellation || status === undefined);
  const diagnostic = { error: message, ...(selected ? { operationId: selected.operationId } : {}),
    ...(status !== undefined ? { status } : {}), ...(errorClass ? { errorClass } : {}),
    ...(unknown ? { outcome: 'unknown', detail: 'The request may have been applied. Do not retry without checking its outcome.' } : {}) };
  // Signal diagnostics are best effort; a blocked stderr must not delay shutdown.
  const diagnosticSignal = controller.signal.aborted ? AbortSignal.timeout(100) : controller.signal;
  if (output === 'text') await write(process.stderr, `${human(message)}${status === undefined ? '' : ` HTTP ${status}.`}${errorClass ? ` ${errorClass}.` : ''}\n`, diagnosticSignal).catch(() => { process.exitCode = 1; });
  else await json(process.stderr, diagnostic, diagnosticSignal).catch(() => { process.exitCode = 1; });
} finally {
  process.removeListener('SIGINT', interrupt);
  process.removeListener('SIGTERM', terminate);
  // Output has completed or been interrupted. SDK timers must not delay shutdown.
  process.exit(cancellation === 'SIGINT' ? 130 : cancellation === 'SIGTERM' ? 143 : process.exitCode ?? 0);
}
