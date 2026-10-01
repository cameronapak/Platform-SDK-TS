<?php

namespace Cameronapak\PlatformSdk\Bibles\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesPassagesResourceGetRequestFormat;

class BiblesPassagesResourceGetRequest extends JsonSerializableType
{
    /**
     * @var ?value-of<BiblesPassagesResourceGetRequestFormat> $format The desired Bible content format (text or html). Headings and notes are included only when format=html; format=text returns verse text without them.
     */
    public ?string $format;

    /**
     * @var ?bool $includeHeadings Whether or not headings should be included in the Bible content. Headings are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
     */
    public ?bool $includeHeadings;

    /**
     * @var ?bool $includeNotes Whether or not notes should be included in the Bible content. Notes are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
     */
    public ?bool $includeNotes;

    /**
     * @param array{
     *   format?: ?value-of<BiblesPassagesResourceGetRequestFormat>,
     *   includeHeadings?: ?bool,
     *   includeNotes?: ?bool,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->format = $values['format'] ?? null;
        $this->includeHeadings = $values['includeHeadings'] ?? null;
        $this->includeNotes = $values['includeNotes'] ?? null;
    }
}
