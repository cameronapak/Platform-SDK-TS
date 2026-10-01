<?php

namespace Cameronapak\PlatformSdk\SearchUnified\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\SearchUnified\Types\V1SearchUnifiedCollectionGetRequestUserIntent;

class V1SearchUnifiedCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var string $query The search query string used to find matching results.
     */
    public string $query;

    /**
     * @var int $bibleId The Bible version identifier
     */
    public int $bibleId;

    /**
     * An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
     * Language ranges in this parameter may only be of the Basic Range format.
     *
     * @var ?array<string> $languageRangesArray
     */
    public ?array $languageRangesArray;

    /**
     * @var ?value-of<V1SearchUnifiedCollectionGetRequestUserIntent> $userIntent The searcher's intent. Defaults to unknown, matching the Core Search service.
     */
    public ?string $userIntent;

    /**
     * Result kinds to include: `verses` and/or `topics`. Use bracket notation to pass multiple
     * values, for example `fields[]=verses&fields[]=topics`. Omit to return all kinds. Query
     * metadata is always included.
     *
     * @var ?array<string> $fieldsArray
     */
    public ?array $fieldsArray;

    /**
     * @param array{
     *   query: string,
     *   bibleId: int,
     *   languageRangesArray?: ?array<string>,
     *   userIntent?: ?value-of<V1SearchUnifiedCollectionGetRequestUserIntent>,
     *   fieldsArray?: ?array<string>,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->query = $values['query'];
        $this->bibleId = $values['bibleId'];
        $this->languageRangesArray = $values['languageRangesArray'] ?? null;
        $this->userIntent = $values['userIntent'] ?? null;
        $this->fieldsArray = $values['fieldsArray'] ?? null;
    }
}
