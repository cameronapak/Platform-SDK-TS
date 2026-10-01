<?php

namespace Cameronapak\PlatformSdk\Organizations;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\Organizations\Requests\V1OrganizationsCollectionGetRequest;
use Cameronapak\PlatformSdk\Organizations\Types\V1OrganizationsCollectionGetResponse;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use JsonException;
use Psr\Http\Client\ClientExceptionInterface;
use Cameronapak\PlatformSdk\Organizations\Types\V1OrganizationsResourceGetResponse;
use Cameronapak\PlatformSdk\Organizations\Requests\V1OrganizationsBiblesCollectionGetRequest;
use Cameronapak\PlatformSdk\Organizations\Types\V1OrganizationsBiblesCollectionGetResponse;

class OrganizationsClient
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
     * Returns a paginated list of Organization objects. Use bible_ids[] to filter
     * to organizations associated with the given Bible version(s); when omitted,
     * all organizations are returned.
     *
     * @param V1OrganizationsCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1OrganizationsCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1OrganizationsCollectionGet(V1OrganizationsCollectionGetRequest $request, ?array $options = null): ?V1OrganizationsCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->bibleIdsArray !== null) {
            $query['bible_ids[]'] = $request->bibleIdsArray;
        }
        if ($request->pageSize !== null) {
            $query['page_size'] = $request->pageSize;
        }
        if ($request->fieldsArray !== null) {
            $query['fields[]'] = $request->fieldsArray;
        }
        if ($request->pageToken !== null) {
            $query['page_token'] = $request->pageToken;
        }
        $headers = [];
        $headers['Accept-Language'] = 'en';
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/organizations",
                    method: HttpMethod::GET,
                    headers: $headers,
                    query: $query,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                $json = $response->getBody()->getContents();
                if (empty($json)) {
                    return null;
                }
                return V1OrganizationsCollectionGetResponse::fromJson($json);
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

    /**
     * Get a single organization resource by its id.
     *
     * @param string $organizationId The Organization unique ID provided in the Platform.
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1OrganizationsResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1OrganizationsResourceGet(string $organizationId, ?array $options = null): ?V1OrganizationsResourceGetResponse
    {
        $organizationId = RawClient::encodePathSegment($organizationId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/organizations/{$organizationId}",
                    method: HttpMethod::GET,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                $json = $response->getBody()->getContents();
                if (empty($json)) {
                    return null;
                }
                return V1OrganizationsResourceGetResponse::fromJson($json);
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

    /**
     * Get bibles associated with a specific organization by its id.
     *
     * @param string $organizationId The Organization unique ID provided in the Platform.
     * @param V1OrganizationsBiblesCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1OrganizationsBiblesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1OrganizationsBiblesCollectionGet(string $organizationId, V1OrganizationsBiblesCollectionGetRequest $request = new V1OrganizationsBiblesCollectionGetRequest(), ?array $options = null): ?V1OrganizationsBiblesCollectionGetResponse
    {
        $organizationId = RawClient::encodePathSegment($organizationId);
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->pageSize !== null) {
            $query['page_size'] = $request->pageSize;
        }
        if ($request->fieldsArray !== null) {
            $query['fields[]'] = $request->fieldsArray;
        }
        if ($request->pageToken !== null) {
            $query['page_token'] = $request->pageToken;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/organizations/{$organizationId}/bibles",
                    method: HttpMethod::GET,
                    query: $query,
                ),
                $options,
            );
            $statusCode = $response->getStatusCode();
            if ($statusCode >= 200 && $statusCode < 400) {
                $json = $response->getBody()->getContents();
                if (empty($json)) {
                    return null;
                }
                return V1OrganizationsBiblesCollectionGetResponse::fromJson($json);
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
