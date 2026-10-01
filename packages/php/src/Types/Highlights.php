<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Highlights extends JsonSerializableType
{
    /**
     * @var ?array<HighlightsDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([HighlightsDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<HighlightsDataItem>,
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
