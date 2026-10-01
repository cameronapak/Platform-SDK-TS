<?php

namespace Cameronapak\PlatformSdk\Highlights\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class V1HighlightsCollectionPostRequestHighlight extends JsonSerializableType
{
    /**
     * @var int $bibleId Bible version identifier
     */
    #[JsonProperty('bible_id')]
    public int $bibleId;

    /**
     * @var string $passageId The passage identifier (verse USFM format)
     */
    #[JsonProperty('passage_id')]
    public string $passageId;

    /**
     * @var string $color The highlight color in hex format
     */
    #[JsonProperty('color')]
    public string $color;

    /**
     * @param array{
     *   bibleId: int,
     *   passageId: string,
     *   color: string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->bibleId = $values['bibleId'];
        $this->passageId = $values['passageId'];
        $this->color = $values['color'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
