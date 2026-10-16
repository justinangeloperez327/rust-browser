//! WebView2 adapter: COM environment creation and controller hosting belong here.
//! The native host integration and callbacks are developed in Group 3.

/// Initial navigation target for the first hosted view.
pub const START_PAGE: &str = "https://example.com";

/// The WebView2 Evergreen Runtime must be installed on Windows.
#[must_use]
pub const fn runtime_name() -> &'static str {
    "Microsoft Edge WebView2 Evergreen Runtime"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_page_is_https() {
        assert!(START_PAGE.starts_with("https://"));
    }
}
