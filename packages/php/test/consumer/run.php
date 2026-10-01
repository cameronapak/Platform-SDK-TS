<?php

require __DIR__ . '/vendor/autoload.php';

use Cameronapak\PlatformSdk\PlatformClient;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Composer\InstalledVersions;
use GuzzleHttp\Client;
use Psr\Http\Message\ResponseInterface;

set_error_handler(static function ($severity, $message, $file, $line) {
    throw new ErrorException($message, 0, $severity, $file, $line);
});

$installed = realpath(InstalledVersions::getInstallPath('cameronapak/platform-sdk'));
$classFile = (new ReflectionClass(PlatformClient::class))->getFileName();
if ($installed === false || !str_starts_with($classFile, $installed . '/') || !str_starts_with($installed, __DIR__ . '/vendor/')) {
    throw new RuntimeException('Consumer resolved the checkout instead of the installed artifact');
}
$map = json_decode(file_get_contents($installed . '/sdk-map.json'), true, flags: JSON_THROW_ON_ERROR);

// Parameters are constructed through generated public models, not handwritten HTTP.
function invokeOperation(PlatformClient $client, array $entry, array $values, array $options, bool $raw): mixed
{
    $arguments = [];
    $requestValues = [];
    foreach ($values as $wire => $value) {
        $field = $entry['parameters'][$wire];
        if (in_array($field, $entry['args'], true)) {
            $arguments[$field] = $value;
        } else {
            $requestValues[$field] = $value;
        }
    }
    if ($entry['requestType'] !== null) {
        $type = $entry['requestType'];
        foreach ($requestValues as $field => &$value) {
            $propertyType = (new ReflectionProperty($type, $field))->getType();
            if ($propertyType instanceof ReflectionNamedType && $propertyType->getName() === 'string' && is_numeric($value)) {
                $value = (string) $value;
            } elseif ($propertyType instanceof ReflectionNamedType && !$propertyType->isBuiltin() && is_array($value)) {
                $model = $propertyType->getName();
                $value = $model::fromJson(json_encode($value, JSON_THROW_ON_ERROR));
            }
        }
        unset($value);
        $arguments['request'] = new $type($requestValues);
    }
    $arguments['options'] = $options;
    $method = $entry['method'] . ($raw ? 'WithResponse' : '');
    return $client->{$entry['accessor']}->{$method}(...$arguments);
}

while (($line = fgets(STDIN)) !== false) {
    try {
        $job = json_decode($line, true, flags: JSON_THROW_ON_ERROR);
        $config = $job['client'];
        $options = ['baseUrl' => $job['baseUrl'], 'client' => new Client($config['transport'] ?? []), 'maxRetries' => $config['maxRetries'] ?? 0, 'headers' => $config['headers'] ?? []];
        if (isset($config['timeout'])) {
            $options['timeout'] = $config['timeout'];
        }
        $client = new PlatformClient($config['appKey'], $config['token'] ?? null, $options);
        $result = invokeOperation($client, $map[$job['operationId']], $job['parameters'], $job['options'] ?? [], $job['raw'] ?? false);
        if ($result instanceof ResponseInterface) {
            $result = ['status' => $result->getStatusCode(), 'headers' => $result->getHeaders(), 'text' => (string) $result->getBody()];
        }
        echo json_encode(['result' => $result], JSON_THROW_ON_ERROR) . "\n";
    } catch (Throwable $error) {
        $result = ['error' => get_class($error), 'message' => $error->getMessage(), 'display' => (string) $error, 'status' => $error->getCode()];
        if ($error instanceof CameronapakApiException) {
            $result['body'] = $error->getBody();
            if (method_exists($error, 'getHeaders')) {
                $result['headers'] = $error->getHeaders();
            }
        }
        echo json_encode($result, JSON_THROW_ON_ERROR) . "\n";
    }
}
