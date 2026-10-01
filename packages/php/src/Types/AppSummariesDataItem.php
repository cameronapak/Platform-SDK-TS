<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * A public app summary resource.
 */
class AppSummariesDataItem extends JsonSerializableType
{
    /**
     * @var ?string $appId
     */
    #[JsonProperty('app_id')]
    public ?string $appId;

    /**
     * @var ?string $name
     */
    #[JsonProperty('name')]
    public ?string $name;

    /**
     * @var ?string $description
     */
    #[JsonProperty('description')]
    public ?string $description;

    /**
     * @var ?string $websiteUrl
     */
    #[JsonProperty('website_url')]
    public ?string $websiteUrl;

    /**
     * @var ?value-of<AppSummariesDataItemStatus> $status
     */
    #[JsonProperty('status')]
    public ?string $status;

    /**
     * @param array{
     *   appId?: ?string,
     *   name?: ?string,
     *   description?: ?string,
     *   websiteUrl?: ?string,
     *   status?: ?value-of<AppSummariesDataItemStatus>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->appId = $values['appId'] ?? null;
        $this->name = $values['name'] ?? null;
        $this->description = $values['description'] ?? null;
        $this->websiteUrl = $values['websiteUrl'] ?? null;
        $this->status = $values['status'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
