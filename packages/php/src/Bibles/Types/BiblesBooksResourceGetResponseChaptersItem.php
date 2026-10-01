<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesBooksResourceGetResponseChaptersItem extends JsonSerializableType
{
    /**
     * @var ?string $id Chapter identifier
     */
    #[JsonProperty('id')]
    public ?string $id;

    /**
     * @var ?string $passageId Passage identifier
     */
    #[JsonProperty('passage_id')]
    public ?string $passageId;

    /**
     * @var ?string $title Chapter title
     */
    #[JsonProperty('title')]
    public ?string $title;

    /**
     * @var ?array<BiblesBooksResourceGetResponseChaptersItemVersesItem> $verses Verses in the chapter
     */
    #[JsonProperty('verses'), ArrayType([BiblesBooksResourceGetResponseChaptersItemVersesItem::class])]
    public ?array $verses;

    /**
     * @param array{
     *   id?: ?string,
     *   passageId?: ?string,
     *   title?: ?string,
     *   verses?: ?array<BiblesBooksResourceGetResponseChaptersItemVersesItem>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->passageId = $values['passageId'] ?? null;
        $this->title = $values['title'] ?? null;
        $this->verses = $values['verses'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
