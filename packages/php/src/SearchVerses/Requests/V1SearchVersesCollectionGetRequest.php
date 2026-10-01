<?php

namespace Cameronapak\PlatformSdk\SearchVerses\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\SearchVerses\Types\V1SearchVersesCollectionGetRequestUserIntent;

class V1SearchVersesCollectionGetRequest extends JsonSerializableType
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
     * @var ?value-of<V1SearchVersesCollectionGetRequestUserIntent> $userIntent The searcher's intent. Defaults to unknown, matching the Core Search service.
     */
    public ?string $userIntent;

    /**
     * @var ?int $pageSize The number of verse results to return in the collection. Must be between 1 and 99.
     */
    public ?int $pageSize;

    /**
     * @var ?string $pageToken The page token to retrieve results from.
     */
    public ?string $pageToken;

    /**
     * @param array{
     *   query: string,
     *   bibleId: int,
     *   userIntent?: ?value-of<V1SearchVersesCollectionGetRequestUserIntent>,
     *   pageSize?: ?int,
     *   pageToken?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->query = $values['query'];
        $this->bibleId = $values['bibleId'];
        $this->userIntent = $values['userIntent'] ?? null;
        $this->pageSize = $values['pageSize'] ?? null;
        $this->pageToken = $values['pageToken'] ?? null;
    }
}
