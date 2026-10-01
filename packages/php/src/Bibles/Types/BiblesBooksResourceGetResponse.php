<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesBooksResourceGetResponse extends JsonSerializableType
{
    /**
     * @var ?string $id Book identifier
     */
    #[JsonProperty('id')]
    public ?string $id;

    /**
     * @var ?string $title Book title
     */
    #[JsonProperty('title')]
    public ?string $title;

    /**
     * @var ?string $fullTitle Full book title if available
     */
    #[JsonProperty('full_title')]
    public ?string $fullTitle;

    /**
     * @var ?string $abbreviation Book name abbreviation if provided by the publisher
     */
    #[JsonProperty('abbreviation')]
    public ?string $abbreviation;

    /**
     * @var ?value-of<BiblesBooksResourceGetResponseCanon> $canon Canon identifier
     */
    #[JsonProperty('canon')]
    public ?string $canon;

    /**
     * @var ?array<BiblesBooksResourceGetResponseChaptersItem> $chapters Chapters in the book
     */
    #[JsonProperty('chapters'), ArrayType([BiblesBooksResourceGetResponseChaptersItem::class])]
    public ?array $chapters;

    /**
     * @var ?BiblesBooksResourceGetResponseIntro $intro
     */
    #[JsonProperty('intro')]
    public ?BiblesBooksResourceGetResponseIntro $intro;

    /**
     * @param array{
     *   id?: ?string,
     *   title?: ?string,
     *   fullTitle?: ?string,
     *   abbreviation?: ?string,
     *   canon?: ?value-of<BiblesBooksResourceGetResponseCanon>,
     *   chapters?: ?array<BiblesBooksResourceGetResponseChaptersItem>,
     *   intro?: ?BiblesBooksResourceGetResponseIntro,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->title = $values['title'] ?? null;
        $this->fullTitle = $values['fullTitle'] ?? null;
        $this->abbreviation = $values['abbreviation'] ?? null;
        $this->canon = $values['canon'] ?? null;
        $this->chapters = $values['chapters'] ?? null;
        $this->intro = $values['intro'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
