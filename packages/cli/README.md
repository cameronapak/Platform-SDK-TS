# Platform CLI

An experimental, unofficial terminal client for the YouVersion Platform API. This package is local and unpublished. It generates all 33 API commands over the TypeScript Platform SDK, including read/write classifications, help, and typed input constraints.

## Install locally

From the repository root, use Node.js 22 or newer and pnpm:

```sh
pnpm install --frozen-lockfile
pnpm build:all
mkdir -p /tmp/yvp-pack
pnpm --config.ignore-scripts=true pack --pack-destination /tmp/yvp-pack
pnpm --dir packages/cli --config.ignore-scripts=true pack --pack-destination /tmp/yvp-pack
```

In a separate consumer directory, install both artifacts:

```sh
npm init -y
npm install --ignore-scripts /tmp/yvp-pack/cameronapak-platform-sdk-0.1.0.tgz /tmp/yvp-pack/cameronapak-platform-cli-0.1.0.tgz
./node_modules/.bin/yvp --help
./node_modules/.bin/yvp catalog
./node_modules/.bin/yvp bibles collection-get --help
```

The following examples use `yvp` for that installed executable.

## Credentials and inputs

Supply credentials through your environment or secret manager, never command-line arguments:

| Environment input | Purpose |
| --- | --- |
| `YOUVERSION_APP_KEY` | Required for every API execution. |
| `YOUVERSION_ACCESS_TOKEN` | OAuth access token for user-authorized operations. |
| `YOUVERSION_DATA_EXCHANGE_TOKEN` | Data exchange approval credential. |
| `YOUVERSION_DATA_EXCHANGE_APP_KEY` | Optional, distinct app-key query context on approval operations. |

Help, catalog, and version need no credentials and make no requests. Ordinary app-key operations do not send an unrelated OAuth access token. Approval GET requires the data exchange token. Approval POST uses the only locally present credential; if both tokens exist, select `--auth-mode exchange-token` or `--auth-mode oauth`. The CLI never falls back to another credential after rejection.

```sh
yvp bibles collection-get --language-ranges en --language-ranges fr --page-size 7
yvp languages collection-get --accept-language es-MX --page-size 5
yvp fonts stylesheet-get --font-id 42 --output text
```

Repeat array flags without comma splitting. Booleans take explicit `true` or `false`. Quote the wildcard `'*'`. `--format` selects an API representation; `--output` selects CLI formatting. Command help includes schema constraints and prose-only server restrictions.

If a command has a JSON body, select exactly one of `--body-file <path>` or `--body-stdin`. Only declared fields are accepted, including nested objects. The CLI inserts no defaults, examples, or request IDs. For example, a highlight body file contains a caller-chosen UUID:

```json
{
  "request_id": "20000000-0000-4000-8000-000000000001",
  "highlight": { "bible_id": 206, "passage_id": "PSA.23.4", "color": "12abef" }
}
```

```sh
yvp highlights collection-post --body-file highlight.json --yes
```

## Consent, disclosure, and failures

Writes prompt only when stdin and stderr are interactive. Refusal is the default. Automation and writes using `--body-stdin` require `--yes`. Validation happens before confirmation, and help never executes a write.

`--yes` does not permit sensitive output. Highlights, user permissions, approval HTML, and callback Location are metadata-only unless you select `--show-sensitive`. Token issuance requires both write consent and `--show-sensitive` before dispatch. Even with disclosure selected, the CLI guards configured credential values in output keys, values, text, and URLs, including common URI/JSON escaping. It does not recognize arbitrary unknown credential transformations.

Stdout defaults to one JSON envelope with `operationId`, actual HTTP `status`, `kind` (`json`, `text`, or `empty`), and `data`. Empty or withheld data is `null`; withheld results include `redacted: true`. Approval output includes `location`, withheld as `null` by default. `--output text` emits guarded raw text only for ordinary text endpoints, not approval or JSON endpoints. Failures leave stdout empty and report safe diagnostics on stderr without server bodies or stacks.

Approval callbacks are never followed, and the CLI never opens a browser. Approval exit zero means the request completed, not that the user granted permissions. Ordinary SDK redirects retain SDK behavior.

The default execution deadline is 30 seconds. Set `--timeout-seconds` to a finite positive value up to 300. The deadline starts after validation and consent and includes response-body consumption. The CLI and SDK do not retry. If a write is interrupted after dispatch without a definitive result, its outcome is unknown. Check its outcome before retrying. A received result with output-delivery failure is reported separately; one-time token delivery cannot be guaranteed.

Exit codes: `0` success, `1` request/deadline/delivery failure, `2` invalid input or refusal, `130` SIGINT, and `143` SIGTERM. Synthetic SDK abort status 499 is not reported as an API status.

If you use `--base-url`, choose a trusted HTTPS or loopback HTTP destination with no URL credentials, query, or fragment. This setting does not restrict ordinary SDK redirect chains.

## Develop and verify

From the repository root:

```sh
pnpm generate:cli
pnpm check:cli
pnpm check
```

The generator joins authoritative OpenAPI operations with SDK bindings, emits typed calls, and rejects unsupported schemas, inventory drift, and name collisions. If the contract fingerprint changes, review OpenAPI, SDK bindings, and policy first, then explicitly run `pnpm generate:cli --approve-contract`. Normal regeneration cannot approve a changed contract. Installed-consumer tests pack both packages, run the installed binary with fake credentials, and use loopback fixtures. No live API tests or credential persistence are included.
