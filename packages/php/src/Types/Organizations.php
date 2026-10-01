<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class Organizations extends JsonSerializableType
{
    /**
     * @var ?array<OrganizationsDataItem> $data
     */
    #[JsonProperty('data'), ArrayType([OrganizationsDataItem::class])]
    public ?array $data;

    /**
     * @var ?string $nextPageToken Token to send to server when retrieving the next page of results.
     */
    #[JsonProperty('next_page_token')]
    public ?string $nextPageToken;

    /**
     * @param array{
     *   data?: ?array<OrganizationsDataItem>,
     *   nextPageToken?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->data = $values['data'] ?? null;
        $this->nextPageToken = $values['nextPageToken'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
