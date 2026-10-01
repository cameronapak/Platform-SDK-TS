<?php

namespace Cameronapak\PlatformSdk\SearchQueries\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class V1SearchQueriesCollectionGetRequest extends JsonSerializableType
{
    /**
     * An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
     * Language ranges in this parameter may only be of the Basic Range format.
     *
     * @var ?array<string> $languageRangesArray
     */
    public ?array $languageRangesArray;

    /**
     * @var ?string $query The partial query for as-you-type suggestions. Omit for trending queries.
     */
    public ?string $query;

    /**
     * @var ?bool $trending Return recently popular searches for the language instead of suggestions.
     */
    public ?bool $trending;

    /**
     * @param array{
     *   languageRangesArray?: ?array<string>,
     *   query?: ?string,
     *   trending?: ?bool,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->languageRangesArray = $values['languageRangesArray'] ?? null;
        $this->query = $values['query'] ?? null;
        $this->trending = $values['trending'] ?? null;
    }
}
