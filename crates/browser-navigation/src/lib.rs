//! Interpret address-bar and launch input without exposing privileged URL schemes.

use std::fmt;

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

/// Returns either a validated http(s) destination or a search URL.
/// Encodes search bytes into the query component.
pub fn resolve_address(input: &str) -> Result<String, AddressError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(AddressError::Empty);
    }
    if input.chars().any(char::is_control) {
        return Err(AddressError::InvalidHost);
    }
    let lower = input.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        let separator = input.find("://").expect("checked scheme");
        let rest = &input[separator + 3..];
        let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
        if authority.is_empty() || authority.contains('@') || authority.contains(' ') || authority.contains('\\') {
            return Err(AddressError::InvalidHost);
        }
        return Ok(input.to_string());
    }
    if input.contains("://") || lower.starts_with("javascript:") || lower.starts_with("file:")
        || lower.starts_with("data:") || lower.starts_with("vbscript:") {
        return Err(AddressError::UnsupportedScheme);
    }
    if !input.contains(char::is_whitespace) && input.contains('.') && !input.contains('/') && !input.contains(':') {
        return Ok(format!("https://{input}"));
    }
    let mut escaped = String::new();
    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => escaped.push(*byte as char),
            b' ' => escaped.push('+'),
            _ => escaped.push_str(&format!("%{byte:02X}")),
        }
    }
    Ok(format!("https://www.google.com/search?q={escaped}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn domains_get_https() {
        assert_eq!(resolve_address("example.com").unwrap(), "https://example.com");
    }
    #[test]
    fn search_terms_are_encoded() {
        assert_eq!(
            resolve_address("rust + webview").unwrap(),
            "https://www.google.com/search?q=rust+%2B+webview"
        );
    }
    #[test]
    fn url_preserves_path() {
        assert_eq!(resolve_address("https://example.com/a?q=1").unwrap(), "https://example.com/a?q=1");
    }
    #[test]
    fn unsafe_schemes_rejected() {
        assert_eq!(resolve_address("file:///etc/passwd"), Err(AddressError::UnsupportedScheme));
        assert_eq!(resolve_address("javascript:alert(1)"), Err(AddressError::UnsupportedScheme));
    }
    #[test]
    fn empty_host_rejected() {
        assert_eq!(resolve_address("https:///"), Err(AddressError::InvalidHost));
    }
}
