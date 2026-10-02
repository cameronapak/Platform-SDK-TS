import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { cp, mkdir, mkdtemp, readFile, rm, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { promisify } from 'node:util';
import test from 'node:test';

const exec = promisify(execFile);
const root = resolve('.');

test('CLI generation is deterministic and rejects unreviewed drift', async (t) => {
  const temporary = await mkdtemp(join(tmpdir(), 'platform-cli-generation-'));
  t.after(() => rm(temporary, { recursive: true, force: true }));
  for (const path of ['scripts/generate-cli.mjs', 'openapi/openapi.json', 'src/generated',
    'packages/cli/package.json', 'packages/cli/contract.sha256', 'packages/cli/src/generated.ts']) {
    await mkdir(dirname(join(temporary, path)), { recursive: true });
    await cp(join(root, path), join(temporary, path), { recursive: true });
  }
  await symlink(join(root, 'node_modules'), join(temporary, 'node_modules'), 'dir');
  await symlink(join(root, 'packages/cli/node_modules'), join(temporary, 'packages/cli/node_modules'), 'dir');
  const generate = (args = []) => exec(process.execPath, [join(temporary, 'scripts/generate-cli.mjs'), ...args]);
  const output = join(temporary, 'packages/cli/src/generated.ts');
  const approved = await readFile(join(temporary, 'packages/cli/contract.sha256'), 'utf8');
  const original = await readFile(output, 'utf8');
  await generate();
  assert.equal(await readFile(output, 'utf8'), original);
  await generate();
  assert.equal(await readFile(output, 'utf8'), original);
  assert.match((await generate(['--check'])).stdout, /33 operations/);

  await writeFile(output, `${original}// stale\n`);
  await assert.rejects(generate(['--check']), (error) => /generated commands are stale/.test(error.stderr));
  await generate();

  const specPath = join(temporary, 'openapi/openapi.json');
  const specification = await readFile(specPath, 'utf8');
  const spec = JSON.parse(specification);
  spec.paths['/v1/fonts'].get.description = 'Changed source requires policy review.';
  await writeFile(specPath, JSON.stringify(spec));
  await assert.rejects(generate(), (error) => /contract changed/.test(error.stderr));
  assert.equal(await readFile(join(temporary, 'packages/cli/contract.sha256'), 'utf8'), approved);
  assert.equal(await readFile(output, 'utf8'), original);
  await writeFile(specPath, specification);

  const bible = spec.paths['/v1/bibles'].get;
  const parameter = bible.parameters.find((entry) => entry.name === 'page_size');
  parameter.schema.unsupportedValidation = true;
  await writeFile(specPath, JSON.stringify(spec));
  await assert.rejects(generate(['--approve-contract']), (error) => /unsupported schema keyword/.test(error.stderr));
  await writeFile(specPath, specification);

  const mapPath = join(temporary, 'src/generated/sdk-map.json');
  const map = JSON.parse(await readFile(mapPath, 'utf8'));
  map['bibles.collection_get'].path = '/wrong';
  await writeFile(mapPath, JSON.stringify(map));
  await assert.rejects(generate(), (error) => /method\/path mismatch/.test(error.stderr));
});
