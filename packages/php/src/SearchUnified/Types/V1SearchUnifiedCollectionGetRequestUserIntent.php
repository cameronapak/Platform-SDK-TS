<?php

namespace Cameronapak\PlatformSdk\SearchUnified\Types;

enum V1SearchUnifiedCollectionGetRequestUserIntent: string
{
    case Unknown = "unknown";
    case Topical = "topical";
    case Text = "text";
    case Reference = "reference";
}
