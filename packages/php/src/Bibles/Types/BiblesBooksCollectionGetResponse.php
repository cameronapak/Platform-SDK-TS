<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesBooksCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<BiblesBooksCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([BiblesBooksCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<BiblesBooksCollectionGetResponseDataItem>,
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
