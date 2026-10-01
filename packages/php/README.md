# PHP Platform SDK

An experimental, unofficial, unpublished PHP client for the YouVersion Platform API.
This is a personal Forge experiment, not an official or supported YouVersion SDK.

## Install a local artifact

Use PHP 8.2 or newer, Composer, and a PSR-18 HTTP client and PSR-17 factories.
Guzzle 7 is the verified transport. No package is published to Packagist.

From the repository root, create an archive:

```sh
mkdir -p .cache/php-artifacts
composer archive --working-dir=packages/php --format=zip --dir=../../.cache/php-artifacts
```

In your application, add an artifact repository pointing to that directory and
install the SDK and an HTTP implementation:

```sh
composer config repositories.platform-sdk artifact /absolute/path/to/.cache/php-artifacts
composer require cameronapak/platform-sdk:0.1.0 guzzlehttp/guzzle:^7.15.2
```

If you rebuild version `0.1.0`, use a new archive filename and refresh its lock entry.
The repository's consumer checker uses content-addressed filenames to avoid stale
Composer downloads. You need `ext-zip` to use Composer artifact repositories.

## Call the API

```php
use Cameronapak\PlatformSdk\PlatformClient;
use Cameronapak\PlatformSdk\Bibles\Requests\BiblesCollectionGetRequest;
use GuzzleHttp\Client;

$platform = new PlatformClient(
    yvpAppKey: getenv('YOUVERSION_APP_KEY'),
    options: ['client' => new Client(), 'timeout' => 10.0, 'maxRetries' => 2],
);
$page = $platform->bibles->collectionGet(new BiblesCollectionGetRequest([
    'languageRangesArray' => ['en', 'es-419'],
    'pageSize' => 7,
]));
```

Pass `token` when an operation acts on user data. User operations require the app
key and OAuth access token together. Request options override client timeouts,
retry counts, and headers without discarding other client headers:

```php
$platform = new PlatformClient(
    yvpAppKey: getenv('YOUVERSION_APP_KEY'),
    token: getenv('YOUVERSION_ACCESS_TOKEN'),
    options: ['client' => new Client(), 'headers' => ['Accept-Language' => 'fr-CA']],
);
$bible = $platform->bibles->resourceGet(111, ['maxRetries' => 0]);
```

Pagination is explicit. Read `nextPageToken` and pass it as `pageToken` on the
next collection request. Mixed numeric/wildcard page sizes accept integers or
strings. The API owns numeric limits and the conditions for using `'*'`.
Responses use generated public properties. JSON collection responses can be
`null` when the server sends no body; required empty text stays `''`.

## Inspect approval redirects and errors

```php
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalPostRequest;

$response = $platform->dataExchange->approvalPostWithResponse(
    new DataExchangeApprovalPostRequest(['token' => $exchangeToken]),
);
$status = $response->getStatusCode();
$callback = $response->getHeaderLine('Location');
```

`approvalGetWithResponse` and `approvalPostWithResponse` return native PSR response
objects without following callbacks. The ordinary methods preserve generated
text and void returns. Approval GET and POST with an exchange token suppress
SDK-owned Authorization headers; tokenless POST uses configured OAuth.

`CameronapakApiException` exposes status through `getCode()`, the original body
through `getBody()`, and lowercase response-header keys through `getHeaders()`.
Messages and automatic string conversion omit response bodies and transport
URLs. Explicit body, header, response, and `getPrevious()` access can contain
sensitive data; do not log them without application-level redaction.

## Transport boundary

The SDK accepts a PSR-18 client through `options['client']` and discovers PSR-17
factories. Guzzle requests disable redirect following and HTTP-error exceptions
even when timeout overrides are supplied. This lets the generated runtime own
error classification and retries.

Timeout overrides use Fern's native Guzzle/Symfony support. Other PSR-18 clients
receive a warning and must enforce timeouts themselves. Only Guzzle is verified
here. Configure HTTPS certificates, proxies, and connection settings on your
transport. Do not inject app credentials in transport middleware or default
headers: the SDK cannot suppress credentials added after request construction.

Retries preserve Fern's backoff and retry-status policy. Set client or per-request
`maxRetries` to `0` when your application owns retries. String path identifiers
are encoded as one segment; standalone `.` and `..` are rejected.

## Regenerate and check

```sh
pnpm generate:php
pnpm check:php-generated
pnpm check:php
```

Generation requires the repository's pinned Fern CLI and a running Docker daemon.
Checks require PHP, Composer, `ext-zip`, and `unzip`. Generated formatting remains
owned by the pinned generator and is checked through exact regeneration.

The checker installs an archive in a temporary Composer consumer, verifies the
installed source location and locked dependencies, runs PHPStan on representative
typed calls, and executes all shared wire cases plus PHP runtime regressions.
Tests use fake credentials and loopback HTTP servers, not the live Platform API.
