import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareOpenapi } from './generation.mjs';
import { adaptGo } from '../packages/go/adapt.mjs';
import { writeOperationMap } from '../packages/go/operation-map.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const output = join(root, '.cache/go-generated');
const staged = join(root, '.cache/go-adapted');
const target = join(root, 'packages/go');
const { version } = JSON.parse(readFileSync(join(root, 'fern/fern.config.json'), 'utf8'));

prepareOpenapi(join(root, 'openapi/openapi.json'), join(root, '.cache/openapi.fern.json'));
rmSync(output, { recursive: true, force: true });
execFileSync('npx', ['--yes', `fern-api@${version}`, 'generate', '--group', 'go-sdk', '--local', '--no-prompt', '--force', '--log-level', 'info'], { cwd: root, stdio: 'inherit' });
if (!existsSync(join(output, 'client/client.go')) || !existsSync(join(output, 'go.mod'))) {
  throw new Error('Go generator produced no client or module');
}
rmSync(staged, { recursive: true, force: true });
cpSync(output, staged, {
  recursive: true,
  filter: (path) => {
    const name = relative(output, path);
    if (name.startsWith('.') || name.startsWith('wiremock') || name.includes('_test')) return false;
    return statSync(path).isDirectory() || path.endsWith('.go') || ['go.mod', 'go.sum'].includes(name);
  },
});
adaptGo(staged);
writeOperationMap(staged, join(root, 'openapi/openapi.json'));
cpSync(join(root, 'LICENSE'), join(staged, 'LICENSE'));
execFileSync('gofmt', ['-w', staged], { cwd: root, stdio: 'inherit' });
execFileSync('go', ['mod', 'tidy'], { cwd: staged, stdio: 'inherit', env: { ...process.env, GOWORK: 'off' } });

function generatedFiles(directory, prefix = '') {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(prefix, entry.name);
    if (entry.name === 'test') return [];
    if (entry.isDirectory()) return generatedFiles(join(directory, entry.name), path);
    return entry.name.endsWith('.go') || ['go.mod', 'go.sum', 'sdk-map.json', 'LICENSE'].includes(path) ? [path] : [];
  }).sort();
}
const expected = generatedFiles(staged);
const actual = generatedFiles(target);
if (process.argv.includes('--check')) {
  if (JSON.stringify(expected) !== JSON.stringify(actual) || expected.some((file) => !readFileSync(join(staged, file)).equals(readFileSync(join(target, file))))) {
    throw new Error('Go generated source is stale; run pnpm generate:go');
  }
  console.log('Go generated source and operation inventory are current');
} else {
  for (const file of actual) rmSync(join(target, file));
  for (const file of expected) {
    mkdirSync(dirname(join(target, file)), { recursive: true });
    cpSync(join(staged, file), join(target, file));
  }
  console.log('Generated Go SDK with verified OpenAPI operation inventory');
}
