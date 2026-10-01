<?php

namespace Cameronapak\PlatformSdk\Organizations\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * The Address Schema belonging to the Organization Resource in the Platform.
 */
class V1OrganizationsCollectionGetResponseDataItemAddress extends JsonSerializableType
{
    /**
     * @var ?string $formattedAddress The human-readable address of the organization.
     */
    #[JsonProperty('formatted_address')]
    public ?string $formattedAddress;

    /**
     * @var ?string $formattedLocality A less specific and more broad field that can be a combination of different regional fields.
     */
    #[JsonProperty('formatted_locality')]
    public ?string $formattedLocality;

    /**
     * @var ?string $placeId The textual identifier that uniquely identifies a place.
     */
    #[JsonProperty('place_id')]
    public ?string $placeId;

    /**
     * @var ?float $latitude The location of the address profile.
     */
    #[JsonProperty('latitude')]
    public ?float $latitude;

    /**
     * @var ?float $longitude The location of the address profile.
     */
    #[JsonProperty('longitude')]
    public ?float $longitude;

    /**
     * @var ?V1OrganizationsCollectionGetResponseDataItemAddressAdministrativeAreaLevel1 $administrativeAreaLevel1
     */
    #[JsonProperty('administrative_area_level_1')]
    public ?V1OrganizationsCollectionGetResponseDataItemAddressAdministrativeAreaLevel1 $administrativeAreaLevel1;

    /**
     * @var ?V1OrganizationsCollectionGetResponseDataItemAddressLocality $locality
     */
    #[JsonProperty('locality')]
    public ?V1OrganizationsCollectionGetResponseDataItemAddressLocality $locality;

    /**
     * @var ?V1OrganizationsCollectionGetResponseDataItemAddressCountry $country
     */
    #[JsonProperty('country')]
    public ?V1OrganizationsCollectionGetResponseDataItemAddressCountry $country;

    /**
     * @param array{
     *   formattedAddress?: ?string,
     *   formattedLocality?: ?string,
     *   placeId?: ?string,
     *   latitude?: ?float,
     *   longitude?: ?float,
     *   administrativeAreaLevel1?: ?V1OrganizationsCollectionGetResponseDataItemAddressAdministrativeAreaLevel1,
     *   locality?: ?V1OrganizationsCollectionGetResponseDataItemAddressLocality,
     *   country?: ?V1OrganizationsCollectionGetResponseDataItemAddressCountry,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->formattedAddress = $values['formattedAddress'] ?? null;
        $this->formattedLocality = $values['formattedLocality'] ?? null;
        $this->placeId = $values['placeId'] ?? null;
        $this->latitude = $values['latitude'] ?? null;
        $this->longitude = $values['longitude'] ?? null;
        $this->administrativeAreaLevel1 = $values['administrativeAreaLevel1'] ?? null;
        $this->locality = $values['locality'] ?? null;
        $this->country = $values['country'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
