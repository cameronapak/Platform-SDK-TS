const credentialNames = ['YOUVERSION_APP_KEY', 'YOUVERSION_ACCESS_TOKEN', 'YOUVERSION_DATA_EXCHANGE_TOKEN', 'YOUVERSION_DATA_EXCHANGE_APP_KEY'];
const forms = new Set<string>();
for (const name of credentialNames) {
  const value = process.env[name];
  if (!value) continue;
  const encoded = encodeURIComponent(value);
  for (const form of [value, encoded, encoded.replace(/%[0-9A-F]{2}/g, (escape) => escape.toLowerCase()),
    encodeURI(value), new URLSearchParams({ value }).toString().slice(6), JSON.stringify(value).slice(1, -1)]) forms.add(form);
}
const secrets = [...forms].sort((a, b) => b.length - a.length);

export function guardText(value: string): string {
  for (const secret of secrets) value = value.replaceAll(secret, '[REDACTED]');
  return value;
}

export function guard(value: unknown): unknown {
  if (typeof value === 'string') return guardText(value);
  if (Array.isArray(value)) return value.map(guard);
  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, child]) => [guardText(key), guard(child)]));
  }
  return value;
}

export function human(value: string): string {
  return guardText(value).replace(/[\x00-\x1f\x7f-\x9f]/g, (character) => `\\u${character.charCodeAt(0).toString(16).padStart(4, '0')}`);
}

// Prevent stream errors from triggering Node's unguarded uncaught-error output.
process.stdout.on('error', () => {});
process.stderr.on('error', () => {});

async function deliver(stream: NodeJS.WriteStream, value: string, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  let aborted: (() => void) | undefined;
  try {
    await new Promise<void>((resolve, reject) => {
      aborted = () => reject(signal.reason);
      signal.addEventListener('abort', aborted, { once: true });
      stream.write(value, (error) => error ? reject(error) : resolve());
    });
    signal.throwIfAborted();
  } finally {
    if (aborted) signal.removeEventListener('abort', aborted);
  }
}

export async function write(stream: NodeJS.WriteStream, value: string, signal: AbortSignal): Promise<void> {
  await deliver(stream, guardText(value), signal);
}

export async function json(stream: NodeJS.WriteStream, value: unknown, signal: AbortSignal): Promise<void> {
  // Guard the data, not serialized syntax (a credential may contain punctuation).
  await deliver(stream, `${JSON.stringify(guard(value))}\n`, signal);
}
