<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

class BiblesResourceGetResponse extends JsonSerializableType
{
    /**
     * @var ?int $id Bible version identifier
     */
    #[JsonProperty('id')]
    public ?int $id;

    /**
     * @var ?string $abbreviation Bible version abbreviation
     */
    #[JsonProperty('abbreviation')]
    public ?string $abbreviation;

    /**
     * @var ?string $promotionalContent Longer form of copyright text provided by the publisher for the given Bible version.
     */
    #[JsonProperty('promotional_content')]
    public ?string $promotionalContent;

    /**
     * @var ?string $copyright Short version of the copyright text provided by the publisher for the given Bible version.
     */
    #[JsonProperty('copyright')]
    public ?string $copyright;

    /**
     * @var ?string $info Additional information about the Bible text version. This is commonly displayed in the reader footer for the Bible.
     */
    #[JsonProperty('info')]
    public ?string $info;

    /**
     * @var ?string $publisherUrl URL to link to publisher page from the reader's footer
     */
    #[JsonProperty('publisher_url')]
    public ?string $publisherUrl;

    /**
     * @var ?string $languageTag BCP47 canonical language tag for this Bible version
     */
    #[JsonProperty('language_tag')]
    public ?string $languageTag;

    /**
     * @var ?string $localizedAbbreviation Localized Bible version abbreviation
     */
    #[JsonProperty('localized_abbreviation')]
    public ?string $localizedAbbreviation;

    /**
     * @var ?string $localizedTitle Localized title of Bible version
     */
    #[JsonProperty('localized_title')]
    public ?string $localizedTitle;

    /**
     * @var ?string $title English title of Bible version
     */
    #[JsonProperty('title')]
    public ?string $title;

    /**
     * @var ?array<string> $books
     */
    #[JsonProperty('books'), ArrayType(['string'])]
    public ?array $books;

    /**
     * @var ?string $youversionDeepLink A deep link to this Bible version inside YouVersion
     */
    #[JsonProperty('youversion_deep_link')]
    public ?string $youversionDeepLink;

    /**
     * @var ?string $organizationId
     */
    #[JsonProperty('organization_id')]
    public ?string $organizationId;

    /**
     * @param array{
     *   id?: ?int,
     *   abbreviation?: ?string,
     *   promotionalContent?: ?string,
     *   copyright?: ?string,
     *   info?: ?string,
     *   publisherUrl?: ?string,
     *   languageTag?: ?string,
     *   localizedAbbreviation?: ?string,
     *   localizedTitle?: ?string,
     *   title?: ?string,
     *   books?: ?array<string>,
     *   youversionDeepLink?: ?string,
     *   organizationId?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->id = $values['id'] ?? null;
        $this->abbreviation = $values['abbreviation'] ?? null;
        $this->promotionalContent = $values['promotionalContent'] ?? null;
        $this->copyright = $values['copyright'] ?? null;
        $this->info = $values['info'] ?? null;
        $this->publisherUrl = $values['publisherUrl'] ?? null;
        $this->languageTag = $values['languageTag'] ?? null;
        $this->localizedAbbreviation = $values['localizedAbbreviation'] ?? null;
        $this->localizedTitle = $values['localizedTitle'] ?? null;
        $this->title = $values['title'] ?? null;
        $this->books = $values['books'] ?? null;
        $this->youversionDeepLink = $values['youversionDeepLink'] ?? null;
        $this->organizationId = $values['organizationId'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
