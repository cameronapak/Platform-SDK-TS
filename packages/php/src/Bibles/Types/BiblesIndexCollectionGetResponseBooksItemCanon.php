<?php

namespace Cameronapak\PlatformSdk\Bibles\Types;

enum BiblesIndexCollectionGetResponseBooksItemCanon: string
{
    case NewTestament = "new_testament";
    case OldTestament = "old_testament";
    case Deuterocanon = "deuterocanon";
}
