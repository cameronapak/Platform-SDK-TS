import type { YouVersionPlatform } from './index.js';

/** Request types keyed by the OpenAPI operationId used to generate them. */
export interface SdkOperationRequestMap {
  "bibles_books_chapters_verses.collection_get": YouVersionPlatform.BiblesBooksChaptersVersesCollectionGetRequest;
  "bibles_books_chapters_verses.resource_get": YouVersionPlatform.BiblesBooksChaptersVersesResourceGetRequest;
  "bibles_books_chapters.collection_get": YouVersionPlatform.BiblesBooksChaptersCollectionGetRequest;
  "bibles_books_chapters.resource_get": YouVersionPlatform.BiblesBooksChaptersResourceGetRequest;
  "bibles_books.collection_get": YouVersionPlatform.BiblesBooksCollectionGetRequest;
  "bibles_books.resource_get": YouVersionPlatform.BiblesBooksResourceGetRequest;
  "bibles_index.collection_get": YouVersionPlatform.BiblesIndexCollectionGetRequest;
  "bibles_passages.resource_get": YouVersionPlatform.BiblesPassagesResourceGetRequest;
  "bibles.collection_get": YouVersionPlatform.BiblesCollectionGetRequest;
  "bibles.resource_get": YouVersionPlatform.BiblesResourceGetRequest;
  "data_exchange.approval_get": YouVersionPlatform.DataExchangeApprovalGetRequest;
  "data_exchange.approval_post": YouVersionPlatform.DataExchangeApprovalPostRequest;
  "data_exchange.token_post": YouVersionPlatform.DataExchangeTokenPostRequest;
  "v1.apps.permissions.collection_get": YouVersionPlatform.V1AppsPermissionsCollectionGetRequest;
  "v1.apps.resource_get": YouVersionPlatform.V1AppsResourceGetRequest;
  "v1.fonts.resource_get": YouVersionPlatform.V1FontsResourceGetRequest;
  "v1.fonts.stylesheet_get": YouVersionPlatform.V1FontsStylesheetGetRequest;
  "v1.highlights.collection_get": YouVersionPlatform.V1HighlightsCollectionGetRequest;
  "v1.highlights.collection_post": YouVersionPlatform.V1HighlightsCollectionPostRequest;
  "v1.highlights.resource_delete": YouVersionPlatform.V1HighlightsResourceDeleteRequest;
  "v1.languages.collection_get": YouVersionPlatform.V1LanguagesCollectionGetRequest;
  "v1.languages.resource_get": YouVersionPlatform.V1LanguagesResourceGetRequest;
  "v1.licenses.collection_get": YouVersionPlatform.V1LicensesCollectionGetRequest;
  "v1.organizations.bibles.collection_get": YouVersionPlatform.V1OrganizationsBiblesCollectionGetRequest;
  "v1.organizations.collection_get": YouVersionPlatform.V1OrganizationsCollectionGetRequest;
  "v1.organizations.resource_get": YouVersionPlatform.V1OrganizationsResourceGetRequest;
  "v1.search_queries.collection_get": YouVersionPlatform.V1SearchQueriesCollectionGetRequest;
  "v1.search_topics.collection_get": YouVersionPlatform.V1SearchTopicsCollectionGetRequest;
  "v1.search_unified.collection_get": YouVersionPlatform.V1SearchUnifiedCollectionGetRequest;
  "v1.search_verses.collection_get": YouVersionPlatform.V1SearchVersesCollectionGetRequest;
  "v1.verse_of_the_days.canonical.resource_get": YouVersionPlatform.V1VerseOfTheDaysCanonicalResourceGetRequest;
}

/** Query-only projections of generated SDK request types. */
export interface SdkOperationQueryMap {
  "bibles_books.collection_get": Pick<YouVersionPlatform.BiblesBooksCollectionGetRequest, Extract<"canon", keyof YouVersionPlatform.BiblesBooksCollectionGetRequest>>;
  "bibles_passages.resource_get": Pick<YouVersionPlatform.BiblesPassagesResourceGetRequest, Extract<"format" | "include_headings" | "include_notes", keyof YouVersionPlatform.BiblesPassagesResourceGetRequest>>;
  "bibles.collection_get": Pick<YouVersionPlatform.BiblesCollectionGetRequest, Extract<"all_available" | "language_ranges[]" | "license_id" | "page_size" | "fields[]" | "page_token", keyof YouVersionPlatform.BiblesCollectionGetRequest>>;
  "data_exchange.approval_get": Pick<YouVersionPlatform.DataExchangeApprovalGetRequest, Extract<"token" | "x-yvp-app-key" | "x-yvp-app-id", keyof YouVersionPlatform.DataExchangeApprovalGetRequest>>;
  "data_exchange.approval_post": Pick<YouVersionPlatform.DataExchangeApprovalPostRequest, Extract<"token" | "x-yvp-app-key" | "x-yvp-app-id", keyof YouVersionPlatform.DataExchangeApprovalPostRequest>>;
  "data_exchange.token_post": Pick<YouVersionPlatform.DataExchangeTokenPostRequest, Extract<"x-yvp-app-key" | "x-yvp-app-id", keyof YouVersionPlatform.DataExchangeTokenPostRequest>>;
  "v1.highlights.collection_get": Pick<YouVersionPlatform.V1HighlightsCollectionGetRequest, Extract<"bible_id" | "passage_id", keyof YouVersionPlatform.V1HighlightsCollectionGetRequest>>;
  "v1.highlights.resource_delete": Pick<YouVersionPlatform.V1HighlightsResourceDeleteRequest, Extract<"bible_id", keyof YouVersionPlatform.V1HighlightsResourceDeleteRequest>>;
  "v1.languages.collection_get": Pick<YouVersionPlatform.V1LanguagesCollectionGetRequest, Extract<"page_size" | "fields[]" | "page_token" | "country" | "bibles_available", keyof YouVersionPlatform.V1LanguagesCollectionGetRequest>>;
  "v1.licenses.collection_get": Pick<YouVersionPlatform.V1LicensesCollectionGetRequest, Extract<"bible_id" | "developer_id" | "all_available", keyof YouVersionPlatform.V1LicensesCollectionGetRequest>>;
  "v1.organizations.bibles.collection_get": Pick<YouVersionPlatform.V1OrganizationsBiblesCollectionGetRequest, Extract<"page_size" | "fields[]" | "page_token", keyof YouVersionPlatform.V1OrganizationsBiblesCollectionGetRequest>>;
  "v1.organizations.collection_get": Pick<YouVersionPlatform.V1OrganizationsCollectionGetRequest, Extract<"bible_ids[]" | "page_size" | "fields[]" | "page_token", keyof YouVersionPlatform.V1OrganizationsCollectionGetRequest>>;
  "v1.search_queries.collection_get": Pick<YouVersionPlatform.V1SearchQueriesCollectionGetRequest, Extract<"language_ranges[]" | "query" | "trending", keyof YouVersionPlatform.V1SearchQueriesCollectionGetRequest>>;
  "v1.search_topics.collection_get": Pick<YouVersionPlatform.V1SearchTopicsCollectionGetRequest, Extract<"query" | "language_ranges[]", keyof YouVersionPlatform.V1SearchTopicsCollectionGetRequest>>;
  "v1.search_unified.collection_get": Pick<YouVersionPlatform.V1SearchUnifiedCollectionGetRequest, Extract<"query" | "bible_id" | "language_ranges[]" | "user_intent" | "fields[]", keyof YouVersionPlatform.V1SearchUnifiedCollectionGetRequest>>;
  "v1.search_verses.collection_get": Pick<YouVersionPlatform.V1SearchVersesCollectionGetRequest, Extract<"query" | "bible_id" | "user_intent" | "page_size" | "page_token", keyof YouVersionPlatform.V1SearchVersesCollectionGetRequest>>;
}

export type SdkOperationId = keyof SdkOperationRequestMap;
export type SdkQueryOperationId = keyof SdkOperationQueryMap;

export type SdkRequest<OperationId extends SdkOperationId> =
  SdkOperationRequestMap[OperationId];

export type SdkQuery<OperationId extends SdkQueryOperationId> =
  SdkOperationQueryMap[OperationId];

