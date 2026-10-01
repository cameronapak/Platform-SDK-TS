<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesIndexCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?string $textDirection
     */
    #[JsonProperty('text_direction')]
    public ?string $textDirection;

    /**
     * @var ?array<BiblesIndexCollectionGetResponseBooksItem> $books
     */
    #[JsonProperty('books'), ArrayType([BiblesIndexCollectionGetResponseBooksItem::class])]
    public ?array $books;

    /**
     * @param array{
     *   textDirection?: ?string,
     *   books?: ?array<BiblesIndexCollectionGetResponseBooksItem>,
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
