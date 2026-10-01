<?php

namespace Cameronapak\PlatformSdk;

use Cameronapak\PlatformSdk\DataExchange\DataExchangeClient;
use Cameronapak\PlatformSdk\Bibles\BiblesClient;
use Cameronapak\PlatformSdk\Highlights\HighlightsClient;
use Cameronapak\PlatformSdk\Fonts\FontsClient;
use Cameronapak\PlatformSdk\SearchQueries\SearchQueriesClient;
use Cameronapak\PlatformSdk\SearchUnified\SearchUnifiedClient;
use Cameronapak\PlatformSdk\SearchTopics\SearchTopicsClient;
use Cameronapak\PlatformSdk\SearchVerses\SearchVersesClient;
use Cameronapak\PlatformSdk\Languages\LanguagesClient;
use Cameronapak\PlatformSdk\Licenses\LicensesClient;
use Cameronapak\PlatformSdk\Apps\AppsClient;
use Cameronapak\PlatformSdk\Permissions\PermissionsClient;
use Cameronapak\PlatformSdk\Organizations\OrganizationsClient;
use Cameronapak\PlatformSdk\VerseOfTheDays\VerseOfTheDaysClient;
use Psr\Http\Client\ClientInterface;
use Cameronapak\PlatformSdk\Core\Client\RawClient;

class PlatformClient
{
    /**
     * @var DataExchangeClient $dataExchange
     */
    public DataExchangeClient $dataExchange;

    /**
     * @var BiblesClient $bibles
     */
    public BiblesClient $bibles;

    /**
     * @var HighlightsClient $highlights
     */
    public HighlightsClient $highlights;

    /**
     * @var FontsClient $fonts
     */
    public FontsClient $fonts;

    /**
     * @var SearchQueriesClient $searchQueries
     */
    public SearchQueriesClient $searchQueries;

    /**
     * @var SearchUnifiedClient $searchUnified
     */
    public SearchUnifiedClient $searchUnified;

    /**
     * @var SearchTopicsClient $searchTopics
     */
    public SearchTopicsClient $searchTopics;

    /**
     * @var SearchVersesClient $searchVerses
     */
    public SearchVersesClient $searchVerses;

    /**
     * @var LanguagesClient $languages
     */
    public LanguagesClient $languages;

    /**
     * @var LicensesClient $licenses
     */
    public LicensesClient $licenses;

    /**
     * @var AppsClient $apps
     */
    public AppsClient $apps;

    /**
     * @var PermissionsClient $permissions
     */
    public PermissionsClient $permissions;

    /**
     * @var OrganizationsClient $organizations
     */
    public OrganizationsClient $organizations;

    /**
     * @var VerseOfTheDaysClient $verseOfTheDays
     */
    public VerseOfTheDaysClient $verseOfTheDays;

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
     * @param string $yvpAppKey
     * @param ?string $token The token to use for authentication.
     * @param ?array{
     *   baseUrl?: string,
     *   client?: ClientInterface,
     *   maxRetries?: int,
     *   timeout?: float,
     *   headers?: array<string, string>,
     * } $options
     */
    public function __construct(
        string $yvpAppKey,
        ?string $token = null,
        ?array $options = null,
    ) {
        $defaultHeaders = [
            'X-YVP-App-Key' => $yvpAppKey,
            'X-Fern-Language' => 'PHP',
            'X-Fern-SDK-Name' => 'Cameronapak\PlatformSdk',
        ];
        if ($token != null) {
            $defaultHeaders['Authorization'] = "Bearer $token";
        }

        $this->options = $options ?? [];

        $this->options['headers'] = array_merge(
            $defaultHeaders,
            $this->options['headers'] ?? [],
        );

        $this->client = new RawClient(
            options: $this->options,
        );

        $this->dataExchange = new DataExchangeClient($this->client, $this->options);
        $this->bibles = new BiblesClient($this->client, $this->options);
        $this->highlights = new HighlightsClient($this->client, $this->options);
        $this->fonts = new FontsClient($this->client, $this->options);
        $this->searchQueries = new SearchQueriesClient($this->client, $this->options);
        $this->searchUnified = new SearchUnifiedClient($this->client, $this->options);
        $this->searchTopics = new SearchTopicsClient($this->client, $this->options);
        $this->searchVerses = new SearchVersesClient($this->client, $this->options);
        $this->languages = new LanguagesClient($this->client, $this->options);
        $this->licenses = new LicensesClient($this->client, $this->options);
        $this->apps = new AppsClient($this->client, $this->options);
        $this->permissions = new PermissionsClient($this->client, $this->options);
        $this->organizations = new OrganizationsClient($this->client, $this->options);
        $this->verseOfTheDays = new VerseOfTheDaysClient($this->client, $this->options);
    }
}
