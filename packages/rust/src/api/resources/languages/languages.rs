use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LanguagesClient {
    pub http_client: HttpClient,
}

impl LanguagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a collection of language objects. Add the Country parameter to filter to prominent languages for that country. Send an Accept-Language header to control which name is returned in each language's localized_name; the negotiated locale is echoed in the Content-Language response header. Each object's display_names map carries a name in every supported locale and is therefore large, so request fields[] without display_names (and use page_size) when you only need localized_name.
    ///
    /// # Arguments
    ///
    /// * `page_size` - The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    /// * `fields_array` - A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    /// * `page_token` - The page token to retrieve results from.
    /// * `country` - The ISO 3166 2 character country code
    /// * `bibles_available` - Filter languages based on whether Bible content is available for that language.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1languages_collection_get(
        &self,
        request: &V1LanguagesCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<V1LanguagesCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/languages",
                None,
                QueryBuilder::new()
                    .serialize("page_size", request.page_size.clone())
                    .string_array("fields[]", request.fields_array.clone())
                    .string("page_token", request.page_token.clone())
                    .string("country", request.country.clone())
                    .bool("bibles_available", request.bibles_available.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a single language resource by its BCP47 language code. Send an Accept-Language header to control which name is returned in localized_name; the negotiated locale is echoed in the Content-Language response header. The display_names map carries a name in every supported locale and is therefore large, so request fields[] without display_names when you only need localized_name.
    ///
    /// # Arguments
    ///
    /// * `language_id` - The language identifier uses the canonical BCP 47 language code, optionally including the script subtag when it distinguishes writing systems (for example, sr-Latn vs sr-Cyrl). Region subtags are excluded because they usually represent contextual or user-specific preferences rather than the intrinsic identity of the language, with one exception: a small, enumerated set of region variants that speakers treat as distinct languages is preserved (es-419, es-ES, pt-BR, pt-PT, zh-Hant-HK, zh-Hant-TW). Every other region, plus variants and extensions, is excluded, and a request for one redirects to the canonical region-agnostic id. This keeps identifiers stable and minimal while still surfacing the variants users distinguish.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1languages_resource_get(
        &self,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<V1LanguagesResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/languages/{}", crate::encode_path_segment(language_id)?),
                None,
                None,
                options,
            )
            .await
    }
}
