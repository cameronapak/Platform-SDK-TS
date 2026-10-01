import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { cpSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkWire } from './test/wire.mjs';

const source = dirname(fileURLToPath(import.meta.url));
const temporary = mkdtempSync(join(tmpdir(), 'platform-sdk-php-consumer-'));
const consumer = join(temporary, 'consumer');
function run(command, args, cwd = consumer) {
  console.log('+', command, ...args);
  return execFileSync(command, args, { cwd, stdio: 'inherit' });
}
function phpFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? phpFiles(join(directory, entry.name)) : entry.name.endsWith('.php') ? [join(directory, entry.name)] : []);
}
try {
  for (const file of phpFiles(join(source, 'src'))) execFileSync('php', ['-l', file], { stdio: 'pipe' });
  console.log('PHP generated syntax checks passed');
  // The explicit version is required for unpublished artifact consumption.
  // Composer warns that published Packagist libraries should omit it instead.
  run('composer', ['validate', '--no-check-publish', '--no-interaction'], source);
  cpSync(join(source, 'test/consumer'), consumer, { recursive: true });
  mkdirSync(join(consumer, 'artifacts'));
  run('composer', ['archive', '--format=zip', `--dir=${join(consumer, 'artifacts')}`, '--file=platform-sdk', '--no-interaction'], source);
  const archive = join(consumer, 'artifacts/platform-sdk.zip');
  const files = execFileSync('unzip', ['-Z1', archive], { encoding: 'utf8' }).trim().split('\n').filter((file) => !file.endsWith('/'));
  assert(files.every((file) => /^src\/.+\.php$/.test(file) || ['composer.json', 'LICENSE', 'README.md', 'sdk-map.json'].includes(file)), 'PHP archive contains maintainer files');
  for (const required of ['composer.json', 'LICENSE', 'README.md', 'sdk-map.json', 'src/PlatformClient.php']) assert(files.includes(required), `PHP archive lacks ${required}`);
  const bytes = readFileSync(archive);
  const digest = createHash('sha256').update(bytes).digest('hex');
  // Immutable artifact URLs avoid Composer reusing an older same-version archive.
  renameSync(archive, join(consumer, `artifacts/platform-sdk-${digest}.zip`));
  const oldLock = JSON.parse(readFileSync(join(consumer, 'composer.lock'), 'utf8'));
  run('composer', ['update', 'cameronapak/platform-sdk', '--no-install', '--no-interaction', '--no-plugins', '--no-scripts']);
  const newLock = JSON.parse(readFileSync(join(consumer, 'composer.lock'), 'utf8'));
  const others = (lock) => [...lock.packages, ...lock['packages-dev']].filter((entry) => entry.name !== 'cameronapak/platform-sdk');
  assert.deepEqual(others(newLock), others(oldLock), 'PHP consumer updated unrelated dependencies');
  const installed = newLock.packages.find((entry) => entry.name === 'cameronapak/platform-sdk');
  assert.equal(installed.dist.url, `artifacts/platform-sdk-${digest}.zip`);
  assert.equal(installed.dist.shasum, createHash('sha1').update(bytes).digest('hex'));
  run('composer', ['install', '--no-interaction', '--no-plugins', '--no-scripts', '--prefer-dist']);
  run('composer', ['check-platform-reqs', '--no-interaction']);
  run('php', ['vendor/bin/phpstan', 'analyze', 'typing.php', '--level=6', '--no-progress']);
  run('php', ['runtime.php']);
  await checkWire(consumer);
  console.log('PHP Composer artifact, typing, native runtime, all operations, and shared wire checks passed');
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
