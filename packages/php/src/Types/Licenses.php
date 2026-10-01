<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Licenses extends JsonSerializableType
{
    /**
     * @var ?array<LicensesDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([LicensesDataItem::class])]
    public ?array $data;

    /**
     * @param array{
     *   data?: ?array<LicensesDataItem>,
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
