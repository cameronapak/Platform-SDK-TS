<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class FontsDataItemVariantsItemSourcesItem extends JsonSerializableType
{
    /**
     * @var value-of<FontsDataItemVariantsItemSourcesItemFormat> $format The file format for this source asset.
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
     *   format: value-of<FontsDataItemVariantsItemSourcesItemFormat>,
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
