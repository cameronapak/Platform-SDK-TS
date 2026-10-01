import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { replaceExactly } from '../../scripts/generation.mjs';

// Preserve the pinned Fern runtime. Every compatibility edit checks its input
// so regeneration stops rather than silently accepting changed upstream code.
export function adaptRust(directory, specFile) {
  function edit(file, transform) {
    const path = join(directory, file);
    const replace = (source, search, replacement, count = 1) =>
      replaceExactly(source, search, replacement, count, path);
    writeFileSync(path, transform(readFileSync(path, 'utf8'), replace));
  }

  edit('Cargo.toml', (source, replace) => {
    source = replace(source, 'documentation = "https://docs.rs/cameronapak_platform_sdk"\n', '');
    return replace(source, 'edition = "2021"',
      'edition = "2021"\npublish = false\ninclude = ["src/**", "Cargo.toml", "Cargo.lock", "LICENSE", "README.md", "sdk-map.json"]').trimEnd() + '\n';
  });
  edit('src/lib.rs', (source, replace) => replace(source,
    '//! The official Rust SDK for the YouVersion Platform API.',
    '//! An experimental, unofficial Rust SDK for the YouVersion Platform API.'));
  edit('src/config.rs', (source, replace) => {
    source = replace(source, '#[derive(Debug, Clone)]', '#[derive(Clone)]');
    source = replace(source, '    pub base_url: String,', `    /// Builds both regular and no-redirect transports with the same TLS/proxy settings.
    /// Do not inject origin API credentials through reqwest defaults; they cannot be removed.
    /// Use SDK authentication and custom headers so approval suppression can apply.
    pub transport: Option<std::sync::Arc<dyn Fn() -> reqwest::ClientBuilder + Send + Sync>>,
    pub base_url: String,`);
    return replace(source, '            base_url: Environment::default()',
      '            transport: None,\n            base_url: Environment::default()');
  });
  edit('src/client.rs', (source, replace) => replace(source,
    '    /// Set the request timeout', `    /// Configure reqwest without replacing SDK authentication, headers, or retries.
    /// The factory builds both regular and redirect-disabled approval clients.
    /// Configure origin API credentials through SDK authentication and header settings.
    /// Default Authorization or other origin credential injection is unsupported:
    /// reqwest can add those headers after suppression, and the SDK cannot remove them.
    pub fn transport(mut self, factory: impl Fn() -> reqwest::ClientBuilder + Send + Sync + 'static) -> Self {
        self.config.transport = Some(std::sync::Arc::new(factory));
        self
    }

    /// Set the request timeout`));

  edit('src/core/utils.rs', (source) => source + `
/// Encode a caller-provided path parameter as one segment.
/// Reject dot segments before URL parsing can normalize them into another path.
pub fn encode_path_segment(value: &str) -> Result<String, crate::ApiError> {
    if matches!(value, "." | "..") {
        return Err(crate::ApiError::Configuration("path parameters cannot be dot segments".into()));
    }
    const RESERVED: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-').remove(b'_').remove(b'.').remove(b'~');
    Ok(percent_encoding::utf8_percent_encode(value, RESERVED).to_string())
}
`);
  edit('src/core/mod.rs', (source, replace) => replace(source,
    'pub use utils::join_url;', 'pub use utils::{join_url, encode_path_segment};'));

  edit('src/core/http_client.rs', (source, replace) => {
    source = replace(source, '    client: Client,', '    client: Client,\n    approval_client: Client,');
    source = replace(source, `        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(ApiError::Network)?;`, `        let builder = || config.transport.as_ref().map(|factory| factory()).unwrap_or_else(Client::builder)
            .timeout(config.timeout).user_agent(&config.user_agent);
        let client = builder().build().map_err(ApiError::Network)?;
        let approval_client = builder().redirect(reqwest::redirect::Policy::none())
            .build().map_err(ApiError::Network)?;`);
    source = replace(source, '            executor: None,', '            executor: None,\n            approval_client,');
    source = replace(source, '            executor: Some(executor),',
      '            executor: Some(executor),\n            approval_client: Client::new(),');
    source = replace(source, `            let mut req = req;
            self.apply_auth_headers(&mut req, options).await?;
            self.apply_custom_headers(&mut req, options)?;`, `            let mut req = req;
            if let Some(seconds) = options.as_ref().and_then(|options| options.timeout_seconds) {
                *req.timeout_mut() = Some(std::time::Duration::from_secs(seconds));
            }
            // Generated English locale defaults must precede caller headers.
            let segments: Vec<_> = req.url().path().rsplit('/').take(2).collect();
            if segments.iter().any(|segment| *segment == "languages" || *segment == "organizations") {
                req.headers_mut().insert("Accept-Language", HeaderValue::from_static("en"));
            }
            self.apply_custom_headers(&mut req, options)?;
            let approval = req.url().path().ends_with("/data-exchange");
            let suppress_bearer = approval && (req.method() == Method::GET ||
                req.url().query_pairs().any(|(name, _)| name == "token"));
            // Suppression includes caller headers and never acquires an OAuth token.
            if suppress_bearer {
                req.headers_mut().remove("Authorization");
            }
            self.apply_auth_headers(&mut req, options, !suppress_bearer).await?;
            if req.url().path().ends_with("/stylesheet") {
                if let Some(key) = options.as_ref().and_then(|options| options.api_key.as_ref()).or(self.config.api_key.as_ref()) {
                    let mut query: Vec<_> = req.url().query_pairs().filter(|(name, _)| name != "app_key")
                        .map(|(name, value)| (name.into_owned(), value.into_owned())).collect();
                    query.push(("app_key".into(), key.clone()));
                    req.url_mut().query_pairs_mut().clear().extend_pairs(query);
                }
            }`);
    source = replace(source, `    async fn apply_auth_headers(
        &self,
        request: &mut Request,
        options: &Option<RequestOptions>,
    )`, `    async fn apply_auth_headers(
        &self,
        request: &mut Request,
        options: &Option<RequestOptions>,
        include_bearer: bool,
    )`);
    source = replace(source, '                "api_key",', '                "X-YVP-App-Key",');
    source = replace(source, '        // Apply bearer token - priority:',
      '        if !include_bearer { return Ok(()); }\n\n        // Apply bearer token - priority:');
    source = replace(source, '        let mut last_error = None;', `        let approval = request.url().path().ends_with("/data-exchange");
        let client = if approval { &self.approval_client } else { &self.client };
        let mut last_error = None;`);
    source = replace(source, 'match self.client.execute(cloned_request).await {',
      'match client.execute(cloned_request).await {');
    source = replace(source, 'Ok(response) if response.status().is_success() => return Ok(response),',
      'Ok(response) if response.status().is_success() || (approval && response.status().as_u16() == 303) => return Ok(response),');
    source = replace(source, 'std::time::Duration::from_millis(100 * 2_u64.pow(attempt))', 'Self::retry_delay(attempt)', 2);
    source = replace(source, '    fn is_retryable_status(status_code: u16) -> bool {', `    fn retry_delay(attempt: u32) -> std::time::Duration {
        // Preserve native delays for normal retries; saturate before arithmetic overflow.
        std::time::Duration::from_millis(100_u64.saturating_mul(2_u64.checked_pow(attempt).unwrap_or(u64::MAX)).min(30_000))
    }

    fn is_retryable_status(status_code: u16) -> bool {`);
    source = replace(source, `        let status = response.status().as_u16();
        let text = response.text().await.map_err(ApiError::Network)?;

        if text.is_empty() {
            if status >= 400 {
                return Err(ApiError::Http {
                    status,
                    message: String::new(),
                });
            }
            return serde_json::from_value(serde_json::Value::Null).map_err(|_| ApiError::Http {
                status,
                message: String::new(),
            });
        }

        serde_json::from_str(&text).map_err(ApiError::Serialization)`,
      '        Ok(self.parse_response_raw(response).await?.body)');
    // Required text retains an empty string; optional and unit responses retain null.
    source = replace(source, '        if text.is_empty() {',
      '        if text.is_empty() && (std::any::type_name::<T>() != std::any::type_name::<String>() || status_code >= 400) {');
    source = replace(source, '        let body: T = serde_json::from_str(&text).map_err(ApiError::Serialization)?;', `        let value = if std::any::type_name::<T>() == std::any::type_name::<()>() &&
            ((200..300).contains(&status_code) || status_code == 303) {
            // No body is declared for this operation, even when a redirect sends text.
            serde_json::Value::Null
        } else if std::any::type_name::<T>() == std::any::type_name::<String>() ||
            std::any::type_name::<T>() == std::any::type_name::<Option<String>>() {
            serde_json::Value::String(text)
        } else {
            serde_json::from_str(&text).map_err(ApiError::Serialization)?
        };
        let body: T = serde_json::from_value(value).map_err(ApiError::Serialization)?;`);
    // reqwest preserves source/error classification while dropping credential-bearing URLs.
    source = replace(source, 'ApiError::Network', 'ApiError::network', 15);
    source = replace(source, 'ApiError::Http {\n', 'ApiError::Http {\n                    body: serde_json::Value::Null,\n', 3);
    return source;
  });

  edit('src/error.rs', (source, replace) => {
    source = replace(source, '        message: String,', '        body: serde_json::Value,\n        message: String,', 4);
    source = replace(source, 'Http { status: u16, message: String },',
      'Http { status: u16, message: String, body: serde_json::Value },');
    source = replace(source, '    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {', `    pub fn network(error: reqwest::Error) -> Self {
        Self::Network(error.without_url())
    }

    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::BadRequestError { .. } => Some(400),
            Self::UnauthorizedError { .. } => Some(401),
            Self::NotFoundError { .. } => Some(404),
            Self::UnprocessableEntityError { .. } => Some(422),
            Self::Http { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&serde_json::Value> {
        match self {
            Self::BadRequestError { body, .. } | Self::UnauthorizedError { body, .. } |
            Self::NotFoundError { body, .. } | Self::UnprocessableEntityError { body, .. } |
            Self::Http { body, .. } => Some(body),
            _ => None,
        }
    }

    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        let response_body = body.map(|text| serde_json::from_str(text)
            .unwrap_or_else(|_| serde_json::Value::String(text.into()))).unwrap_or(serde_json::Value::Null);`);
    for (const variant of ['BadRequestError', 'UnauthorizedError', 'NotFoundError', 'UnprocessableEntityError']) {
      source = replace(source, `return Self::${variant} {`, `return Self::${variant} {\n                            body: response_body.clone(),`, 2);
    }
    return replace(source, '_ => Self::Http {', '_ => Self::Http {\n                body: response_body,');
  });

  const resources = join(directory, 'src/api/resources');
  let pathEdits = 0;
  let searchEdits = 0;
  for (const resource of readdirSync(resources).filter((name) => name !== 'mod.rs')) {
    edit(`src/api/resources/${resource}/${resource}.rs`, (source, replace) => {
      // Every string path argument comes from a generated method signature.
      source = source.replace(/    pub async fn \w+\(([\s\S]*?)\n    \}/g, (block, signature) => {
        const args = [...signature.split(') ->')[0].matchAll(/\b(\w+): &str/g)].map((match) => match[1]);
        if (!args.length) return block;
        let count = 0;
        block = block.replace(/&format!\(([\s\S]*?)\)/g, (format) => {
          count++;
          for (const argument of args) format = replace(format, argument, `crate::encode_path_segment(${argument})?`);
          return format;
        });
        if (count !== 1) throw new Error(`Rust path expression ${resource} changed`);
        pathEdits += args.length;
        return block;
      });
      // Fern's name-based heuristic parses literal search text into unrelated parameters.
      if (resource.startsWith('search_')) {
        source = replace(source, '.structured_query("query", request.query.clone())', '.string("query", request.query.clone())');
        searchEdits++;
      }
      if (resource === 'data_exchange') {
        source = replace(source, ') -> Result<String, ApiError> {', ') -> Result<Option<String>, ApiError> {');
        // Only approval endpoints need public raw-response variants for callback metadata.
        const methods = [...source.matchAll(/    pub async fn (approval_\w+)\(([\s\S]*?)\n    \}/g)];
        if (methods.length !== 2) throw new Error('Rust approval method count changed');
        for (const match of methods) {
          let raw = match[0].replace(`fn ${match[1]}(`, `fn ${match[1]}_with_raw_response(`);
          raw = replace(raw, 'Result<', 'Result<crate::RawResponse<');
          raw = replace(raw, ', ApiError>', '>, ApiError>');
          raw = replace(raw, '.execute_request(', '.execute_request_raw(');
          const end = source.lastIndexOf('\n}');
          if (end < 0) throw new Error('Rust resource implementation boundary changed');
          source = source.slice(0, end) + '\n\n' + raw + source.slice(end);
        }
      }
      return source;
    });
  }
  if (pathEdits !== 16) throw new Error(`Expected 16 Rust string path adaptations, found ${pathEdits}`);
  if (searchEdits !== 4) throw new Error(`Expected 4 Rust search query adaptations, found ${searchEdits}`);

  // The pinned builder requires optional query arrays even though Serde defaults them.
  // Preserve required language ranges and body arrays; check optionality against OpenAPI.
  const spec = JSON.parse(readFileSync(specFile, 'utf8'));
  for (const [file, operationId, fields] of [
    ['collection_get_query_request', 'bibles.collection_get', ['fields']],
    ['v1_search_unified_collection_get_query_request', 'v1.search_unified.collection_get', ['fields']],
    ['v1_languages_collection_get_query_request', 'v1.languages.collection_get', ['fields']],
    ['v1_organizations_collection_get_query_request', 'v1.organizations.collection_get', ['bible_ids', 'fields']],
    ['v1_organizations_bibles_collection_get_query_request', 'v1.organizations.bibles.collection_get', ['fields']],
  ]) {
    const operation = Object.values(spec.paths).flatMap(Object.values).find((item) => item.operationId === operationId);
    edit(`src/api/types/${file}.rs`, (source, replace) => {
      for (const field of fields) {
        const parameter = operation?.parameters?.find((item) => item.name === `${field}[]`);
        if (!parameter || parameter.required || parameter.schema?.type !== 'array') throw new Error(`Optional Rust array contract changed: ${operationId}.${field}`);
        source = replace(source, `.ok_or_else(|| BuildError::missing_field("${field}_array"))?`, '.unwrap_or_default()');
        source = replace(source, `    /// - [\`${field}_array\`](${source.match(/pub struct (\w+)Builder/)?.[1]}Builder::${field}_array)\n`, '');
      }
      if (!source.includes('.ok_or_else')) source = replace(source, '    /// This method will fail if any of the following fields are not set:\n', '');
      return source;
    });
  }

  const pageSizes = [
    ['bibles_bibles_collection_get_request_page_size.rs', 'BiblesCollectionGetRequestPageSize'],
    ['languages_v_1_languages_collection_get_request_page_size.rs', 'V1LanguagesCollectionGetRequestPageSize'],
    ['organizations_v_1_organizations_collection_get_request_page_size.rs', 'V1OrganizationsCollectionGetRequestPageSize'],
    ['organizations_v_1_organizations_bibles_collection_get_request_page_size.rs', 'V1OrganizationsBiblesCollectionGetRequestPageSize'],
  ];
  for (const [file, name] of pageSizes) {
    edit(`src/api/types/${file}`, (source, replace) => {
      // The pinned generator retains only the wildcard branch of integer | "*".
      replace(source, `pub enum ${name} {\n    #[serde(rename = "*")]\n    All,\n}`, 'checked');
      return `pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ${name} { All, Numeric(u8) }

impl Serialize for ${name} {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self { Self::All => serializer.serialize_str("*"), Self::Numeric(value) => serializer.serialize_u8(*value) }
    }
}
impl<'de> Deserialize<'de> for ${name} {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(value) if value == "*" => Ok(Self::All),
            serde_json::Value::Number(value) => value.as_u64().filter(|value| (1..=99).contains(value))
                .map(|value| Self::Numeric(value as u8)).ok_or_else(|| serde::de::Error::custom("page size must be 1..=99 or *")),
            _ => Err(serde::de::Error::custom("page size must be 1..=99 or *")),
        }
    }
}
impl std::str::FromStr for ${name} {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "*" { return Ok(Self::All); }
        value.parse::<u8>().ok().filter(|value| (1..=99).contains(value))
            .map(Self::Numeric).ok_or("page size must be 1..=99 or *")
    }
}
impl fmt::Display for ${name} {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::All => formatter.write_str("*"), Self::Numeric(value) => write!(formatter, "{value}") }
    }
}
`;
    });
  }
}
