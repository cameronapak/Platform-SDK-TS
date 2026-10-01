<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;
use DateTime;
use Cameronapak\PlatformSdk\Core\Types\Date;

class LicensesDataItem extends JsonSerializableType
{
    /**
     * @var ?int $id
     */
    #[JsonProperty('id')]
    public ?int $id;

    /**
     * @var ?string $name
     */
    #[JsonProperty('name')]
    public ?string $name;

    /**
     * @var ?int $version
     */
    #[JsonProperty('version')]
    public ?int $version;

    /**
     * @var ?string $organizationId The YouVersion Organization ID that owns this license, or null for licenses not owned by any organization (e.g. Public Domain and Creative Commons).
     */
    #[JsonProperty('organization_id')]
    public ?string $organizationId;

    /**
     * @var ?string $html HTML representation of the license terms.
     */
    #[JsonProperty('html')]
    public ?string $html;

    /**
     * @var ?array<int> $bibleIds
     */
    #[JsonProperty('bible_ids'), ArrayType(['integer'])]
    public ?array $bibleIds;

    /**
     * @var ?string $uri URI pointing to the license terms.
     */
    #[JsonProperty('uri')]
    public ?string $uri;

    /**
     * @var ?DateTime $agreedDt The date which the developer id passed agreed to this license or null if not agreed to
     */
    #[JsonProperty('agreed_dt'), Date(Date::TYPE_DATETIME)]
    public ?DateTime $agreedDt;

    /**
     * @var ?string $yvpUserId The YouVersion Platform User id that was logged into the dev portal and agreed to the license
     */
    #[JsonProperty('yvp_user_id')]
    public ?string $yvpUserId;

    /**
     * @param array{
     *   id?: ?int,
     *   name?: ?string,
     *   version?: ?int,
     *   organizationId?: ?string,
     *   html?: ?string,
     *   bibleIds?: ?array<int>,
     *   uri?: ?string,
     *   agreedDt?: ?DateTime,
     *   yvpUserId?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->name = $values['name'] ?? null;
        $this->version = $values['version'] ?? null;
        $this->organizationId = $values['organizationId'] ?? null;
        $this->html = $values['html'] ?? null;
        $this->bibleIds = $values['bibleIds'] ?? null;
        $this->uri = $values['uri'] ?? null;
        $this->agreedDt = $values['agreedDt'] ?? null;
        $this->yvpUserId = $values['yvpUserId'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
