<?php

namespace Cameronapak\PlatformSdk\Licenses\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class V1LicensesCollectionGetRequest extends JsonSerializableType
{
    /**
     * @var int $bibleId The Bible version identifier
     */
    public int $bibleId;

    /**
     * @var string $developerId The Developer's unique ID in the Platform.
     */
    public string $developerId;

    /**
     * @var ?bool $allAvailable This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
     */
    public ?bool $allAvailable;

    /**
     * @param array{
     *   bibleId: int,
     *   developerId: string,
     *   allAvailable?: ?bool,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->bibleId = $values['bibleId'];
        $this->developerId = $values['developerId'];
        $this->allAvailable = $values['allAvailable'] ?? null;
    }
}
