<?php

namespace Cameronapak\PlatformSdk\Fonts\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1FontsResourceGetResponseVariantsItem extends JsonSerializableType
{
    /**
     * @var int $weight Numeric font weight for this variant.
     */
    #[JsonProperty('weight')]
    public int $weight;

    /**
     * @var value-of<V1FontsResourceGetResponseVariantsItemStyle> $style Font style for this variant.
     */
    #[JsonProperty('style')]
    public string $style;

    /**
     * @var array<V1FontsResourceGetResponseVariantsItemSourcesItem> $sources CDN assets available for this specific weight and style.
     */
    #[JsonProperty('sources'), ArrayType([V1FontsResourceGetResponseVariantsItemSourcesItem::class])]
    public array $sources;

    /**
     * @param array{
     *   weight: int,
     *   style: value-of<V1FontsResourceGetResponseVariantsItemStyle>,
     *   sources: array<V1FontsResourceGetResponseVariantsItemSourcesItem>,
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
