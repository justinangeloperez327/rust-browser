//! Windows-native WebView2 host, isolated from engine-neutral domain types.

use browser_engine::{BrowserCommand, EngineCapabilities, EngineDescriptor, EngineKind};

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
            reload: false,
            history: false,
            suspension: false,
            downloads: false,
            private_profiles: false,
        }
    }
}

/// Dispatch failure is distinguished from a command that is not wired yet.
#[derive(Debug, PartialEq, Eq)]
pub enum NavigationDispatchError<E> {
    Unsupported,
    Engine(E),
}

/// Executes navigation through the concrete engine on its owning UI thread.
/// Back/forward/reload/stop are deliberately rejected until wired.
pub fn dispatch_navigation<E>(
    command: BrowserCommand,
    mut navigate: impl FnMut(&str) -> Result<(), E>,
) -> Result<(), NavigationDispatchError<E>> {
    match command {
        BrowserCommand::Navigate(url) => {
            let url = url.trim();
            if url.is_empty() {
                return Err(NavigationDispatchError::Unsupported);
            }
            navigate(url).map_err(NavigationDispatchError::Engine)
        }
        _ => Err(NavigationDispatchError::Unsupported),
    }
}

#[cfg(windows)]
pub fn run() -> windows_webview::Result<()> {
    use windows_webview::WebViewWindow;

    WebViewWindow::new("RustBrowser")
        .size(1200, 800)
        .run(|host| {
            let webview = host.webview();
            let completed_view = webview.clone();
            let registrations = vec![
                webview.on_navigation_completed(move |args| {
                    let event = browser_engine::BrowserEvent::NavigationCompleted {
                        url: completed_view.source(),
                        success: args.is_success(),
                    };
                    tracing::info!(?event, "WebView2 navigation completed");
                })?,
                webview.on_document_title_changed(|title| {
                    let event = browser_engine::BrowserEvent::TitleChanged(title.to_owned());
                    tracing::info!(?event, "WebView2 title changed");
                })?,
                webview.on_process_failed(|args| {
                    tracing::error!(kind = ?args.kind(), "WebView2 process failure");
                })?,
            ];
            host.retain_all(registrations);
            dispatch_navigation(BrowserCommand::Navigate(START_PAGE.to_owned()), |url| {
                webview.navigate(url)
            })
            .map_err(|error| match error {
                NavigationDispatchError::Engine(error) => error,
                NavigationDispatchError::Unsupported => unreachable!("startup uses Navigate"),
            })?;
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
        assert!(!engine.capabilities().reload);
    }

    #[test]
    fn dispatches_navigation_target() {
        let mut received = String::new();
        let result = dispatch_navigation(
            BrowserCommand::Navigate(" https://example.com ".into()),
            |url| {
                received = url.to_owned();
                Ok::<_, &'static str>(())
            },
        );
        assert_eq!(result, Ok(()));
        assert_eq!(received, "https://example.com");
    }

    #[test]
    fn rejects_unsupported_commands_without_invoking_engine() {
        let mut called = false;
        let result = dispatch_navigation(BrowserCommand::Reload, |_| {
            called = true;
            Ok::<_, &'static str>(())
        });
        assert_eq!(result, Err(NavigationDispatchError::Unsupported));
        assert!(!called);
    }
}
