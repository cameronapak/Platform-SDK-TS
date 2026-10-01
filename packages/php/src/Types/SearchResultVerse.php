<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
 */
class SearchResultVerse extends JsonSerializableType
{
    /**
     * @var string $reference USFM scripture reference for the matched verse.
     */
    #[JsonProperty('reference')]
    public string $reference;

    /**
     * @param array{
     *   reference: string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->reference = $values['reference'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
