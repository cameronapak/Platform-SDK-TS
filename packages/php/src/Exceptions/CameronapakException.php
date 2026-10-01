<?php

namespace Cameronapak\PlatformSdk\Exceptions;

use Exception;

/**
 * Base exception class for all exceptions thrown by the SDK.
 */
class CameronapakException extends Exception
{
    public function __toString(): string
    {
        // Do not print request arguments or a credential-bearing transport cause.
        return static::class . ': ' . $this->message . "\n";
    }
}
