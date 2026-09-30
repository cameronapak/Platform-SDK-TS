import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareOpenapi } from './generation.mjs';
import { adaptPython } from '../packages/python/adapt.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const output = join(root, '.cache', 'python-generated');
const staged = join(root, '.cache', 'python-adapted');
const target = join(root, 'packages', 'python', 'src', 'cameronapak_platform_sdk');
const { version } = JSON.parse(readFileSync(join(root, 'fern', 'fern.config.json'), 'utf8'));
const check = process.argv.includes('--check');

prepareOpenapi(join(root, 'openapi', 'openapi.json'), join(root, '.cache', 'openapi.fern.json'));
rmSync(output, { recursive: true, force: true });
execFileSync('npx', ['--yes', `fern-api@${version}`, 'generate', '--group', 'python-sdk', '--local', '--no-prompt', '--force', '--log-level', 'info'], { cwd: root, stdio: 'inherit' });
if (!existsSync(join(output, 'client.py'))) throw new Error('Python generator produced no client');
rmSync(staged, { recursive: true, force: true });
cpSync(output, staged, {
  recursive: true,
  filter: (path) => {
    const name = relative(output, path);
    return !name.startsWith('.') && !name.startsWith('tests') && (statSync(path).isDirectory() || path.endsWith('.py'));
  },
});
adaptPython(staged);
execFileSync('python3', [join(root, 'packages', 'python', 'operation-map.py'), staged, join(root, 'openapi', 'openapi.json')], { cwd: root, stdio: 'inherit' });
writeFileSync(join(staged, 'py.typed'), '');

function files(directory, prefix = '') {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(prefix, entry.name);
    return entry.isDirectory() ? files(join(directory, entry.name), path) : [path];
  }).sort();
}
if (check) {
  const expected = files(staged);
  const actual = existsSync(target) ? files(target).filter((file) => !file.includes('__pycache__')) : [];
  if (JSON.stringify(expected) !== JSON.stringify(actual) || expected.some((file) => !readFileSync(join(staged, file)).equals(readFileSync(join(target, file))))) {
    throw new Error('Python generated source is stale; run pnpm generate:python');
  }
  console.log('Python generated source and operation inventory are current');
} else {
  mkdirSync(dirname(target), { recursive: true });
  rmSync(target, { recursive: true, force: true });
  cpSync(staged, target, { recursive: true });
  console.log('Generated Python SDK with verified OpenAPI operation inventory');
}
