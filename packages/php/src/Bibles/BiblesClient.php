<?php

namespace Cameronapak\PlatformSdk\Bibles;

use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;
use Cameronapak\PlatformSdk\Bibles\Requests\BiblesCollectionGetRequest;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesCollectionGetResponse;
use Cameronapak\PlatformSdk\Exceptions\CameronapakException;
use Cameronapak\PlatformSdk\Exceptions\CameronapakApiException;
use Cameronapak\PlatformSdk\Core\Json\JsonApiRequest;
use Cameronapak\PlatformSdk\Environments;
use Cameronapak\PlatformSdk\Core\Client\HttpMethod;
use JsonException;
use Psr\Http\Client\ClientExceptionInterface;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesResourceGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesIndexCollectionGetResponse;
use Cameronapak\PlatformSdk\Bibles\Requests\BiblesPassagesResourceGetRequest;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesPassagesResourceGetResponse;
use Cameronapak\PlatformSdk\Bibles\Requests\BiblesBooksCollectionGetRequest;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksCollectionGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksResourceGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksChaptersCollectionGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksChaptersResourceGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksChaptersVersesCollectionGetResponse;
use Cameronapak\PlatformSdk\Bibles\Types\BiblesBooksChaptersVersesResourceGetResponse;

class BiblesClient
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
     * Retrieves a paginated list of Bible versions available.
     * When multiple language_ranges parameters are specified, the set of Bibles returned will be from
     * the first language range which has available Bibles.
     *
     * @param BiblesCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function collectionGet(BiblesCollectionGetRequest $request = new BiblesCollectionGetRequest(), ?array $options = null): ?BiblesCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->allAvailable !== null) {
            $query['all_available'] = $request->allAvailable;
        }
        if ($request->languageRangesArray !== null) {
            $query['language_ranges[]'] = $request->languageRangesArray;
        }
        if ($request->licenseId !== null) {
            $query['license_id'] = $request->licenseId;
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
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles",
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
                return BiblesCollectionGetResponse::fromJson($json);
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
     * Get a Bible resource for a single Bible version.
     * This does not include the Bible's text content; use the Passages endpoint for that.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function resourceGet(int $bibleIdPath, ?array $options = null): ?BiblesResourceGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}",
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
                return BiblesResourceGetResponse::fromJson($json);
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
     * Retrieves the indexing structure for the specified Bible version.  This includes the full hierarchy of
     * books, chapters, and verse counts.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesIndexCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function indexCollectionGet(int $bibleIdPath, ?array $options = null): ?BiblesIndexCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/index",
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
                return BiblesIndexCollectionGetResponse::fromJson($json);
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
     * Returns the specified scripture passage in the requested format. Headings and
     * notes may be included via query parameters, but only when format=html. They
     * are omitted when format=text. The response includes content text and metadata
     * such as verse ranges and formatting details.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $passageIdPath The passage identifier (verse or chapter USFM format)
     * @param BiblesPassagesResourceGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesPassagesResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function passagesResourceGet(int $bibleIdPath, string $passageIdPath, BiblesPassagesResourceGetRequest $request = new BiblesPassagesResourceGetRequest(), ?array $options = null): ?BiblesPassagesResourceGetResponse
    {
        $passageIdPath = RawClient::encodePathSegment($passageIdPath);
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->format !== null) {
            $query['format'] = $request->format;
        }
        if ($request->includeHeadings !== null) {
            $query['include_headings'] = $request->includeHeadings;
        }
        if ($request->includeNotes !== null) {
            $query['include_notes'] = $request->includeNotes;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/passages/{$passageIdPath}",
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
                return BiblesPassagesResourceGetResponse::fromJson($json);
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
     * Retrieves the list of books (e.g. Genesis, Exodus) for the specified Bible version.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param BiblesBooksCollectionGetRequest $request
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksCollectionGet(int $bibleIdPath, BiblesBooksCollectionGetRequest $request = new BiblesBooksCollectionGetRequest(), ?array $options = null): ?BiblesBooksCollectionGetResponse
    {
        $options = RawClient::mergeOptions($this->options, $options);
        $query = [];
        if ($request->canon !== null) {
            $query['canon'] = $request->canon;
        }
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books",
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
                return BiblesBooksCollectionGetResponse::fromJson($json);
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
     * Get a Book resource. This does not include the text content; use the Passages endpoint for that.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $bookId The Bible Book identifier which is commonly the first 3 characters of the USFM reference
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksResourceGet(int $bibleIdPath, string $bookId, ?array $options = null): ?BiblesBooksResourceGetResponse
    {
        $bookId = RawClient::encodePathSegment($bookId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books/{$bookId}",
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
                return BiblesBooksResourceGetResponse::fromJson($json);
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
     * Get a collection of Chapters for the given Bible and Book
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $bookId The Bible Book identifier which is commonly the first 3 characters of the USFM reference
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksChaptersCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksChaptersCollectionGet(int $bibleIdPath, string $bookId, ?array $options = null): ?BiblesBooksChaptersCollectionGetResponse
    {
        $bookId = RawClient::encodePathSegment($bookId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books/{$bookId}/chapters",
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
                return BiblesBooksChaptersCollectionGetResponse::fromJson($json);
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
     * Get a Chapter resource. This does not include the text content; use the Passages endpoint for that.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $bookId The Bible Book identifier which is commonly the first 3 characters of the USFM reference
     * @param string $chapterId The Bible Chapter identifier which is part of the USFM reference.
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksChaptersResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksChaptersResourceGet(int $bibleIdPath, string $bookId, string $chapterId, ?array $options = null): ?BiblesBooksChaptersResourceGetResponse
    {
        $bookId = RawClient::encodePathSegment($bookId);
        $chapterId = RawClient::encodePathSegment($chapterId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books/{$bookId}/chapters/{$chapterId}",
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
                return BiblesBooksChaptersResourceGetResponse::fromJson($json);
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
     * Get a collection of Verses for a Chapter.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $bookId The Bible Book identifier which is commonly the first 3 characters of the USFM reference
     * @param string $chapterId The Bible Chapter identifier which is part of the USFM reference.
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksChaptersVersesCollectionGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksChaptersVersesCollectionGet(int $bibleIdPath, string $bookId, string $chapterId, ?array $options = null): ?BiblesBooksChaptersVersesCollectionGetResponse
    {
        $bookId = RawClient::encodePathSegment($bookId);
        $chapterId = RawClient::encodePathSegment($chapterId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books/{$bookId}/chapters/{$chapterId}/verses",
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
                return BiblesBooksChaptersVersesCollectionGetResponse::fromJson($json);
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
     * Get a Verse resource. This does not include the text content; use the Passages endpoint for that.
     *
     * @param int $bibleIdPath The Bible version identifier
     * @param string $bookId The Bible Book identifier which is commonly the first 3 characters of the USFM reference
     * @param string $chapterId The Bible Chapter identifier which is part of the USFM reference.
     * @param string $verseId The Bible Verse identifier pulled from part of the USFM reference
     * @param ?array{
     *   baseUrl?: string,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     *   queryParameters?: array<string, mixed>,
     *   bodyProperties?: array<string, mixed>,
     * } $options
     * @return ?BiblesBooksChaptersVersesResourceGetResponse
     * @throws CameronapakException
     * @throws CameronapakApiException
     */
    public function booksChaptersVersesResourceGet(int $bibleIdPath, string $bookId, string $chapterId, string $verseId, ?array $options = null): ?BiblesBooksChaptersVersesResourceGetResponse
    {
        $bookId = RawClient::encodePathSegment($bookId);
        $chapterId = RawClient::encodePathSegment($chapterId);
        $verseId = RawClient::encodePathSegment($verseId);
        $options = RawClient::mergeOptions($this->options, $options);
        try {
            $response = $this->client->sendRequest(
                new JsonApiRequest(
                    baseUrl: $options['baseUrl'] ?? $this->client->options['baseUrl'] ?? Environments::Default_->value,
                    path: "v1/bibles/{$bibleIdPath}/books/{$bookId}/chapters/{$chapterId}/verses/{$verseId}",
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
                return BiblesBooksChaptersVersesResourceGetResponse::fromJson($json);
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
