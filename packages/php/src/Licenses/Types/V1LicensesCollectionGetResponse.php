<?php

namespace Cameronapak\PlatformSdk\Licenses\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1LicensesCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1LicensesCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1LicensesCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<V1LicensesCollectionGetResponseDataItem>,
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
