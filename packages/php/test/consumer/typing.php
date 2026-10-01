<?php

use Cameronapak\PlatformSdk\PlatformClient;
use Cameronapak\PlatformSdk\Bibles\Requests\BiblesCollectionGetRequest;
use Cameronapak\PlatformSdk\Languages\Requests\V1LanguagesCollectionGetRequest;
use Cameronapak\PlatformSdk\Highlights\Requests\V1HighlightsCollectionPostRequest;
use Cameronapak\PlatformSdk\Highlights\Types\V1HighlightsCollectionPostRequestHighlight;
use Cameronapak\PlatformSdk\DataExchange\Requests\DataExchangeApprovalPostRequest;
use Psr\Http\Message\ResponseInterface;

function example(PlatformClient $client): ?int
{
    $page = $client->bibles->collectionGet(new BiblesCollectionGetRequest([
        'languageRangesArray' => ['en', 'es-419'], 'pageSize' => 7, 'fieldsArray' => ['id'],
    ]));
    $client->bibles->collectionGet(new BiblesCollectionGetRequest(['pageSize' => '*', 'fieldsArray' => ['id']]));
    $client->languages->v1LanguagesCollectionGet(new V1LanguagesCollectionGetRequest(), ['headers' => ['Accept-Language' => 'fr-CA']]);
    $client->highlights->v1HighlightsCollectionPost(new V1HighlightsCollectionPostRequest([
        'requestId' => '20000000-0000-4000-8000-000000000001',
        'highlight' => new V1HighlightsCollectionPostRequestHighlight(['bibleId' => 111, 'passageId' => 'JHN.3.16', 'color' => 'abcdef']),
    ]));
    $response = $client->dataExchange->approvalPostWithResponse(new DataExchangeApprovalPostRequest(['token' => 'exchange-token']));
    readCallback($response);
    return $page?->data[0]->id;
}

function readCallback(ResponseInterface $response): string
{
    return $response->getHeaderLine('Location');
}
