<?php

namespace Cameronapak\PlatformSdk\SearchQueries\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * A query object: a search string a user might run, plus where it came from (e.g. a community suggestion or a trending search).
 */
class V1SearchQueriesCollectionGetResponseDataItem extends JsonSerializableType
{
    /**
     * @var string $text The suggested or trending search query string.
     */
    #[JsonProperty('text')]
    public string $text;

    /**
     * @var ?string $source Where the query came from, e.g. `community` or `trending`.
     */
    #[JsonProperty('source')]
    public ?string $source;

    /**
     * @param array{
     *   text: string,
     *   source?: ?string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->text = $values['text'];
        $this->source = $values['source'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
