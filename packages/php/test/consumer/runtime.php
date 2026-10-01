<?php

require __DIR__ . '/vendor/autoload.php';

use Cameronapak\PlatformSdk\PlatformClient;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\Core\Client\RetryDecoratingClient;
use Cameronapak\PlatformSdk\DataExchange\DataExchangeClient;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalGetRequest;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalPostRequest;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Fonts\FontsClient;
use Cameronapak\PlatformSdk\Licenses\Requests\V1LicensesCollectionGetRequest;
use Cameronapak\PlatformSdk\Licenses\Types\V1LicensesCollectionGetResponseDataItem;
use GuzzleHttp\Client;
use GuzzleHttp\Exception\ConnectException;
use GuzzleHttp\Handler\MockHandler;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use GuzzleHttp\Psr7\Request;
use GuzzleHttp\Psr7\Response;

function check(bool $condition, string $message): void
{
    if (!$condition) {
        throw new RuntimeException($message);
    }
}

$checks = 0;
foreach (['unset', 'client', 'request'] as $timeout) {
    foreach ([400, 401, 404, 422] as $status) {
        $history = [];
        $handler = HandlerStack::create(new MockHandler([new Response($status, ['X-Request-Id' => 'error-17'], 'fake-secret-in-body')]));
        $handler->push(Middleware::history($history));
        $client = new PlatformClient('fake-app-key', 'fake-oauth', ['client' => new Client(['handler' => $handler]), 'timeout' => $timeout === 'client' ? 1.0 : null, 'maxRetries' => 2]);
        try {
            $client->bibles->resourceGet(111, $timeout === 'request' ? ['timeout' => 1.0] : []);
            throw new RuntimeException('Expected permanent HTTP error');
        } catch (CameronapakApiException $error) {
            check($error->getCode() === $status && $error->getBody() === 'fake-secret-in-body', 'Lost HTTP error data');
            check($error->getHeaders()['x-request-id'][0] === 'error-17', 'Lost error headers');
            check(!str_contains($error->getMessage() . (string) $error, 'fake-secret'), 'Exception printed response secret');
        }
        check(count($history) === 1, 'Permanent status retried with timeout');
        $checks++;
    }
}

// Native retry timing is exercised with its existing sleep injection, not real waits.
foreach ([408, 429, 503] as $status) {
    $history = [];
    $handler = HandlerStack::create(new MockHandler([new Response($status), new Response(200)]));
    $handler->push(Middleware::history($history));
    $sleeps = [];
    $retry = new RetryDecoratingClient(new Client(['handler' => $handler]), 1, 100, function ($delay) use (&$sleeps) { $sleeps[] = $delay; });
    check($retry->send(new Request('GET', 'https://example.invalid'), 1)->getStatusCode() === 200, 'Retry failed to return success');
    check(count($history) === 2 && count($sleeps) === 1 && $sleeps[0] > 0, 'Incorrect native retry attempts');
    $checks++;
}

foreach ([0, 1] as $override) {
    $history = [];
    $handler = HandlerStack::create(new MockHandler([new Response(503), new Response(200, [], '{"id":111}')]));
    $handler->push(Middleware::history($history));
    $retry = new RetryDecoratingClient(new Client(['handler' => $handler]), 2, 1, static function () {});
    check($retry->send(new Request('GET', 'https://example.invalid'), maxRetries: $override)->getStatusCode() === ($override === 0 ? 503 : 200), 'Retry override was ignored');
    check(count($history) === $override + 1, 'Retry override attempt count');
    $checks++;
}

$history = [];
$handler = HandlerStack::create(new MockHandler([new Response(503), new Response(200, [], '{"id":111}')]));
$handler->push(Middleware::history($history));
$client = new PlatformClient('app', options: ['client' => new Client(['handler' => $handler]), 'maxRetries' => 2]);
try {
    $client->bibles->resourceGet(111, ['maxRetries' => 0]);
    throw new RuntimeException('Expected no-retry HTTP error');
} catch (CameronapakApiException $error) {
    check($error->getCode() === 503 && count($history) === 1, 'SDK request retry override was not forwarded');
}
$checks++;

$history = [];
$handler = HandlerStack::create(new MockHandler([new Response(200, [], 'stylesheet')]));
$handler->push(Middleware::history($history));
$authCalls = 0;
$raw = new RawClient(['client' => new Client(['handler' => $handler]), 'getAuthHeaders' => function () use (&$authCalls) {
    return ['X-YVP-App-Key' => 'app-' . ++$authCalls];
}]);
check((new FontsClient($raw))->v1FontsStylesheetGet(42) === 'stylesheet', 'Stylesheet response changed');
check($authCalls === 1, 'Stylesheet acquired credentials more than once');
check($history[0]['request']->getHeaderLine('X-YVP-App-Key') === 'app-1' && $history[0]['request']->getUri()->getQuery() === 'app_key=app-1', 'Stylesheet query and header credentials disagree');
$checks++;

$history = [];
$handler = HandlerStack::create(new MockHandler(array_fill(0, 4, new Response(303, ['Location' => 'https://callback.invalid'], 'Redirecting'))));
$handler->push(Middleware::history($history));
$authCalls = 0;
$raw = new RawClient(['client' => new Client(['handler' => $handler]), 'headers' => ['X-YVP-App-Key' => 'app', 'aUtHoRiZaTiOn' => 'Bearer leaked'], 'getAuthHeaders' => function () use (&$authCalls) {
    $authCalls++;
    return ['Authorization' => 'Bearer acquired'];
}]);
$approvals = new DataExchangeClient($raw);
$approvals->approvalGetWithResponse(new DataExchangeApprovalGetRequest(['token' => 'exchange']), ['headers' => ['AUTHORIZATION' => 'Bearer request-leak']]);
$approvals->approvalPostWithResponse(new DataExchangeApprovalPostRequest(['token' => '0']));
$approvals->approvalPostWithResponse(options: ['queryParameters' => ['token' => ''], 'headers' => ['authorization' => 'Bearer request-leak']]);
check($authCalls === 0, 'Suppressed approval acquired auth headers');
foreach ($history as $request) check(!$request['request']->hasHeader('Authorization'), 'Approval leaked Authorization');
$response = $approvals->approvalPostWithResponse();
check($authCalls === 1 && $history[3]['request']->getHeaderLine('Authorization') === 'Bearer acquired', 'Tokenless POST did not acquire auth');
check($response->getStatusCode() === 303 && $response->getHeaderLine('Location') === 'https://callback.invalid' && (string) $response->getBody() === 'Redirecting', 'Raw redirect lost metadata/body');
$checks++;

$exception = new ConnectException('failed https://example.invalid?app_key=fake-secret-network', new Request('GET', 'https://example.invalid'));
$client = new PlatformClient('fake-secret-app', options: ['client' => new Client(['handler' => new MockHandler([$exception])]), 'maxRetries' => 0]);
try {
    $client->fonts->v1FontsStylesheetGet(42);
    throw new RuntimeException('Expected transport exception');
} catch (CameronapakException $error) {
    check(!str_contains($error->getMessage() . (string) $error, 'fake-secret'), 'Transport exception printed credential URL');
    check($error->getPrevious() instanceof ConnectException, 'Transport exception lost explicit cause');
}
$checks++;

foreach (['.', '..'] as $segment) {
    try {
        $client->languages->v1LanguagesResourceGet($segment);
        throw new RuntimeException('Expected dot-segment rejection');
    } catch (InvalidArgumentException $error) {
        check($error->getMessage() === 'Path identifiers cannot be dot segments', 'Unexpected dot-segment error');
    }
    $checks++;
}

foreach (['fake-secret-app', 42] as $invalidDate) {
    $body = json_encode(['data' => [['id' => 9, 'agreed_dt' => $invalidDate]]], JSON_THROW_ON_ERROR);
    $client = new PlatformClient('fake-secret-app', options: ['client' => new Client(['handler' => new MockHandler([new Response(200, [], $body)])]), 'maxRetries' => 0]);
    try {
        $client->licenses->v1LicensesCollectionGet(new V1LicensesCollectionGetRequest(['bibleId' => 111, 'developerId' => 'developer-9']));
        throw new RuntimeException('Expected date deserialization failure');
    } catch (CameronapakException $error) {
        check($error->getMessage() === 'Failed to deserialize response', 'Deserialization error copied response data');
        check(!str_contains((string) $error, 'fake-secret'), 'Deserialization exception printed credential');
        check($error->getPrevious() instanceof JsonException, 'Deserialization exception lost explicit cause');
        if (is_string($invalidDate)) {
            check(str_contains($error->getPrevious()->getMessage(), $invalidDate), 'Explicit cause lost original date value');
        }
    }
    $checks++;
}

foreach ([
    ['id' => 9],
    ['id' => 10, 'agreed_dt' => null],
    ['id' => 11, 'agreed_dt' => '2026-09-30T20:17:09-04:30'],
] as $license) {
    $body = json_encode(['data' => [$license]], JSON_THROW_ON_ERROR);
    $client = new PlatformClient('app', options: ['client' => new Client(['handler' => new MockHandler([new Response(200, [], $body)])])]);
    $page = $client->licenses->v1LicensesCollectionGet(new V1LicensesCollectionGetRequest(['bibleId' => 111, 'developerId' => 'developer-9']));
    $item = $page?->data[0] ?? null;
    check($item instanceof V1LicensesCollectionGetResponseDataItem && $item->id === $license['id'], 'Lost typed license response');
    if (isset($license['agreed_dt'])) {
        check($item->agreedDt instanceof DateTime, 'License timestamp was not converted to DateTime');
        check($item->agreedDt->setTimezone(new DateTimeZone('UTC'))->format('c') === '2026-10-01T00:47:09+00:00', 'License timestamp lost time or offset');
    } else {
        check($item->agreedDt === null, 'Omitted or null license date was not preserved');
    }
    $checks++;
}

echo "PHP native runtime checks: $checks passed\n";
