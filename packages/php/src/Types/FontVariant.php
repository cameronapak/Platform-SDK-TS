<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class FontVariant extends JsonSerializableType
{
    /**
     * @var int $weight Numeric font weight for this variant.
     */
    #[JsonProperty('weight')]
    public int $weight;

    /**
     * @var value-of<FontVariantStyle> $style Font style for this variant.
     */
    #[JsonProperty('style')]
    public string $style;

    /**
     * @var array<FontVariantSourcesItem> $sources CDN assets available for this specific weight and style.
     */
    #[JsonProperty('sources'), ArrayType([FontVariantSourcesItem::class])]
    public array $sources;

    /**
     * @param array{
     *   weight: int,
     *   style: value-of<FontVariantStyle>,
     *   sources: array<FontVariantSourcesItem>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->weight = $values['weight'];
        $this->style = $values['style'];
        $this->sources = $values['sources'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
