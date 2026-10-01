<?php

namespace Cameronapak\PlatformSdk\DataExchange\Requests;

use Cameronapak\PlatformSdk\Core\Json\JsonSerializableType;

class DataExchangeApprovalPostRequest extends JsonSerializableType
{
    /**
     * @var ?string $token Short-lived data exchange token created by `POST /data-exchange/token`.
     */
    public ?string $token;

    /**
     * @var ?string $xYvpAppKey Public app key used to resolve the app when a token is not supplied.
     */
    public ?string $xYvpAppKey;

    /**
     * @var ?string $xYvpAppId App identifier used when a public app key is not supplied.
     */
    public ?string $xYvpAppId;

    /**
     * @param array{
     *   token?: ?string,
     *   xYvpAppKey?: ?string,
     *   xYvpAppId?: ?string,
     * } $values
     */
    public function __construct(
        array $values = [],
    ) {
        $this->token = $values['token'] ?? null;
        $this->xYvpAppKey = $values['xYvpAppKey'] ?? null;
        $this->xYvpAppId = $values['xYvpAppId'] ?? null;
    }
}
