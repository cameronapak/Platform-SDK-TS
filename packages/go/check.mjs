import { execFileSync } from 'node:child_process';
import { cpSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const source = dirname(fileURLToPath(import.meta.url));
const root = resolve(source, '../..');
const temporary = mkdtempSync(join(tmpdir(), 'platform-sdk-go-consumer-'));
const sdk = join(temporary, 'sdk');
const consumer = join(temporary, 'consumer');
const env = { ...process.env, GOWORK: 'off', SDK_PACKAGE_DIR: sdk, SDK_REPOSITORY: root };
function run(command, args, cwd = sdk) {
  console.log('+', command, ...args);
  return execFileSync(command, args, { cwd, env, stdio: 'inherit' });
}
try {
  cpSync(source, sdk, { recursive: true, filter: (path) => path !== join(source, 'test') && !path.endsWith('.mjs') });
  cpSync(join(source, 'test'), consumer, { recursive: true, filter: (path) => path !== join(source, 'test/internal') });
  cpSync(join(source, 'test/internal'), join(sdk, 'internal'), { recursive: true });
  const formatting = execFileSync('gofmt', ['-l', sdk, consumer], { env, encoding: 'utf8' });
  if (formatting.trim()) throw new Error(`Go source needs formatting:\n${formatting}`);
  const module = readFileSync(join(sdk, 'go.mod'));
  const sums = readFileSync(join(sdk, 'go.sum'));
  run('go', ['mod', 'tidy']);
  if (!module.equals(readFileSync(join(sdk, 'go.mod'))) || !sums.equals(readFileSync(join(sdk, 'go.sum')))) {
    throw new Error('Go module metadata is stale');
  }
  run('go', ['mod', 'verify']);
  run('go', ['build', './...']);
  run('go', ['vet', './...']);
  run('go', ['test', '-race', '-count=1', '-timeout=60s', '-v', './internal']);
  run('go', ['mod', 'edit', `-replace=github.com/cameronapak/Platform-SDK-TS/packages/go=${sdk}`], consumer);
  run('go', ['mod', 'tidy'], consumer);
  const installed = JSON.parse(execFileSync('go', ['list', '-m', '-json', 'github.com/cameronapak/Platform-SDK-TS/packages/go'], { cwd: consumer, env, encoding: 'utf8' }));
  if (installed.Dir !== sdk) throw new Error('Consumer resolved the source checkout instead of the copied module');
  run('go', ['vet', './...'], consumer);
  run('go', ['test', '-race', '-count=1', '-timeout=60s', '-v', './...'], consumer);
  console.log('Clean Go consumer build, operation inventory, wire conformance, and race checks passed');
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
