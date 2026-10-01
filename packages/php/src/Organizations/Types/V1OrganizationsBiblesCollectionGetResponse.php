<?php

namespace Cameronapak\PlatformSdk\Organizations\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class V1OrganizationsBiblesCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1OrganizationsBiblesCollectionGetResponseDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([V1OrganizationsBiblesCollectionGetResponseDataItem::class])]
    public ?array $data;

    /**
     * @var ?string $nextPageToken Token to send to server when retrieving the next page of results.
     */
    #[JsonProperty('next_page_token')]
    public ?string $nextPageToken;

    /**
     * @var ?int $totalSize Total number of bibles in collection matching parameters.
     */
    #[JsonProperty('total_size')]
    public ?int $totalSize;

    /**
     * @param array{
     *   data?: ?array<V1OrganizationsBiblesCollectionGetResponseDataItem>,
     *   nextPageToken?: ?string,
     *   totalSize?: ?int,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->data = $values['data'] ?? null;
        $this->nextPageToken = $values['nextPageToken'] ?? null;
        $this->totalSize = $values['totalSize'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
