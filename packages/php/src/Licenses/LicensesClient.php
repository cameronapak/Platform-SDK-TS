<?php

namespace Cameronapak\PlatformSdk\Licenses;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\Licenses\Requests\V1LicensesCollectionGetRequest;
use Cameronapak\PlatformSdk\Licenses\Types\V1LicensesCollectionGetResponse;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use JsonException;
use Psr\Http\Client\ClientExceptionInterface;

class LicensesClient
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
     * Returns a list of license metadata. By default, the response returns all licenses.
     * In the case that `developer_id` is passed in the query, it filters to only include licenses
     * the developer has agreed to. In the case that `bible_id` is passed it only returns the
     * license under which that Bible ID is currently licensed. The license object also contains
     * a list of all Bible ids are offered under that license.
     * Optionally combine `developer_id` with the `all_available` query parameter to receive
     * every license while still surfacing the developer's agreement metadata when present.
     *
     * @param V1LicensesCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1LicensesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1LicensesCollectionGet(V1LicensesCollectionGetRequest $request, ?array $options = null): ?V1LicensesCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        $query['bible_id'] = $request->bibleId;
        $query['developer_id'] = $request->developerId;
        if ($request->allAvailable !== null) {
            $query['all_available'] = $request->allAvailable;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/licenses",
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
                return V1LicensesCollectionGetResponse::fromJson($json);
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
