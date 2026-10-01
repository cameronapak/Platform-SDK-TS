<?php

namespace Cameronapak\PlatformSdk\SearchTopics\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * An unpaginated set of topics related to a query, for pivoting to other verses in the same topic. Backed by Core Search /topics, which returns a fixed set of topics; there is no page_size or page_token. Query metadata (did_you_mean, search_instead_for) is always included.
 */
class V1SearchTopicsCollectionGetResponse extends JsonSerializableType
{
    /**
     * @var ?array<V1SearchTopicsCollectionGetResponseTopicsItem> $topics
     */
    #[JsonProperty('topics'), ArrayType([V1SearchTopicsCollectionGetResponseTopicsItem::class])]
    public ?array $topics;

    /**
     * @var array<string> $didYouMean Alternative spellings the search service suggests for the query.
     */
    #[JsonProperty('did_you_mean'), ArrayType(['string'])]
    public array $didYouMean;

    /**
     * @var ?string $searchInsteadFor A corrected query the topics were actually returned for, if any.
     */
    #[JsonProperty('search_instead_for')]
    public ?string $searchInsteadFor;

    /**
     * @var int $totalSize Total number of topics returned in this response. Because the endpoint is unpaginated, this is the full count returned, not a running total across pages.
     */
    #[JsonProperty('total_size')]
    public int $totalSize;

    /**
     * @param array{
     *   didYouMean: array<string>,
     *   totalSize: int,
     *   topics?: ?array<V1SearchTopicsCollectionGetResponseTopicsItem>,
     *   searchInsteadFor?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->topics = $values['topics'] ?? null;
        $this->didYouMean = $values['didYouMean'];
        $this->searchInsteadFor = $values['searchInsteadFor'] ?? null;
        $this->totalSize = $values['totalSize'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
