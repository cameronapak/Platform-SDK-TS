<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class VerseOfTheDays extends JsonSerializableType
{
    /**
     * @var array<VerseOfTheDaysDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([VerseOfTheDaysDataItem::class])]
    public array $data;

    /**
     * @param array{
     *   data: array<VerseOfTheDaysDataItem>,
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
