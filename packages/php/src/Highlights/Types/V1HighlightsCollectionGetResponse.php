<?php

namespace Cameronapak\PlatformSdk\Highlights\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1HighlightsCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1HighlightsCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1HighlightsCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<V1HighlightsCollectionGetResponseDataItem>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->data = $values['data'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
