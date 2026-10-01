<?php

namespace Cameronapak\PlatformSdk\SearchUnified\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * A single unified, unpaginated set of search results grouped by kind: verse references and related topics. Returns one combined page of top results per kind; there is no page_size or page_token. Use the kind-specific endpoints (search-verses, search-topics) to page through the full result set for a kind.
 */
class V1SearchUnifiedCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1SearchUnifiedCollectionGetResponseVersesItem> $verses
     */
    #[JsonProperty('verses'), ArrayType([V1SearchUnifiedCollectionGetResponseVersesItem::class])]
    public ?array $verses;

    /**
     * @var ?array<V1SearchUnifiedCollectionGetResponseTopicsItem> $topics
     */
    #[JsonProperty('topics'), ArrayType([V1SearchUnifiedCollectionGetResponseTopicsItem::class])]
    public ?array $topics;

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
     * @param array{
     *   didYouMean: array<string>,
     *   verses?: ?array<V1SearchUnifiedCollectionGetResponseVersesItem>,
     *   topics?: ?array<V1SearchUnifiedCollectionGetResponseTopicsItem>,
     *   userIntent?: ?string,
     *   searchInsteadFor?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->verses = $values['verses'] ?? null;
        $this->topics = $values['topics'] ?? null;
        $this->userIntent = $values['userIntent'] ?? null;
        $this->didYouMean = $values['didYouMean'];
        $this->searchInsteadFor = $values['searchInsteadFor'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
