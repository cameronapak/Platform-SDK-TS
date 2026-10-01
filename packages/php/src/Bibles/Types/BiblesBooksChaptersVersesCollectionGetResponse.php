<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesBooksChaptersVersesCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<BiblesBooksChaptersVersesCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([BiblesBooksChaptersVersesCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<BiblesBooksChaptersVersesCollectionGetResponseDataItem>,
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
