<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Chapters extends JsonSerializableType
{
    /**
     * @var ?array<ChaptersDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([ChaptersDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<ChaptersDataItem>,
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
