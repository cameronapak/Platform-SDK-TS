import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

// Fern treats examples of mixed numeric/string query schemas as strings.
// Keep the authoritative OpenAPI document unchanged for every language.
export function prepareOpenapi(source, destination) {
  const openapi = JSON.parse(readFileSync(source, 'utf8'));
  function stringifyQueryExamples(value) {
    if (Array.isArray(value)) {
      value.forEach(stringifyQueryExamples);
      return;
    }
    if (value === null || typeof value !== 'object') return;
    if (value.in === 'query' && value.schema) {
      for (const schema of [value.schema, ...(value.schema.anyOf ?? [])]) {
        if (typeof schema.example === 'number') schema.example = String(schema.example);
      }
    }
    Object.values(value).forEach(stringifyQueryExamples);
  }
  stringifyQueryExamples(openapi);
  mkdirSync(dirname(destination), { recursive: true });
  writeFileSync(destination, `${JSON.stringify(openapi, null, 2)}\n`);
}

export function replaceExactly(source, search, replacement, expected, file) {
  const matches = source.split(search).length - 1;
  if (matches !== expected) {
    throw new Error(`Expected ${expected} generation pattern(s) in ${file}, found ${matches}`);
  }
  return source.replaceAll(search, replacement);
}

// Keep parse failures in Fern's native error channel, not in API response data.
export function adaptResponseParsing(source, file) {
  const replace = (search, replacement) => {
    source = replaceExactly(source, search, replacement, 1, file);
  };
  if (file.endsWith('/getResponseBody.ts')) {
    replace("import { getBinaryResponse } from './BinaryResponse.js';", `import { getBinaryResponse } from './BinaryResponse.js';

export class NonJsonResponseError extends Error {
  constructor(public readonly statusCode: number, public readonly rawBody: string) {
    super('Response body is not JSON');
  }
}`);
    replace(`      return {
        ok: false,
        error: {
          reason: 'non-json',
          statusCode: response.status,
          rawBody: text,
        },
      };`, '      throw new NonJsonResponseError(response.status, text);');
  } else if (file.endsWith('/Fetcher.ts')) {
    replace('import { getResponseBody } from "./getResponseBody.js";',
      'import { getResponseBody, NonJsonResponseError } from "./getResponseBody.js";');
    replace('            const body = await getResponseBody(response, args.responseType);', `            let body: unknown;
            try {
                if (response.status >= 300 && args.redirect === "manual" && args.responseType == null) {
                    await response.text();
                } else {
                    body = await getResponseBody(response, args.responseType);
                }
            } catch (error) {
                if (!(error instanceof NonJsonResponseError)) throw error;
                return {
                    ok: false,
                    error: { reason: "non-json", statusCode: error.statusCode, rawBody: error.rawBody },
                    rawResponse: toRawResponse(response),
                };
            }`);
  } else if (file.endsWith('/getErrorResponseBody.ts')) {
    replace('import { getResponseBody } from "./getResponseBody.js";',
      'import { getResponseBody, NonJsonResponseError } from "./getResponseBody.js";');
    replace('        return getResponseBody(response);', `        try {
            return await getResponseBody(response);
        } catch (error) {
            if (!(error instanceof NonJsonResponseError)) throw error;
            return error.rawBody;
        }`);
    source = replaceExactly(source, 'return text.length > 0 ? fromJson(text) : undefined;',
      'try { return text.length > 0 ? fromJson(text) : undefined; } catch { return text; }', 2, file);
  }
  return source;
}
