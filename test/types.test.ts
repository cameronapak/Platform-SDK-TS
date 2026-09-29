import { YouVersionPlatformClient } from '../src/index.js';

declare const client: YouVersionPlatformClient;

// @ts-expect-error language_ranges[] is required by the OpenAPI contract.
client.bibles.collectionGet({});

// @ts-expect-error language_ranges[] is required by the OpenAPI contract.
client.searchQueries.v1SearchQueriesCollectionGet({});

// @ts-expect-error language_ranges[] is required by the OpenAPI contract.
client.searchUnified.v1SearchUnifiedCollectionGet({ query: 'hope', bible_id: 3034 });

// @ts-expect-error language_ranges[] is required by the OpenAPI contract.
client.searchTopics.v1SearchTopicsCollectionGet({ query: 'hope' });
