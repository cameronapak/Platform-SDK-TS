<?php

namespace Cameronapak\PlatformSdk\SearchVerses;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\SearchVerses\Requests\V1SearchVersesCollectionGetRequest;
use Cameronapak\PlatformSdk\SearchVerses\Types\V1SearchVersesCollectionGetResponse;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use JsonException;
use Psr\Http\Client\ClientExceptionInterface;

class SearchVersesClient
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
     * Search for Bible verses by query, returning a paginated list of verse references. Verse results carry references and metadata only (e.g. JHN.3.16), never passage text; resolve verse text through the licensed-content endpoint. The bible_id determines the language searched, so this endpoint takes no language parameter. Use page_size and page_token to page through the full result set.
     *
     * @param V1SearchVersesCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1SearchVersesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1SearchVersesCollectionGet(V1SearchVersesCollectionGetRequest $request, ?array $options = null): ?V1SearchVersesCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        $query['query'] = $request->query;
        $query['bible_id'] = $request->bibleId;
        if ($request->userIntent !== null) {
            $query['user_intent'] = $request->userIntent;
        }
        if ($request->pageSize !== null) {
            $query['page_size'] = $request->pageSize;
        }
        if ($request->pageToken !== null) {
            $query['page_token'] = $request->pageToken;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/search-verses",
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
                return V1SearchVersesCollectionGetResponse::fromJson($json);
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
