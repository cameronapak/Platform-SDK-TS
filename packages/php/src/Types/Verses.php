<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Verses extends JsonSerializableType
{
    /**
     * @var ?array<VersesDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([VersesDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<VersesDataItem>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->data = $values['data'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
