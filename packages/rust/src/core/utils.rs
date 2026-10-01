/// URL building utilities
/// Safely join a base URL with a path, handling slashes properly
///
/// # Examples
/// ```
/// use example_api::utils::url::join_url;
///
/// assert_eq!(join_url("https://api.example.com", "users"), "https://api.example.com/users");
/// assert_eq!(join_url("https://api.example.com/", "users"), "https://api.example.com/users");
/// assert_eq!(join_url("https://api.example.com", "/users"), "https://api.example.com/users");
/// assert_eq!(join_url("https://api.example.com/", "/users"), "https://api.example.com/users");
/// ```
pub fn join_url(base_url: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_url_no_slashes() {
        assert_eq!(
            join_url("https://api.example.com", "users"),
            "https://api.example.com/users"
        );
    }

    #[test]
    fn test_join_url_trailing_slash_on_base() {
        assert_eq!(
            join_url("https://api.example.com/", "users"),
            "https://api.example.com/users"
        );
    }

    #[test]
    fn test_join_url_leading_slash_on_path() {
        assert_eq!(
            join_url("https://api.example.com", "/users"),
            "https://api.example.com/users"
        );
    }

    #[test]
    fn test_join_url_both_slashes() {
        assert_eq!(
            join_url("https://api.example.com/", "/users"),
            "https://api.example.com/users"
        );
    }

    #[test]
    fn test_join_url_multi_segment_path() {
        assert_eq!(
            join_url("https://api.example.com", "v1/users/123"),
            "https://api.example.com/v1/users/123"
        );
    }

    #[test]
    fn test_join_url_empty_path() {
        assert_eq!(
            join_url("https://api.example.com", ""),
            "https://api.example.com/"
        );
    }

    #[test]
    fn test_join_url_empty_base() {
        assert_eq!(join_url("", "users"), "/users");
    }
}

/// Encode a caller-provided path parameter as one segment.
/// Reject dot segments before URL parsing can normalize them into another path.
pub fn encode_path_segment(value: &str) -> Result<String, crate::ApiError> {
    if matches!(value, "." | "..") {
        return Err(crate::ApiError::Configuration(
            "path parameters cannot be dot segments".into(),
        ));
    }
    const RESERVED: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    Ok(percent_encoding::utf8_percent_encode(value, RESERVED).to_string())
}
