<?php

namespace Cameronapak\PlatformSdk\DataExchange;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalGetRequest;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use Psr\Http\Client\ClientExceptionInterface;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalPostRequest;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeTokenPostRequest;
use Cameronapak\PlatformSdk\DataExchange\Types\DataExchangeTokenPostResponse;
use JsonException;
use Psr\Http\Message\ResponseInterface;

class DataExchangeClient
{
    /**
     * @var array{
     *   baseUrl?: string,
     *   client?: ClientInterface,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     * } $options @phpstan-ignore-next-line Property is used in endpoint methods via HttpEndpointGenerator
     */
    private array $options;

    /**
     * @var RawClient $client
     */
    private RawClient $client;

    /**
     * @param RawClient $client
     * @param ?array{
     *   baseUrl?: string,
     *   client?: ClientInterface,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     * } $options
     */
    public function __construct(
        RawClient $client,
        ?array $options = null,
    ) {
        $this->client = $client;
        $this->options = $options ?? [];
    }

    /**
     * Returns the browser-rendered data exchange approval page for a short-lived token. The app
     * should first create a token with `POST /data-exchange/token`, then open this URL in the
     * user's browser with the token and app context. The page lets the user review the requested
     * permissions before approving or cancelling the exchange.
     *
     * @param DataExchangeApprovalGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return string
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function approvalGet(DataExchangeApprovalGetRequest $request, ?array $options = null): string
    {
        return $this->approvalGetWithResponse($request, $options)->getBody()->getContents();
    }

    /**
     * Returns status, headers, and the original body without following redirects.
     * @param DataExchangeApprovalGetRequest $request
     * @param ?array{baseUrl?: string, maxRetries?: int, timeout?: float, headers?: array<string, string>, queryParameters?: array<string, mixed>, bodyProperties?: array<string, mixed>} $options
     * @return ResponseInterface
     */
    public function approvalGetWithResponse(DataExchangeApprovalGetRequest $request, ?array $options = null): ResponseInterface
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        $query['token'] = $request->token;
        if ($request->xYvpAppKey !== null) {
            $query['x-yvp-app-key'] = $request->xYvpAppKey;
        }
        if ($request->xYvpAppId !== null) {
            $query['x-yvp-app-id'] = $request->xYvpAppId;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "data-exchange",
                    method: HttpMethod::GET,
                    query: $query,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                return $response;
            }
        } catch (ClientExceptionInterface $e) {
            throw new CameronapakException(message: 'HTTP transport failed', previous: $e);
        }
        throw new CameronapakApiException(
            message: 'API request failed',
            statusCode: $statusCode,
            body: $response->getBody()->getContents(),
            headers: $response->getHeaders(),
        );
    }

    /**
     * Completes the browser approval flow and redirects the browser to the app's configured
     * callback URL. When approval succeeds, the callback receives
     * `data_exchange_status=granted` and `granted_permissions`. When the user cancels from the
     * approval page, the callback receives `data_exchange_status=cancelled`,
     * `denied_permissions`, and `error=access_denied`. Recoverable errors after a safe callback
     * is known redirect with `data_exchange_status=error`, `error`, and `error_description`.
     *
     * @param DataExchangeApprovalPostRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function approvalPost(DataExchangeApprovalPostRequest $request = new DataExchangeApprovalPostRequest(), ?array $options = null): void
    {
        $this->approvalPostWithResponse($request, $options);
    }

    /**
     * Returns status, headers, and the original body without following redirects.
     * @param DataExchangeApprovalPostRequest $request
     * @param ?array{baseUrl?: string, maxRetries?: int, timeout?: float, headers?: array<string, string>, queryParameters?: array<string, mixed>, bodyProperties?: array<string, mixed>} $options
     * @return ResponseInterface
     */
    public function approvalPostWithResponse(DataExchangeApprovalPostRequest $request = new DataExchangeApprovalPostRequest(), ?array $options = null): ResponseInterface
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->token !== null) {
            $query['token'] = $request->token;
        }
        if ($request->xYvpAppKey !== null) {
            $query['x-yvp-app-key'] = $request->xYvpAppKey;
        }
        if ($request->xYvpAppId !== null) {
            $query['x-yvp-app-id'] = $request->xYvpAppId;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "data-exchange",
                    method: HttpMethod::POST,
                    query: $query,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                return $response;
            }
        } catch (ClientExceptionInterface $e) {
            throw new CameronapakException(message: 'HTTP transport failed', previous: $e);
        }
        throw new CameronapakApiException(
            message: 'API request failed',
            statusCode: $statusCode,
            body: $response->getBody()->getContents(),
            headers: $response->getHeaders(),
        );
    }

    /**
     * Creates a short-lived token that can be passed to the `/data-exchange` browser flow as a
     * query parameter. Send `requested_permissions` in the request body to declare which
     * permissions the user should review during the browser flow. Tokens expire after five minutes
     * and are consumed when the form is submitted.
     *
     * @param DataExchangeTokenPostRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?DataExchangeTokenPostResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function tokenPost(DataExchangeTokenPostRequest $request, ?array $options = null): ?DataExchangeTokenPostResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->xYvpAppKey !== null) {
            $query['x-yvp-app-key'] = $request->xYvpAppKey;
        }
        if ($request->xYvpAppId !== null) {
            $query['x-yvp-app-id'] = $request->xYvpAppId;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "data-exchange/token",
                    method: HttpMethod::POST,
                    query: $query,
                    body: $request,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                $json = $response->getBody()->getContents();
                if (empty($json)) {
                    return null;
                }
                return DataExchangeTokenPostResponse::fromJson($json);
            }
        } catch (JsonException $e) {
            throw new CameronapakException(message: 'Failed to deserialize response', previous: $e);
        } catch (ClientExceptionInterface $e) {
            throw new CameronapakException(message: 'HTTP transport failed', previous: $e);
        }
        throw new CameronapakApiException(
            message: 'API request failed',
            statusCode: $statusCode,
            body: $response->getBody()->getContents(),
            headers: $response->getHeaders(),
        );
    }
}
