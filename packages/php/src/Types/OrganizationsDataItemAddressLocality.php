<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class OrganizationsDataItemAddressLocality extends JsonSerializableType
{
    /**
     * @var ?string $shortName The short name of the place, e.g. "OK" for Oklahoma.
     */
    #[JsonProperty('short_name')]
    public ?string $shortName;

    /**
     * @var ?string $longName The long name of the place, e.g. "Oklahoma" for the state of Oklahoma.
     */
    #[JsonProperty('long_name')]
    public ?string $longName;

    /**
     * @param array{
     *   shortName?: ?string,
     *   longName?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->shortName = $values['shortName'] ?? null;
        $this->longName = $values['longName'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
