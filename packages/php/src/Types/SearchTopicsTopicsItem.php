<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * A topic related to the query, for pivoting to other verses in the same topic.
 */
class SearchTopicsTopicsItem extends JsonSerializableType
{
    /**
     * @var ?int $id The topic's identifier in the Core Search catalog.
     */
    #[JsonProperty('id')]
    public ?int $id;

    /**
     * @var string $text The topic label.
     */
    #[JsonProperty('text')]
    public string $text;

    /**
     * @var array<string> $subtopics Related subtopic labels, if any.
     */
    #[JsonProperty('subtopics'), ArrayType(['string'])]
    public array $subtopics;

    /**
     * @param array{
     *   text: string,
     *   subtopics: array<string>,
     *   id?: ?int,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->id = $values['id'] ?? null;
        $this->text = $values['text'];
        $this->subtopics = $values['subtopics'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
