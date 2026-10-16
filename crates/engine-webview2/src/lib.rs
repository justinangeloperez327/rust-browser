//! Native WebView2 window hosting for the initial browser prototype.
//!
//! A dedicated browser-engine interface and shared environment are planned for Group 4.
//! At this stage, windows-webview owns the native window, UI message loop,
//! controller resizing, COM apartment, and shutdown sequence.

/// Default first page.
pub const START_PAGE: &str = "https://example.com";

/// WebView2 Evergreen Runtime must be installed on the host.
#[must_use]
pub const fn runtime_name() -> &'static str {
    "Microsoft Edge WebView2 Evergreen Runtime"
}

#[cfg(windows)]
pub fn run() -> windows_webview::Result<()> {
    use windows_webview::WebViewWindow;

    WebViewWindow::new("RustBrowser")
        .size(1200, 800)
        .run(|host| {
            host.webview().navigate(START_PAGE)?;
            Ok(())
        })
}

#[cfg(not(windows))]
pub fn run() -> Result<(), &'static str> {
    Err("RustBrowser requires Windows")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_page_is_https() {
        assert!(START_PAGE.starts_with("https://"));
    }

    #[test]
    fn runtime_dependency_is_documented() {
        assert!(runtime_name().contains("WebView2"));
    }
}
