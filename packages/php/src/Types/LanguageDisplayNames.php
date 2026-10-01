<?php

namespace Cameronapak\PlatformSdk\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

/**
 * A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
 */
class LanguageDisplayNames extends JsonSerializableType
{
    /**
     * @var ?string $en
     */
    #[JsonProperty('en')]
    public ?string $en;

    /**
     * @param array{
     *   en?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->en = $values['en'] ?? null;
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
