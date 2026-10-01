<?php

namespace Cameronapak\PlatformSdk\Apps\Types;

enum V1AppsResourceGetResponseStatus: string
{
    case Development = "development";
    case Live = "live";
    case Archived = "archived";
}
