<?php

namespace Cameronapak\PlatformSdk\Permissions\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1AppsPermissionsCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var array<'highlights'> $permissions Permissions the user has granted to the calling app.
     */
    #[JsonProperty('permissions'), ArrayType(['string'])]
    public array $permissions;

    /**
     * @param array{
     *   permissions: array<'highlights'>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->permissions = $values['permissions'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
