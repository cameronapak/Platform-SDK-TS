<?php

namespace Cameronapak\PlatformSdk\Highlights\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Highlights\Types\V1HighlightsCollectionPostRequestHighlight;

class V1HighlightsCollectionPostRequest extends JsonSerializableType
{
    /**
     * @var string $requestId Request UUID for idempotent create retries.
     */
    #[JsonProperty('request_id')]
    public string $requestId;

    /**
     * @var V1HighlightsCollectionPostRequestHighlight $highlight
     */
    #[JsonProperty('highlight')]
    public V1HighlightsCollectionPostRequestHighlight $highlight;

    /**
     * @param array{
     *   requestId: string,
     *   highlight: V1HighlightsCollectionPostRequestHighlight,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->requestId = $values['requestId'];
        $this->highlight = $values['highlight'];
    }
}
