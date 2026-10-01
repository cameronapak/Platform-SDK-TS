<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class Passage extends JsonSerializableType
{
    /**
     * @var ?string $id A canonical representation of the passage returned
     */
    #[JsonProperty('id')]
    public ?string $id;

    /**
     * @var ?string $content The Bible text of the requested passage in either text or html format.
     */
    #[JsonProperty('content')]
    public ?string $content;

    /**
     * @var ?string $reference A human-readable reference
     */
    #[JsonProperty('reference')]
    public ?string $reference;

    /**
     * @param array{
     *   id?: ?string,
     *   content?: ?string,
     *   reference?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->content = $values['content'] ?? null;
        $this->reference = $values['reference'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
