<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

enum BiblesBooksResourceGetResponseCanon: string
{
    case NewTestament = "new_testament";
    case OldTestament = "old_testament";
    case Deuterocanon = "deuterocanon";
}
