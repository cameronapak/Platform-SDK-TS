<?php

namespace Cameronapak\PlatformSdk\Exceptions;

use Throwable;

/**
 * This exception type will be thrown for any non-2XX API responses.
 */
class CameronapakApiException extends CameronapakException
{
    /**
     * @var mixed $body
     */
    private mixed $body;

    /** @var array<string, array<string>> */
    private array $headers;

    /**
     * @param string $message
     * @param int $statusCode
     * @param mixed $body
     * @param ?Throwable $previous
     */
    public function __construct(
        string $message,
        int $statusCode,
        mixed $body,
        ?Throwable $previous = null,
        array $headers = [],
    ) {
        $this->body = $body;
        $this->headers = array_change_key_case($headers, CASE_LOWER);
        parent::__construct($message, $statusCode, $previous);
    }

    /**
     * Returns the body of the response that triggered the exception.
     *
     * @return mixed
     */
    public function getBody(): mixed
    {
        return $this->body;
    }

    /**
     * @return string
     */
    /** @return array<string, array<string>> */
    public function getHeaders(): array
    {
        return $this->headers;
    }

    public function __toString(): string
    {
        // Explicit getBody()/getHeaders() retain data; automatic logging does not.
        return $this->message . '; Status Code: ' . $this->getCode() . "\n";
    }
}
