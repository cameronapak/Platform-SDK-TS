<?php

namespace Cameronapak\PlatformSdk\SearchVerses\Types;

enum V1SearchVersesCollectionGetRequestUserIntent: string
{
    case Unknown = "unknown";
    case Topical = "topical";
    case Text = "text";
    case Reference = "reference";
}
