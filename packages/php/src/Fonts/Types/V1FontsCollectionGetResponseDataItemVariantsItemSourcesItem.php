<?php

namespace Cameronapak\PlatformSdk\Fonts\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem extends JsonSerializableType
{
    /**
     * @var value-of<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat> $format The file format for this source asset.
     */
    #[JsonProperty('format')]
    public string $format;

    /**
     * @var string $url Fully qualified CDN URL for this font source file.
     */
    #[JsonProperty('url')]
    public string $url;

    /**
     * @param array{
     *   format: value-of<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat>,
     *   url: string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->format = $values['format'];
        $this->url = $values['url'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
