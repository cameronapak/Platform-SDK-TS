import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

// Match the pinned generator's public methods to OpenAPI wire signatures.
// The consumer verifies the real requests against independent shared fixtures.
export function writeOperationMap(source, specFile) {
  const spec = JSON.parse(readFileSync(specFile, 'utf8'));
  const root = readFileSync(join(source, 'src/PlatformClient.php'), 'utf8');
  const requests = new Map();
  for (const [, resource, accessor] of root.matchAll(/public (\w+)Client \$(\w+);/g)) {
    const code = readFileSync(join(source, `src/${resource}/${resource}Client.php`), 'utf8');
    const methods = [...code.matchAll(/    public function (\w+)\(([^\n]+)\): ([^\n]+)\n/g)];
    for (let i = 0; i < methods.length; i++) {
      const [, method, signature] = methods[i];
      if (method.endsWith('WithResponse')) continue;
      const block = code.slice(methods[i].index, methods[i + 1]?.index ?? code.length);
      const path = block.match(/path: "([^"]+)"/)?.[1];
      const verb = block.match(/method: HttpMethod::(\w+)/)?.[1];
      if (!path || !verb) throw new Error(`Unrecognized PHP endpoint ${resource}.${method}`);
      const args = [...signature.matchAll(/(?:\w+|\?array) \$(\w+)/g)].map(([, name]) => name).filter((name) => name !== 'options');
      const requestType = signature.match(/(\w+Request) \$request/)?.[1];
      const fields = new Map([...block.matchAll(/\$query\['([^']+)'\] = \$request->(\w+);/g)].map(([, wire, field]) => [wire, field]));
      const pathArgs = [...path.matchAll(/\{\$(\w+)\}/g)].map(([, name]) => name);
      const key = `${verb} /${path.replaceAll(/\{\$\w+\}/g, '{}')}`;
      if (requests.has(key)) throw new Error(`Duplicate PHP operation: ${key}`);
      requests.set(key, { accessor, method, args, requestType: requestType ? `Cameronapak\\PlatformSdk\\${resource}\\Requests\\${requestType}` : null, fields, pathArgs, resource });
    }
  }
  const mapping = {};
  for (const [path, item] of Object.entries(spec.paths)) {
    for (const [verb, operation] of Object.entries(item)) {
      if (!['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'].includes(verb)) continue;
      const key = `${verb.toUpperCase()} ${path.replaceAll(/\{[^}]+\}/g, '{}')}`;
      const generated = requests.get(key);
      if (!generated) throw new Error(`Missing generated PHP operation: ${key}`);
      requests.delete(key);
      const pathNames = [...path.matchAll(/\{([^}]+)\}/g)].map(([, name]) => name);
      const parameters = {};
      for (let parameter of [...(item.parameters ?? []), ...(operation.parameters ?? [])]) {
        if (parameter.$ref) parameter = spec.components.parameters[parameter.$ref.split('/').at(-1)];
        if (parameter.in === 'header') continue;
        const field = parameter.in === 'path' ? generated.pathArgs[pathNames.indexOf(parameter.name)] : generated.fields.get(parameter.name);
        if (!field) throw new Error(`Missing PHP parameter ${parameter.name} in ${operation.operationId}`);
        parameters[parameter.name] = field;
      }
      if (generated.requestType) {
        const request = readFileSync(join(source, `src/${generated.resource}/Requests/${generated.requestType.split('\\').at(-1)}.php`), 'utf8');
        for (const name of Object.keys(operation.requestBody?.content?.['application/json']?.schema?.properties ?? {})) {
          const field = request.match(new RegExp(`#\\[JsonProperty\\('${name}'\\)[^\\n]*\\][\\s\\S]*?public [^;]+ \\$(\\w+);`))?.[1];
          if (!field) throw new Error(`Missing PHP body property ${name}`);
          parameters[name] = field;
        }
      }
      mapping[operation.operationId] = {
        accessor: generated.accessor, method: generated.method, args: generated.args,
        requestType: generated.requestType, parameters, httpMethod: verb.toUpperCase(), path,
      };
    }
  }
  if (requests.size) throw new Error(`Unexpected generated PHP operations: ${[...requests.keys()]}`);
  writeFileSync(join(source, 'sdk-map.json'), `${JSON.stringify(mapping, null, 2)}\n`);
}
