<?php

namespace Cameronapak\PlatformSdk\Fonts\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * A font family resource with available variants and CDN-backed source files.
 */
class V1FontsCollectionGetResponseDataItem extends JsonSerializableType
{
    /**
     * @var int $id Stable integer identifier for the font family.
     */
    #[JsonProperty('id')]
    public int $id;

    /**
     * @var string $slug Stable URL-safe identifier for the font family.
     */
    #[JsonProperty('slug')]
    public string $slug;

    /**
     * @var string $family Canonical font-family name clients should use.
     */
    #[JsonProperty('family')]
    public string $family;

    /**
     * @var array<V1FontsCollectionGetResponseDataItemVariantsItem> $variants Available faces within this font family.
     */
    #[JsonProperty('variants'), ArrayType([V1FontsCollectionGetResponseDataItemVariantsItem::class])]
    public array $variants;

    /**
     * @param array{
     *   id: int,
     *   slug: string,
     *   family: string,
     *   variants: array<V1FontsCollectionGetResponseDataItemVariantsItem>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->id = $values['id'];
        $this->slug = $values['slug'];
        $this->family = $values['family'];
        $this->variants = $values['variants'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
