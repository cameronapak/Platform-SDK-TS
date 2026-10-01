<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class DataExchangeTokenCreate extends JsonSerializableType
{
    /**
     * @var array<'highlights'> $requestedPermissions Data exchange permissions the user should review in the browser flow.
     */
    #[JsonProperty('requested_permissions'), ArrayType(['string'])]
    public array $requestedPermissions;

    /**
     * @param array{
     *   requestedPermissions: array<'highlights'>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->requestedPermissions = $values['requestedPermissions'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
