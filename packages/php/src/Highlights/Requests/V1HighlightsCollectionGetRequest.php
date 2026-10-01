<?php

namespace Cameronapak\PlatformSdk\Highlights\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class V1HighlightsCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var int $bibleId The Bible version identifier
     */
    public int $bibleId;

    /**
     * @var string $passageId The passage identifier (verse or chapter USFM format)
     */
    public string $passageId;

    /**
     * @param array{
     *   bibleId: int,
     *   passageId: string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->bibleId = $values['bibleId'];
        $this->passageId = $values['passageId'];
    }
}
