<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class VerseOfTheDay extends JsonSerializableType
{
    /**
     * @var int $day
     */
    #[JsonProperty('day')]
    public int $day;

    /**
     * @var string $passageId The passage identifier
     */
    #[JsonProperty('passage_id')]
    public string $passageId;

    /**
     * @param array{
     *   day: int,
     *   passageId: string,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->day = $values['day'];
        $this->passageId = $values['passageId'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
