use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BiblesClient {
    pub http_client: HttpClient,
}

impl BiblesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a paginated list of Bible versions available.
    /// When multiple language_ranges parameters are specified, the set of Bibles returned will be from
    /// the first language range which has available Bibles.
    ///
    /// # Arguments
    ///
    /// * `all_available` - This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
    /// * `language_ranges_array` - An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    /// * `license_id` - Filter Bibles by a license identifier
    /// * `page_size` - The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    /// * `fields_array` - A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    /// * `page_token` - The page token to retrieve results from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn collection_get(
        &self,
        request: &CollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<BiblesCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/bibles",
                None,
                QueryBuilder::new()
                    .bool("all_available", request.all_available.clone())
                    .string_array("language_ranges[]", request.language_ranges_array.clone())
                    .int("license_id", request.license_id.clone())
                    .serialize("page_size", request.page_size.clone())
                    .string_array("fields[]", request.fields_array.clone())
                    .string("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a Bible resource for a single Bible version.
    /// This does not include the Bible's text content; use the Passages endpoint for that.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resource_get(
        &self,
        bible_id_path: i64,
        options: Option<RequestOptions>,
    ) -> Result<BiblesResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/bibles/{}", bible_id_path),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the indexing structure for the specified Bible version.  This includes the full hierarchy of
    /// books, chapters, and verse counts.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn index_collection_get(
        &self,
        bible_id_path: i64,
        options: Option<RequestOptions>,
    ) -> Result<BiblesIndexCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/bibles/{}/index", bible_id_path),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the specified scripture passage in the requested format. Headings and
    /// notes may be included via query parameters, but only when format=html. They
    /// are omitted when format=text. The response includes content text and metadata
    /// such as verse ranges and formatting details.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `passage_id_path` - The passage identifier (verse or chapter USFM format)
    /// * `format` - The desired Bible content format (text or html). Headings and notes are included only when format=html; format=text returns verse text without them.
    /// * `include_headings` - Whether or not headings should be included in the Bible content. Headings are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
    /// * `include_notes` - Whether or not notes should be included in the Bible content. Notes are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn passages_resource_get(
        &self,
        bible_id_path: i64,
        passage_id_path: &str,
        request: &PassagesResourceGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BiblesPassagesResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/passages/{}",
                    bible_id_path,
                    crate::encode_path_segment(passage_id_path)?
                ),
                None,
                QueryBuilder::new()
                    .serialize("format", request.format.clone())
                    .bool("include_headings", request.include_headings.clone())
                    .bool("include_notes", request.include_notes.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the list of books (e.g. Genesis, Exodus) for the specified Bible version.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `canon` - The Canon to filter results by
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_collection_get(
        &self,
        bible_id_path: i64,
        request: &BooksCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<BiblesBooksCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/bibles/{}/books", bible_id_path),
                None,
                QueryBuilder::new()
                    .serialize("canon", request.canon.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a Book resource. This does not include the text content; use the Passages endpoint for that.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `book_id` - The Bible Book identifier which is commonly the first 3 characters of the USFM reference
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_resource_get(
        &self,
        bible_id_path: i64,
        book_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BiblesBooksResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/books/{}",
                    bible_id_path,
                    crate::encode_path_segment(book_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a collection of Chapters for the given Bible and Book
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `book_id` - The Bible Book identifier which is commonly the first 3 characters of the USFM reference
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_chapters_collection_get(
        &self,
        bible_id_path: i64,
        book_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<BiblesBooksChaptersCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/books/{}/chapters",
                    bible_id_path,
                    crate::encode_path_segment(book_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a Chapter resource. This does not include the text content; use the Passages endpoint for that.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `book_id` - The Bible Book identifier which is commonly the first 3 characters of the USFM reference
    /// * `chapter_id` - The Bible Chapter identifier which is part of the USFM reference.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_chapters_resource_get(
        &self,
        bible_id_path: i64,
        book_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BiblesBooksChaptersResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/books/{}/chapters/{}",
                    bible_id_path,
                    crate::encode_path_segment(book_id)?,
                    crate::encode_path_segment(chapter_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a collection of Verses for a Chapter.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `book_id` - The Bible Book identifier which is commonly the first 3 characters of the USFM reference
    /// * `chapter_id` - The Bible Chapter identifier which is part of the USFM reference.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_chapters_verses_collection_get(
        &self,
        bible_id_path: i64,
        book_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<BiblesBooksChaptersVersesCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/books/{}/chapters/{}/verses",
                    bible_id_path,
                    crate::encode_path_segment(book_id)?,
                    crate::encode_path_segment(chapter_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a Verse resource. This does not include the text content; use the Passages endpoint for that.
    ///
    /// # Arguments
    ///
    /// * `bible_id_path` - The Bible version identifier
    /// * `book_id` - The Bible Book identifier which is commonly the first 3 characters of the USFM reference
    /// * `chapter_id` - The Bible Chapter identifier which is part of the USFM reference.
    /// * `verse_id` - The Bible Verse identifier pulled from part of the USFM reference
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn books_chapters_verses_resource_get(
        &self,
        bible_id_path: i64,
        book_id: &str,
        chapter_id: &str,
        verse_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BiblesBooksChaptersVersesResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/bibles/{}/books/{}/chapters/{}/verses/{}",
                    bible_id_path,
                    crate::encode_path_segment(book_id)?,
                    crate::encode_path_segment(chapter_id)?,
                    crate::encode_path_segment(verse_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }
}
