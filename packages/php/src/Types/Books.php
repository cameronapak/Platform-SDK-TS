<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Books extends JsonSerializableType
{
    /**
     * @var ?array<BooksDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([BooksDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<BooksDataItem>,
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
