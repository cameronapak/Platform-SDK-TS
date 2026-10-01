<?php

namespace Cameronapak\PlatformSdk\Languages;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\Languages\Requests\V1LanguagesCollectionGetRequest;
use Cameronapak\PlatformSdk\Languages\Types\V1LanguagesCollectionGetResponse;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use JsonException;
use Psr\Http\Client\ClientExceptionInterface;
use Cameronapak\PlatformSdk\Languages\Types\V1LanguagesResourceGetResponse;

class LanguagesClient
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
     * Get a collection of language objects. Add the Country parameter to filter to prominent languages for that country. Send an Accept-Language header to control which name is returned in each language's localized_name; the negotiated locale is echoed in the Content-Language response header. Each object's display_names map carries a name in every supported locale and is therefore large, so request fields[] without display_names (and use page_size) when you only need localized_name.
     *
     * @param V1LanguagesCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1LanguagesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1LanguagesCollectionGet(V1LanguagesCollectionGetRequest $request, ?array $options = null): ?V1LanguagesCollectionGetResponse
    {
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
        if ($request->country !== null) {
            $query['country'] = $request->country;
        }
        if ($request->biblesAvailable !== null) {
            $query['bibles_available'] = $request->biblesAvailable;
        }
        $headers = [];
        $headers['Accept-Language'] = 'en';
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/languages",
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
                return V1LanguagesCollectionGetResponse::fromJson($json);
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
     * Get a single language resource by its BCP47 language code. Send an Accept-Language header to control which name is returned in localized_name; the negotiated locale is echoed in the Content-Language response header. The display_names map carries a name in every supported locale and is therefore large, so request fields[] without display_names when you only need localized_name.
     *
     * @param string $languageId The language identifier uses the canonical BCP 47 language code, optionally including the script subtag when it distinguishes writing systems (for example, sr-Latn vs sr-Cyrl). Region subtags are excluded because they usually represent contextual or user-specific preferences rather than the intrinsic identity of the language, with one exception: a small, enumerated set of region variants that speakers treat as distinct languages is preserved (es-419, es-ES, pt-BR, pt-PT, zh-Hant-HK, zh-Hant-TW). Every other region, plus variants and extensions, is excluded, and a request for one redirects to the canonical region-agnostic id. This keeps identifiers stable and minimal while still surfacing the variants users distinguish.
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?V1LanguagesResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function v1LanguagesResourceGet(string $languageId, ?array $options = null): ?V1LanguagesResourceGetResponse
    {
        $languageId = RawClient::encodePathSegment($languageId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/languages/{$languageId}",
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
                return V1LanguagesResourceGetResponse::fromJson($json);
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
