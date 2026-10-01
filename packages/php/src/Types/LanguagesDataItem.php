<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;
use Cameronapak\PlatformSdk\Core\Types\ArrayType;

/**
 * Region-agnostic language resource keyed by canonical BCP 47 language or language+script. Variants and a simple extensions indicator are included for completeness, but regions are excluded except for a small, enumerated set of region variants that speakers treat as distinct languages (for example pt-PT vs pt-BR, es-ES vs es-419); full extension data is always excluded.
 */
class LanguagesDataItem extends JsonSerializableType
{
    /**
     * @var string $id Canonical BCP 47 id limited to language, language+script, or one of the enumerated region-significant variants (es-419, es-ES, pt-BR, pt-PT, zh-Hant-HK, zh-Hant-TW). Other regions, variants, and extensions are not allowed.
     */
    #[JsonProperty('id')]
    public string $id;

    /**
     * @var string $language ISO 639 canonical language subtag
     */
    #[JsonProperty('language')]
    public string $language;

    /**
     * @var ?string $script ISO 15924 script code if present in id
     */
    #[JsonProperty('script')]
    public ?string $script;

    /**
     * @var ?string $scriptName The English name for the script
     */
    #[JsonProperty('script_name')]
    public ?string $scriptName;

    /**
     * @var ?array<string> $aliases Deprecated or legacy subtags mapped during canonicalization for this language.
     */
    #[JsonProperty('aliases'), ArrayType(['string'])]
    public ?array $aliases;

    /**
     * @var ?LanguagesDataItemDisplayNames $displayNames A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
     */
    #[JsonProperty('display_names')]
    public ?LanguagesDataItemDisplayNames $displayNames;

    /**
     * @var ?string $localizedName The single display name chosen from display_names for the request's Accept-Language header. Selection is per language and uses the Accept-Language priority list, then falls back to the English name, then the first available name. The Content-Language response header reports the locale negotiated for the response as a whole; in a collection an individual language may fall back to a different locale than that header when it has no name in the negotiated one. Use this when you want one name to show the user; use display_names when you need the full set.
     */
    #[JsonProperty('localized_name')]
    public ?string $localizedName;

    /**
     * @var ?array<string> $scripts All scripts known for this language (CLDR/ISO-15924)
     */
    #[JsonProperty('scripts'), ArrayType(['string'])]
    public ?array $scripts;

    /**
     * @var ?array<string> $variants Variants associated with this language (not part of the id)
     */
    #[JsonProperty('variants'), ArrayType(['string'])]
    public ?array $variants;

    /**
     * @var ?array<string> $countries Ids of countries where this language is used or supported. Extended details can be retrieved from the countries API with the provided id.
     */
    #[JsonProperty('countries'), ArrayType(['string'])]
    public ?array $countries;

    /**
     * @var ?value-of<LanguagesDataItemTextDirection> $textDirection Default text direction for this language. ltr is left to right and rtl is right to left.
     */
    #[JsonProperty('text_direction')]
    public ?string $textDirection;

    /**
     * @var ?int $writingPopulation Estimated number of souls that write in this language.
     */
    #[JsonProperty('writing_population')]
    public ?int $writingPopulation;

    /**
     * @var ?int $speakingPopulation Estimated number of souls that speak in this language.
     */
    #[JsonProperty('speaking_population')]
    public ?int $speakingPopulation;

    /**
     * @var ?int $defaultBibleId The chosen default Bible version for this language.
     */
    #[JsonProperty('default_bible_id')]
    public ?int $defaultBibleId;

    /**
     * @param array{
     *   id: string,
     *   language: string,
     *   script?: ?string,
     *   scriptName?: ?string,
     *   aliases?: ?array<string>,
     *   displayNames?: ?LanguagesDataItemDisplayNames,
     *   localizedName?: ?string,
     *   scripts?: ?array<string>,
     *   variants?: ?array<string>,
     *   countries?: ?array<string>,
     *   textDirection?: ?value-of<LanguagesDataItemTextDirection>,
     *   writingPopulation?: ?int,
     *   speakingPopulation?: ?int,
     *   defaultBibleId?: ?int,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->id = $values['id'];
        $this->language = $values['language'];
        $this->script = $values['script'] ?? null;
        $this->scriptName = $values['scriptName'] ?? null;
        $this->aliases = $values['aliases'] ?? null;
        $this->displayNames = $values['displayNames'] ?? null;
        $this->localizedName = $values['localizedName'] ?? null;
        $this->scripts = $values['scripts'] ?? null;
        $this->variants = $values['variants'] ?? null;
        $this->countries = $values['countries'] ?? null;
        $this->textDirection = $values['textDirection'] ?? null;
        $this->writingPopulation = $values['writingPopulation'] ?? null;
        $this->speakingPopulation = $values['speakingPopulation'] ?? null;
        $this->defaultBibleId = $values['defaultBibleId'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
