<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BibleIndex extends JsonSerializableType
{
    /**
     * @var ?string $textDirection
     */
    #[JsonProperty('text_direction')]
    public ?string $textDirection;

    /**
     * @var ?array<BibleIndexBooksItem> $books
     */
    #[JsonProperty('books'), ArrayType([BibleIndexBooksItem::class])]
    public ?array $books;

    /**
     * @param array{
     *   textDirection?: ?string,
     *   books?: ?array<BibleIndexBooksItem>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->textDirection = $values['textDirection'] ?? null;
        $this->books = $values['books'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
