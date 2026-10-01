<?php

namespace Cameronapak\PlatformSdk\SearchVerses\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * A paginated list of Bible verse search results. Verse results carry references and metadata only (e.g. JHN.3.16), never passage text; resolve verse text through the licensed-content endpoint.
 */
class V1SearchVersesCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1SearchVersesCollectionGetResponseVersesItem> $verses
     */
    #[JsonProperty('verses'), ArrayType([V1SearchVersesCollectionGetResponseVersesItem::class])]
    public ?array $verses;

    /**
     * @var ?string $userIntent The intent the Core Search service resolved for the query.
     */
    #[JsonProperty('user_intent')]
    public ?string $userIntent;

    /**
     * @var array<string> $didYouMean Alternative spellings the search service suggests for the query.
     */
    #[JsonProperty('did_you_mean'), ArrayType(['string'])]
    public array $didYouMean;

    /**
     * @var ?string $searchInsteadFor A corrected query the results were actually returned for, if any.
     */
    #[JsonProperty('search_instead_for')]
    public ?string $searchInsteadFor;

    /**
     * @var ?string $nextPageToken Token to send to the server when retrieving the next page of results.
     */
    #[JsonProperty('next_page_token')]
    public ?string $nextPageToken;

    /**
     * @param array{
     *   didYouMean: array<string>,
     *   verses?: ?array<V1SearchVersesCollectionGetResponseVersesItem>,
     *   userIntent?: ?string,
     *   searchInsteadFor?: ?string,
     *   nextPageToken?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->verses = $values['verses'] ?? null;
        $this->userIntent = $values['userIntent'] ?? null;
        $this->didYouMean = $values['didYouMean'];
        $this->searchInsteadFor = $values['searchInsteadFor'] ?? null;
        $this->nextPageToken = $values['nextPageToken'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
