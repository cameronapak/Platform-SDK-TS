<?php

namespace Cameronapak\PlatformSdk\Types;

enum AppSummaryStatus: string
{
    case Development = "development";
    case Live = "live";
    case Archived = "archived";
}
