import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

// Derive callable methods from Fern output, then independently match OpenAPI.
// The resulting consumer dispatch compiles actual calls to every mapped method.
export function writeOperationMap(directory, specFile) {
  const spec = JSON.parse(readFileSync(specFile, 'utf8'));
  const types = new Map();
  for (const file of readdirSync(join(directory, 'src/api/types'))) {
    const code = readFileSync(join(directory, 'src/api/types', file), 'utf8');
    const declaration = code.match(/pub struct (\w+) \{([\s\S]*?)\n\}/);
    if (!declaration) continue;
    const fields = [...declaration[2].matchAll(/((?:    #\[serde[^\n]*\]\n)*)    pub (\w+): ([^\n]+),/g)]
      .map(([, attrs, field, type]) => ({ field, type, wire: attrs.match(/rename = "([^"]+)"/)?.[1] ?? field }));
    types.set(declaration[1], fields);
  }
  const generated = new Map();
  const resources = join(directory, 'src/api/resources');
  for (const resource of readdirSync(resources).filter((name) => name !== 'mod.rs')) {
    const code = readFileSync(join(resources, resource, `${resource}.rs`), 'utf8');
    for (const [, method, block] of code.matchAll(/pub async fn (\w+)\(([\s\S]*?)\n    \}/g)) {
      if (method.endsWith('_with_raw_response')) continue;
      const signature = block.split(') ->')[0];
      const pathArgs = [...signature.matchAll(/\b(\w+): (i64|&str)/g)].map(([, name, type]) => ({ name, type }));
      const requestType = signature.match(/request: &(\w+)/)?.[1] ?? null;
      const verb = block.match(/Method::(\w+)/)?.[1];
      const path = block.match(/Method::\w+,\s*(?:&format!\(\s*)?"([^"]+)"/)?.[1];
      if (!verb || !path) throw new Error(`Rust wire signature changed: ${resource}.${method}`);
      const key = `${verb} /${path.replaceAll('{}', '{param}')}`;
      if (generated.has(key)) throw new Error(`Duplicate Rust operation: ${key}`);
      const query = new Map([...block.matchAll(/\.\w+\("([^"]+)", request\.(\w+)\.clone\(\)\)/g)]
        .map(([, name, field]) => [name, field]));
      generated.set(key, { accessor: resource, method, requestType, pathArgs, query });
    }
  }
  const mapping = {};
  const arms = [];
  const verbs = new Set(['get', 'post', 'delete', 'put', 'patch', 'head', 'options', 'trace']);
  for (const [path, item] of Object.entries(spec.paths)) {
    for (const [verb, operation] of Object.entries(item)) {
      if (!verbs.has(verb)) continue;
      const key = `${verb.toUpperCase()} ${path.replaceAll(/\{[^}]+\}/g, '{param}')}`;
      const entry = generated.get(key);
      if (!entry) throw new Error(`Missing Rust operation: ${key}`);
      generated.delete(key);
      const fields = types.get(entry.requestType) ?? [];
      const parameters = {};
      const pathNames = [...path.matchAll(/\{([^}]+)\}/g)].map((match) => match[1]);
      const declared = [...(item.parameters ?? []), ...(operation.parameters ?? [])];
      for (let parameter of declared) {
        if (parameter.$ref) parameter = spec.components.parameters[parameter.$ref.split('/').at(-1)];
        if (parameter.in === 'header') continue;
        const field = parameter.in === 'path'
          ? entry.pathArgs[pathNames.indexOf(parameter.name)]?.name : entry.query.get(parameter.name);
        if (!field || (parameter.in !== 'path' && !fields.some((item) => item.field === field && item.wire === parameter.name))) {
          throw new Error(`Missing Rust parameter: ${operation.operationId}.${parameter.name}`);
        }
        parameters[parameter.name] = field;
      }
      const body = operation.requestBody?.content?.['application/json']?.schema;
      for (const name of Object.keys(body?.properties ?? {})) {
        const field = fields.find((field) => field.wire === name)?.field;
        if (!field) throw new Error(`Missing Rust body property: ${operation.operationId}.${name}`);
        parameters[name] = field;
      }
      mapping[operation.operationId] = {
        accessor: entry.accessor, method: entry.method, requestType: entry.requestType,
        httpMethod: verb.toUpperCase(), path, parameters,
      };
      const setup = [];
      if (entry.requestType) {
        setup.push(`let mut request = ${entry.requestType}::default();`);
        for (const [wire, field] of Object.entries(parameters)) {
          if (!fields.some((item) => item.field === field)) continue;
          setup.push(`if let Some(value) = parameters.get(${JSON.stringify(wire)}) { request.${field} = serde_json::from_value(value.clone()).map_err(ApiError::Serialization)?; }`);
        }
      }
      const args = entry.pathArgs.map(({ name, type }, index) => type === '&str'
        ? `parameters[${JSON.stringify(pathNames[index])}].as_str().expect("string ${name}")`
        : `parameters[${JSON.stringify(pathNames[index])}].as_i64().expect("integer ${name}")`);
      if (entry.requestType) args.push('&request');
      args.push('options');
      const approval = operation.operationId.startsWith('data_exchange.approval_');
      const method = entry.method + (approval ? '_with_raw_response' : '');
      setup.push(`let response = client.${entry.accessor}.${method}(${args.join(', ')}).await?;`);
      if (approval) {
        setup.push('Ok(Observed { body: serde_json::to_value(response.body).map_err(ApiError::Serialization)?, status: Some(response.status_code), location: response.headers.get("location").map(|value| value.to_str().unwrap().to_string()) })');
      } else {
        setup.push('Ok(Observed { body: serde_json::to_value(response).map_err(ApiError::Serialization)?, status: None, location: None })');
      }
      arms.push(`        ${JSON.stringify(operation.operationId)} => {\n            ${setup.join('\n            ')}\n        }`);
    }
  }
  if (generated.size) throw new Error(`Unexpected Rust operations: ${[...generated.keys()]}`);
  writeFileSync(join(directory, 'sdk-map.json'), `${JSON.stringify(mapping, null, 2)}\n`);
  return `// Generated by operation-map.mjs. Do not edit. Calls compile in a separate consumer.
use cameronapak_platform_sdk::*;
use serde_json::Value;

pub struct Observed { pub body: Value, pub status: Option<u16>, pub location: Option<String> }

pub async fn invoke(client: &PlatformClient, operation: &str, parameters: &Value,
    options: Option<RequestOptions>) -> Result<Observed, ApiError> {
    match operation {
${arms.join(',\n')},
        _ => panic!("unmapped operation: {operation}"),
    }
}
`;
}
