<?php

namespace Cameronapak\PlatformSdk\DataExchange\Types;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;
use Cameronapak\PlatformSdk\Core\Json\JsonProperty;

class DataExchangeTokenPostResponse extends JsonSerializableType
{
    /**
     * @var string $token Opaque single-use token for the data exchange flow. Returned only once.
     */
    #[JsonProperty('token')]
    public string $token;

    /**
     * @var 'data_exchange' $tokenType
     */
    #[JsonProperty('token_type')]
    public string $tokenType;

    /**
     * @var int $expiresIn Token lifetime in seconds.
     */
    #[JsonProperty('expires_in')]
    public int $expiresIn;

    /**
     * @param array{
     *   token: string,
     *   tokenType: 'data_exchange',
     *   expiresIn: int,
     * } $values
     */
    public function __construct(
        array $values,
    ) {
        $this->token = $values['token'];
        $this->tokenType = $values['tokenType'];
        $this->expiresIn = $values['expiresIn'];
    }

    /**
     * @return string
     */
    public function __toString(): string
    {
        return $this->toJson();
    }
}
