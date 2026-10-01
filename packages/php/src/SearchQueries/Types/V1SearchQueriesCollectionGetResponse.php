<?php

namespace Cameronapak\PlatformSdk\SearchQueries\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1SearchQueriesCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1SearchQueriesCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1SearchQueriesCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<V1SearchQueriesCollectionGetResponseDataItem>,
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
