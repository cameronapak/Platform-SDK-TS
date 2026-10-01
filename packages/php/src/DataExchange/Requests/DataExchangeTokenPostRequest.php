<?php

namespace Cameronapak\PlatformSdk\DataExchange\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class DataExchangeTokenPostRequest extends JsonSerializableType
{
    /**
     * @var ?string $xYvpAppKey Public app key used to resolve the app for direct browser flows.
     */
    public ?string $xYvpAppKey;

    /**
     * @var ?string $xYvpAppId App identifier used when a public app key is not supplied.
     */
    public ?string $xYvpAppId;

    /**
     * @var array<'highlights'> $requestedPermissions Data exchange permissions the user should review in the browser flow.
     */
    #[JsonProperty('requested_permissions'), ArrayType(['string'])]
    public array $requestedPermissions;

    /**
     * @param array{
     *   requestedPermissions: array<'highlights'>,
     *   xYvpAppKey?: ?string,
     *   xYvpAppId?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->xYvpAppKey = $values['xYvpAppKey'] ?? null;
        $this->xYvpAppId = $values['xYvpAppId'] ?? null;
        $this->requestedPermissions = $values['requestedPermissions'];
    }
}
