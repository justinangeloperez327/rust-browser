//! Address and search input normalization for WebView2.

use std::fmt;
use url::{Host, Url, form_urlencoded};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    Empty,
    UnsupportedScheme,
    InvalidHost,
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for AddressError {}

fn validated_http(url: Url) -> Result<String, AddressError> {
    if !matches!(url.scheme(), "http" | "https") {
        return Err(AddressError::UnsupportedScheme);
    }
    if url.host().is_none() {
        return Err(AddressError::InvalidHost);
    }
    Ok(url.to_string())
}

fn search_url(query: &str) -> String {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .finish();
    format!("https://www.google.com/search?{encoded}")
}

/// Normalize a web address, hostname, or search term.
/// Non-HTTP(S) schemes are rejected rather than passed to WebView2.
pub fn resolve_address(input: &str) -> Result<String, AddressError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(AddressError::Empty);
    }
    if input.chars().any(char::is_control) {
        return Err(AddressError::InvalidHost);
    }

    if let Ok(url) = Url::parse(input) {
        return validated_http(url);
    }

    // Explicit schemes are never silently interpreted as search terms.
    if input.contains("://") || input.contains(':') {
        if input.contains(char::is_whitespace) {
            return Ok(search_url(input));
        }
        return Err(AddressError::UnsupportedScheme);
    }

    // A single DNS-like name, localhost or IP address can be a navigation target.
    if !input.contains(char::is_whitespace)
        && !input.contains(['/', '\\', '@', '?', '#'])
        && (input.contains('.') || input.eq_ignore_ascii_case("localhost"))
    {
        if let Ok(url) = Url::parse(&format!("https://{input}")) {
            if matches!(
                url.host(),
                Some(Host::Domain(_)) | Some(Host::Ipv4(_)) | Some(Host::Ipv6(_))
            ) {
                return validated_http(url);
            }
        }
        return Err(AddressError::InvalidHost);
    }

    Ok(search_url(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_domains_use_https() {
        assert_eq!(
            resolve_address("example.com").unwrap(),
            "https://example.com/"
        );
    }

    #[test]
    fn search_input_is_url_encoded() {
        assert_eq!(
            resolve_address("rust + webview").unwrap(),
            "https://www.google.com/search?q=rust+%2B+webview"
        );
    }

    #[test]
    fn http_url_keeps_path_and_query() {
        assert_eq!(
            resolve_address("https://example.com/a?q=1").unwrap(),
            "https://example.com/a?q=1"
        );
    }

    #[test]
    fn unsupported_schemes_are_rejected() {
        assert_eq!(
            resolve_address("file:///etc/passwd"),
            Err(AddressError::UnsupportedScheme)
        );
        assert_eq!(
            resolve_address("javascript:alert(1)"),
            Err(AddressError::UnsupportedScheme)
        );
        assert_eq!(
            resolve_address("data:text/html,hello"),
            Err(AddressError::UnsupportedScheme)
        );
    }

    #[test]
    fn malformed_http_urls_are_not_searches() {
        assert_eq!(
            resolve_address("https:///"),
            Err(AddressError::UnsupportedScheme)
        );
        assert_eq!(
            resolve_address("https://"),
            Err(AddressError::UnsupportedScheme)
        );
    }

    #[test]
    fn empty_input_is_rejected() {
        assert_eq!(resolve_address("  "), Err(AddressError::Empty));
    }

    #[test]
    fn localhost_is_accepted() {
        assert_eq!(resolve_address("localhost").unwrap(), "https://localhost/");
    }

    #[test]
    fn input_with_spaces_becomes_search() {
        assert!(
            resolve_address("best Rust browser")
                .unwrap()
                .contains("q=best+Rust+browser")
        );
    }
}
