import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

// Inspect the pinned generated request methods, then match their wire signatures
// to OpenAPI. The consumer tests also invoke each mapped method independently.
export function writeOperationMap(source, specFile) {
  const spec = JSON.parse(readFileSync(specFile, 'utf8'));
  const client = readFileSync(join(source, 'client/client.go'), 'utf8');
  const accessors = new Map([...client.matchAll(/^\t(\w+)\s+\*(\w+)\.Client$/gm)].map(([, field, pkg]) => [pkg, field]));
  const types = new Map();
  for (const file of readdirSync(source).filter((file) => file.endsWith('.go'))) {
    const code = readFileSync(join(source, file), 'utf8');
    for (const [, name, body] of code.matchAll(/type (\w+Request) struct \{([\s\S]*?)\n\}/g)) {
      const fields = [...body.matchAll(/^\t(\w+)\s+[^\n`]+`json:"([^"]+)" url:"([^"]+)"`/gm)]
        .map(([, field, json, url]) => ({ field, json: json.split(',')[0], url: url.split(',')[0] }));
      types.set(name, fields);
    }
  }
  const requests = new Map();
  for (const [pkg, accessor] of accessors) {
    const code = readFileSync(join(source, pkg, 'raw_client.go'), 'utf8');
    for (const [, method, block] of code.matchAll(/func \(r \*RawClient\) (\w+)\(([\s\S]*?)(?=\nfunc |$)/g)) {
      const requestType = block.match(/request \*platform\.(\w+)/)?.[1];
      const fields = types.get(requestType) ?? [];
      const verb = block.match(/Method:\s+http\.Method(\w+)/)?.[1].toUpperCase();
      const pathArgs = [];
      let path = block.match(/endpointURL := baseURL \+ "([^"]+)"/)?.[1];
      if (!path) {
        const encoding = block.match(/endpointURL := internal.EncodeURL\(\s*baseURL\+"([^"]+)",([\s\S]*?)\n\t\)/);
        if (!encoding) throw new Error(`Unrecognized Go endpoint URL in ${pkg}.${method}`);
        pathArgs.push(...[...encoding[2].matchAll(/request\.(\w+)/g)].map((match) => match[1]));
        let index = 0;
        path = encoding[1].replaceAll('%v', () => `{${index++}}`);
        if (index !== pathArgs.length) throw new Error(`Go path arguments changed in ${pkg}.${method}`);
      }
      const key = `${verb} ${path.replaceAll(/\{[^}]+\}/g, '{}')}`;
      if (requests.has(key)) throw new Error(`Duplicate generated Go operation: ${key}`);
      requests.set(key, { accessor, method, requestType, fields, pathArgs });
    }
  }
  const mapping = {};
  const verbs = new Set(['get', 'post', 'delete', 'put', 'patch', 'head', 'options', 'trace']);
  for (const [path, item] of Object.entries(spec.paths)) {
    for (const [verb, operation] of Object.entries(item)) {
      if (!verbs.has(verb)) continue;
      const key = `${verb.toUpperCase()} ${path.replaceAll(/\{[^}]+\}/g, '{}')}`;
      const generated = requests.get(key);
      if (!generated) throw new Error(`Missing generated Go operation: ${key}`);
      requests.delete(key);
      const pathNames = [...path.matchAll(/\{([^}]+)\}/g)].map((match) => match[1]);
      const parameters = {};
      const requestParameters = [...(item.parameters ?? []), ...(operation.parameters ?? [])];
      for (let parameter of requestParameters) {
        if (parameter.$ref) parameter = spec.components.parameters[parameter.$ref.split('/').at(-1)];
        if (parameter.in === 'header') continue;
        const field = parameter.in === 'path'
          ? generated.pathArgs[pathNames.indexOf(parameter.name)]
          : generated.fields.find((field) => field.url === parameter.name)?.field;
        if (!field || !generated.fields.some((entry) => entry.field === field)) {
          throw new Error(`Missing generated Go parameter ${parameter.name} for ${operation.operationId}`);
        }
        parameters[parameter.name] = field;
      }
      const body = operation.requestBody?.content?.['application/json']?.schema;
      for (const name of Object.keys(body?.properties ?? {})) {
        const field = generated.fields.find((field) => field.json === name)?.field;
        if (!field) throw new Error(`Missing generated Go body property ${name}`);
        parameters[name] = field;
      }
      mapping[operation.operationId] = {
        accessor: generated.accessor, method: generated.method, requestType: generated.requestType ?? null,
        httpMethod: verb.toUpperCase(), path, parameters,
      };
    }
  }
  if (requests.size) throw new Error(`Unexpected generated Go operations: ${[...requests.keys()]}`);
  writeFileSync(join(source, 'sdk-map.json'), `${JSON.stringify(mapping, null, 2)}\n`);
}
