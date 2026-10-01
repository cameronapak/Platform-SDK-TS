<?php

namespace Cameronapak\PlatformSdk\Types;

enum AppSummariesDataItemStatus: string
{
    case Development = "development";
    case Live = "live";
    case Archived = "archived";
}
