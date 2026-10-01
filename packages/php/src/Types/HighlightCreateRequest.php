<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class HighlightCreateRequest extends JsonSerializableType
{
    /**
     * @var string $requestId Request UUID for idempotent create retries.
     */
    #[JsonProperty('request_id')]
    public string $requestId;

    /**
     * @var HighlightCreateRequestHighlight $highlight
     */
    #[JsonProperty('highlight')]
    public HighlightCreateRequestHighlight $highlight;

    /**
     * @param array{
     *   requestId: string,
     *   highlight: HighlightCreateRequestHighlight,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->requestId = $values['requestId'];
        $this->highlight = $values['highlight'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
