<?php

namespace Cameronapak\PlatformSdk\Types;

enum BookCanon: string
{
    case NewTestament = "new_testament";
    case OldTestament = "old_testament";
    case Deuterocanon = "deuterocanon";
}
