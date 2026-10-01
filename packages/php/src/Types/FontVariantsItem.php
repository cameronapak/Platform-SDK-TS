<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class FontVariantsItem extends JsonSerializableType
{
    /**
     * @var int $weight Numeric font weight for this variant.
     */
    #[JsonProperty('weight')]
    public int $weight;

    /**
     * @var value-of<FontVariantsItemStyle> $style Font style for this variant.
     */
    #[JsonProperty('style')]
    public string $style;

    /**
     * @var array<FontVariantsItemSourcesItem> $sources CDN assets available for this specific weight and style.
     */
    #[JsonProperty('sources'), ArrayType([FontVariantsItemSourcesItem::class])]
    public array $sources;

    /**
     * @param array{
     *   weight: int,
     *   style: value-of<FontVariantsItemStyle>,
     *   sources: array<FontVariantsItemSourcesItem>,
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
