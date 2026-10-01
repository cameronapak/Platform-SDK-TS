<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * Assigns (or clears) a spotlight manifest entry's go-live month.
 */
class SpotlightPublishDateRequest extends JsonSerializableType
{
    /**
     * @var ?string $publishDate Go-live month as YYYY-MM; null clears the assignment.
     */
    #[JsonProperty('publishDate')]
    public ?string $publishDate;

    /**
     * @param array{
     *   publishDate?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->publishDate = $values['publishDate'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
