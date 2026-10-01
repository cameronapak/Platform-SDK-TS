import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareOpenapi } from './generation.mjs';
import { adaptRust } from '../packages/rust/adapt.mjs';
import { writeOperationMap } from '../packages/rust/operation-map.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const output = join(root, '.cache/rust-generated');
const staged = join(root, '.cache/rust-adapted');
const target = join(root, 'packages/rust');
const { version } = JSON.parse(readFileSync(join(root, 'fern/fern.config.json'), 'utf8'));
const env = { ...process.env, CARGO_TARGET_DIR: join(root, '.cache/rust-target') };
function run(command, args) { execFileSync(command, args, { cwd: root, env, stdio: 'inherit' }); }

prepareOpenapi(join(root, 'openapi/openapi.json'), join(root, '.cache/openapi.fern.json'));
rmSync(output, { recursive: true, force: true });
run('npx', ['--yes', `fern-api@${version}`, 'generate', '--group', 'rust-sdk', '--local', '--no-prompt', '--force', '--log-level', 'info']);
if (!existsSync(join(output, 'src/lib.rs')) || !existsSync(join(output, 'Cargo.toml'))) throw new Error('Rust generator produced no crate');
rmSync(staged, { recursive: true, force: true });
mkdirSync(staged, { recursive: true });
cpSync(join(output, 'src'), join(staged, 'src'), { recursive: true });
cpSync(join(output, 'Cargo.toml'), join(staged, 'Cargo.toml'));
adaptRust(staged, join(root, 'openapi/openapi.json'));
const dispatch = join(root, '.cache/rust-consumer-dispatch.rs');
writeFileSync(dispatch, writeOperationMap(staged, join(root, 'openapi/openapi.json')));
cpSync(join(root, 'LICENSE'), join(staged, 'LICENSE'));
if (existsSync(join(target, 'Cargo.lock'))) cpSync(join(target, 'Cargo.lock'), join(staged, 'Cargo.lock'));
else run('cargo', ['generate-lockfile', '--manifest-path', join(staged, 'Cargo.toml')]);
// Reuse the checked-in dependency lock, rather than updating it on regeneration.
execFileSync('cargo', ['metadata', '--locked', '--format-version', '1', '--manifest-path', join(staged, 'Cargo.toml')], { cwd: root, env, stdio: ['ignore', 'ignore', 'inherit'] });
run('cargo', ['fmt', '--manifest-path', join(staged, 'Cargo.toml')]);
run('rustfmt', ['--edition', '2021', dispatch]);

function files(directory, prefix = '') {
  if (!existsSync(directory)) return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(prefix, entry.name);
    if (entry.isDirectory()) return path === 'src' || path.startsWith('src/') ? files(join(directory, entry.name), path) : [];
    return path.startsWith('src/') || ['Cargo.toml', 'Cargo.lock', 'sdk-map.json', 'LICENSE'].includes(path) ? [path] : [];
  }).sort();
}
const expected = files(staged);
const actual = files(target);
const dispatchTarget = join(target, 'test/consumer/src/dispatch.rs');
if (process.argv.includes('--check')) {
  if (JSON.stringify(expected) !== JSON.stringify(actual) || expected.some((file) => !readFileSync(join(staged, file)).equals(readFileSync(join(target, file)))) ||
    !existsSync(dispatchTarget) || !readFileSync(dispatch).equals(readFileSync(dispatchTarget))) {
    throw new Error('Rust generated source is stale; run pnpm generate:rust');
  }
  console.log('Rust generated source and operation inventory are current');
} else {
  for (const file of actual) rmSync(join(target, file));
  for (const file of expected) {
    mkdirSync(dirname(join(target, file)), { recursive: true });
    cpSync(join(staged, file), join(target, file));
  }
  mkdirSync(dirname(dispatchTarget), { recursive: true });
  cpSync(dispatch, dispatchTarget);
  console.log('Generated Rust SDK with verified OpenAPI operation inventory');
}
