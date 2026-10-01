<?php

namespace Cameronapak\PlatformSdk\Bibles\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksCollectionGetRequestCanon;

class BiblesBooksCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var ?value-of<BiblesBooksCollectionGetRequestCanon> $canon The Canon to filter results by
     */
    public ?string $canon;

    /**
     * @param array{
     *   canon?: ?value-of<BiblesBooksCollectionGetRequestCanon>,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->canon = $values['canon'] ?? null;
    }
}
