<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Fonts extends JsonSerializableType
{
    /**
     * @var ?array<FontsDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([FontsDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<FontsDataItem>,
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
