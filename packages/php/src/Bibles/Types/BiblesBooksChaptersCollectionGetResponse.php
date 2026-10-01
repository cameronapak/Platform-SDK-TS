<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesBooksChaptersCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<BiblesBooksChaptersCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([BiblesBooksChaptersCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<BiblesBooksChaptersCollectionGetResponseDataItem>,
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
