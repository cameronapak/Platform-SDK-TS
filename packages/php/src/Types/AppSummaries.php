<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * A collection of public app summary resources.
 */
class AppSummaries extends JsonSerializableType
{
    /**
     * @var array<AppSummariesDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([AppSummariesDataItem::class])]
    public array $data;

    /**
     * @param array{
     *   data: array<AppSummariesDataItem>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->data = $values['data'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
