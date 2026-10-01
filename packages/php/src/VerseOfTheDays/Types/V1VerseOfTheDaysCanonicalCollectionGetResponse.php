<?php

namespace Cameronapak\PlatformSdk\VerseOfTheDays\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1VerseOfTheDaysCanonicalCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var array<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem::class])]
    public array $data;

    /**
     * @param array{
     *   data: array<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->data = $values['data'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
