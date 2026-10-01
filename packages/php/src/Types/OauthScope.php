<?php

namespace Cameronapak\PlatformSdk\Types;

enum OauthScope: string
{
    case ReadHighlights = "read_highlights";
    case WriteHighlights = "write_highlights";
}
