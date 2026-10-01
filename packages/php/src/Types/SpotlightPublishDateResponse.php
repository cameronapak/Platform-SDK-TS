<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * Manifest entry state returned after a publish-date assignment.
 */
class SpotlightPublishDateResponse extends JsonSerializableType
{
    /**
     * @var string $slug Slug of the updated manifest entry.
     */
    #[JsonProperty('slug')]
    public string $slug;

    /**
     * @var ?string $publishDate The assigned go-live month (YYYY-MM), or null if cleared.
     */
    #[JsonProperty('publishDate')]
    public ?string $publishDate;

    /**
     * @param array{
     *   slug: string,
     *   publishDate?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->slug = $values['slug'];
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
