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
