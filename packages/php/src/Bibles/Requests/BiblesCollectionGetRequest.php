<?php

namespace Cameronapak\PlatformSdk\Bibles\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesCollectionGetRequestPageSize;

class BiblesCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var ?bool $allAvailable This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
     */
    public ?bool $allAvailable;

    /**
     * An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
     * Language ranges in this parameter may only be of the Basic Range format.
     *
     * @var ?array<string> $languageRangesArray
     */
    public ?array $languageRangesArray;

    /**
     * @var ?int $licenseId Filter Bibles by a license identifier
     */
    public ?int $licenseId;

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
     * @param array{
     *   allAvailable?: ?bool,
     *   languageRangesArray?: ?array<string>,
     *   licenseId?: ?int,
     *   pageSize?: int|string|null,
     *   fieldsArray?: ?array<string>,
     *   pageToken?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->allAvailable = $values['allAvailable'] ?? null;
        $this->languageRangesArray = $values['languageRangesArray'] ?? null;
        $this->licenseId = $values['licenseId'] ?? null;
        $this->pageSize = $values['pageSize'] ?? null;
        $this->fieldsArray = $values['fieldsArray'] ?? null;
        $this->pageToken = $values['pageToken'] ?? null;
    }
}
