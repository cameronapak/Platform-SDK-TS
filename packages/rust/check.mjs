import { execFileSync } from 'node:child_process';
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const source = dirname(fileURLToPath(import.meta.url));
const root = resolve(source, '../..');
const temporary = mkdtempSync(join(tmpdir(), 'platform-sdk-rust-consumer-'));
const sdk = join(temporary, 'cameronapak_platform_sdk-0.1.0');
const consumer = join(temporary, 'consumer');
const target = join(root, '.cache/rust-target');
const env = { ...process.env, CARGO_TARGET_DIR: target, SDK_REPOSITORY: root, SDK_PACKAGE_DIR: sdk,
  SDK_TLS_CERT: join(temporary, 'cert.pem'), SDK_TLS_KEY: join(temporary, 'key.pem'), SDK_RETRY_TEST: join(source, 'test/retry.rs') };
function run(command, args, cwd = source) {
  console.log('+', command, ...args);
  return execFileSync(command, args, { cwd, env, stdio: 'inherit' });
}
try {
  run('cargo', ['fmt', '--', '--check']);
  // Fern's utility test data trips Clippy's approx_constant deny lint.
  // Lint the shipped library; run all utility tests below without editing fixtures.
  run('cargo', ['clippy', '--locked', '--lib']);
  run('cargo', ['doc', '--locked', '--no-deps']);
  run('cargo', ['package', '--locked', '--allow-dirty']);
  const artifact = join(target, 'package/cameronapak_platform_sdk-0.1.0.crate');
  const files = execFileSync('tar', ['-tzf', artifact], { encoding: 'utf8' }).trim().split('\n');
  if (files.some((file) => /\/(test|target|\.cache)\/|\.mjs$/.test(file)) || !files.some((file) => file.endsWith('/sdk-map.json'))) {
    throw new Error('Rust artifact contains maintainer files or lacks its operation inventory');
  }
  run('tar', ['-xzf', artifact, '-C', temporary]);
  // Compile and exercise the extracted artifact, not a path to the source checkout.
  const httpClient = join(sdk, 'src/core/http_client.rs');
  const shippedHttpClient = readFileSync(httpClient, 'utf8');
  writeFileSync(httpClient, shippedHttpClient + '\n#[cfg(test)] mod platform_retry_tests { use super::*; include!(env!("SDK_RETRY_TEST")); }\n');
  run('cargo', ['test', '--locked', '--lib'], sdk);
  // The external consumer receives the original artifact, without the private test seam.
  writeFileSync(httpClient, shippedHttpClient);
  cpSync(join(source, 'test/consumer'), consumer, { recursive: true, filter: (path) => path !== join(source, 'test/consumer/target') });
  const manifest = join(consumer, 'Cargo.toml');
  const original = readFileSync(manifest, 'utf8');
  if (original.split('path = "../.."').length !== 2) throw new Error('Rust consumer dependency changed');
  writeFileSync(manifest, original.replace('path = "../.."', 'path = "../cameronapak_platform_sdk-0.1.0"'));
  const metadata = JSON.parse(execFileSync('cargo', ['metadata', '--locked', '--format-version', '1'], { cwd: consumer, env, encoding: 'utf8' }));
  const installed = metadata.packages.find((item) => item.name === 'cameronapak_platform_sdk');
  if (installed?.manifest_path !== join(sdk, 'Cargo.toml')) throw new Error('Rust consumer resolved the checkout instead of the artifact');
  run('cargo', ['fmt', '--', '--check'], consumer);
  execFileSync('openssl', ['req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-keyout', env.SDK_TLS_KEY, '-out', env.SDK_TLS_CERT,
    '-days', '1', '-subj', '/CN=127.0.0.1', '-addext', 'subjectAltName=IP:127.0.0.1', '-addext', 'basicConstraints=critical,CA:FALSE'], { env, stdio: 'ignore' });
  run('cargo', ['clippy', '--locked', '--all-targets'], consumer);
  run('cargo', ['test', '--locked', '--lib', '--', '--nocapture'], consumer);
  run('cargo', ['test', '--locked', '--doc'], consumer);
  console.log('Rust crate artifact, all operations, shared wire conformance, TLS, cancellation, and consumer documentation checks passed');
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
