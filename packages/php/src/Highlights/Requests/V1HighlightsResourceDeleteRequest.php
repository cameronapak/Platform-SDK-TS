<?php

namespace Cameronapak\PlatformSdk\Highlights\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class V1HighlightsResourceDeleteRequest extends JsonSerializableType
{
    /**
     * @var int $bibleId The Bible version identifier
     */
    public int $bibleId;

    /**
     * @param array{
     *   bibleId: int,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->bibleId = $values['bibleId'];
    }
}
