<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * The Organization Resource in the Platform.
 */
class Organization extends JsonSerializableType
{
    /**
     * @var string $id The unique identifier of the organization in the Platform.
     */
    #[JsonProperty('id')]
    public string $id;

    /**
     * @var ?string $parentOrganizationId The id of the parent organization if one exists.
     */
    #[JsonProperty('parent_organization_id')]
    public ?string $parentOrganizationId;

    /**
     * @var ?string $name Publisher's name in the language negotiated by Accept-Language headers. If none match known translations, then the primary language of the publisher is used. Whichever language is chosen will be sent back in the Content-Language header.
     */
    #[JsonProperty('name')]
    public ?string $name;

    /**
     * @var ?string $description Description of the organization. It's purpose and goals, values and mission, etc.
     */
    #[JsonProperty('description')]
    public ?string $description;

    /**
     * @var ?string $email The contact email address for the organization if provided.
     */
    #[JsonProperty('email')]
    public ?string $email;

    /**
     * @var ?string $phone The contact phone number for the organization if provided.
     */
    #[JsonProperty('phone')]
    public ?string $phone;

    /**
     * @var ?string $primaryLanguage The primary language of the organization.
     */
    #[JsonProperty('primary_language')]
    public ?string $primaryLanguage;

    /**
     * @var ?string $websiteUrl The web site for the organization.
     */
    #[JsonProperty('website_url')]
    public ?string $websiteUrl;

    /**
     * @var ?OrganizationAddress $address The Address Schema belonging to the Organization Resource in the Platform.
     */
    #[JsonProperty('address')]
    public ?OrganizationAddress $address;

    /**
     * @param array{
     *   id: string,
     *   parentOrganizationId?: ?string,
     *   name?: ?string,
     *   description?: ?string,
     *   email?: ?string,
     *   phone?: ?string,
     *   primaryLanguage?: ?string,
     *   websiteUrl?: ?string,
     *   address?: ?OrganizationAddress,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->id = $values['id'];
        $this->parentOrganizationId = $values['parentOrganizationId'] ?? null;
        $this->name = $values['name'] ?? null;
        $this->description = $values['description'] ?? null;
        $this->email = $values['email'] ?? null;
        $this->phone = $values['phone'] ?? null;
        $this->primaryLanguage = $values['primaryLanguage'] ?? null;
        $this->websiteUrl = $values['websiteUrl'] ?? null;
        $this->address = $values['address'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
