<?php

namespace Cameronapak\PlatformSdk\SearchTopics\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class V1SearchTopicsCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var string $query The search query string used to find matching results.
     */
    public string $query;

    /**
     * An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
     * Language ranges in this parameter may only be of the Basic Range format.
     *
     * @var ?array<string> $languageRangesArray
     */
    public ?array $languageRangesArray;

    /**
     * @param array{
     *   query: string,
     *   languageRangesArray?: ?array<string>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->query = $values['query'];
        $this->languageRangesArray = $values['languageRangesArray'] ?? null;
    }
}
