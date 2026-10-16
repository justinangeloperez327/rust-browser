//! Windows-native WebView2 host, isolated from engine-neutral domain types.

use browser_engine::{EngineCapabilities, EngineDescriptor, EngineKind};

pub const START_PAGE: &str = "https://example.com";

#[must_use]
pub const fn runtime_name() -> &'static str {
    "Microsoft Edge WebView2 Evergreen Runtime"
}

/// Host adapter capability declaration.
/// Advanced browser functionality must not be marked as supported prematurely.
pub struct WebView2Engine;

impl EngineDescriptor for WebView2Engine {
    fn kind(&self) -> EngineKind {
        EngineKind::WebView2
    }

    fn capabilities(&self) -> EngineCapabilities {
        EngineCapabilities {
            navigation: true,
            reload: true,
            history: true,
            suspension: false,
            downloads: false,
            private_profiles: false,
        }
    }
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
    fn homepage_is_https() {
        assert!(START_PAGE.starts_with("https://"));
    }

    #[test]
    fn capabilities_are_conservative() {
        let engine = WebView2Engine;
        assert_eq!(engine.kind(), EngineKind::WebView2);
        assert!(engine.capabilities().navigation);
        assert!(!engine.capabilities().suspension);
    }
}
