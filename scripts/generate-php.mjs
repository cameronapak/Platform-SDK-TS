import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareOpenapi } from './generation.mjs';
import { adaptPhp } from '../packages/php/adapt.mjs';
import { writeOperationMap } from '../packages/php/operation-map.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const output = join(root, '.cache/php-generated');
const staged = join(root, '.cache/php-adapted');
const target = join(root, 'packages/php');
const { version } = JSON.parse(readFileSync(join(root, 'fern/fern.config.json'), 'utf8'));
prepareOpenapi(join(root, 'openapi/openapi.json'), join(root, '.cache/openapi.fern.json'));
rmSync(output, { recursive: true, force: true });
execFileSync('npx', ['--yes', `fern-api@${version}`, 'generate', '--group', 'php-sdk', '--local', '--no-prompt', '--force', '--log-level', 'info'], { cwd: root, stdio: 'inherit' });
if (!existsSync(join(output, 'src/PlatformClient.php'))) throw new Error('PHP generator produced no client');
rmSync(staged, { recursive: true, force: true });
mkdirSync(staged, { recursive: true });
cpSync(join(output, 'src'), join(staged, 'src'), { recursive: true, filter: (path) => !path.split('/').at(-1).startsWith('.') });
cpSync(join(output, 'composer.json'), join(staged, 'composer.json'));
// Inventory observes native paths before percent-encoding adaptations.
writeOperationMap(staged, join(root, 'openapi/openapi.json'));
adaptPhp(staged);
cpSync(join(root, 'LICENSE'), join(staged, 'LICENSE'));

function files(directory, prefix = '') {
  if (!existsSync(directory)) return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(prefix, entry.name);
    return entry.isDirectory() ? (path === 'src' || path.startsWith('src/') ? files(join(directory, entry.name), path) : [])
      : (path.startsWith('src/') || ['composer.json', 'sdk-map.json', 'LICENSE'].includes(path) ? [path] : []);
  }).sort();
}
const expected = files(staged);
const actual = files(target);
if (process.argv.includes('--check')) {
  if (JSON.stringify(expected) !== JSON.stringify(actual) || expected.some((file) => !readFileSync(join(staged, file)).equals(readFileSync(join(target, file))))) {
    throw new Error('PHP generated source is stale; run pnpm generate:php');
  }
  console.log('PHP generated source and operation inventory are current');
} else {
  for (const file of actual) rmSync(join(target, file));
  for (const file of expected) {
    mkdirSync(dirname(join(target, file)), { recursive: true });
    cpSync(join(staged, file), join(target, file));
  }
  console.log('Generated PHP SDK with verified OpenAPI operation inventory');
}
