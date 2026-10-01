<?php

namespace Cameronapak\PlatformSdk\Languages\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Languages\Types\V1LanguagesCollectionGetRequestPageSize;

class V1LanguagesCollectionGetRequest extends JsonSerializableType
{
    /**
     * The number of items to return in the collection.  Numeric values must be between 1 and 99.
     * Special value "*" is supported only when used in combination with the `fields` parameter and
     * when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
     * server will return all matching items for the requested resource (no numeric page limit).
     *
     * @var int|string|null $pageSize
     */
    public int|string|null $pageSize;

    /**
     * A list of top-level fields to include in each resource object. Use bracket notation to pass
     * multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
     * When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
     *
     * @var ?array<string> $fieldsArray
     */
    public ?array $fieldsArray;

    /**
     * @var ?string $pageToken The page token to retrieve results from.
     */
    public ?string $pageToken;

    /**
     * @var ?string $country The ISO 3166 2 character country code
     */
    public ?string $country;

    /**
     * @var ?bool $biblesAvailable Filter languages based on whether Bible content is available for that language.
     */
    public ?bool $biblesAvailable;

    /**
     * @var 'en' $acceptLanguage The localization preferred as a response to the request. See RFC 2616 section 14.4 for further details.
     */
    public string $acceptLanguage;

    /**
     * @param array{
     *   acceptLanguage?: 'en',
     *   pageSize?: int|string|null,
     *   fieldsArray?: ?array<string>,
     *   pageToken?: ?string,
     *   country?: ?string,
     *   biblesAvailable?: ?bool,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->pageSize = $values['pageSize'] ?? null;
        $this->fieldsArray = $values['fieldsArray'] ?? null;
        $this->pageToken = $values['pageToken'] ?? null;
        $this->country = $values['country'] ?? null;
        $this->biblesAvailable = $values['biblesAvailable'] ?? null;
        $this->acceptLanguage = $values['acceptLanguage'] ?? 'en';
    }
}
