<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class BiblesBooksChaptersResourceGetResponseVersesItem extends JsonSerializableType
{
    /**
     * @var ?string $id Verse identifier
     */
    #[JsonProperty('id')]
    public ?string $id;

    /**
     * @var ?string $passageId Passage identifier
     */
    #[JsonProperty('passage_id')]
    public ?string $passageId;

    /**
     * @var ?string $title Verse title
     */
    #[JsonProperty('title')]
    public ?string $title;

    /**
     * @param array{
     *   id?: ?string,
     *   passageId?: ?string,
     *   title?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->passageId = $values['passageId'] ?? null;
        $this->title = $values['title'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
