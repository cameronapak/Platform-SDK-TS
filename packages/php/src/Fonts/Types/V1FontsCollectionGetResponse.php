<?php

namespace Cameronapak\PlatformSdk\Fonts\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1FontsCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1FontsCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1FontsCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<V1FontsCollectionGetResponseDataItem>,
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
