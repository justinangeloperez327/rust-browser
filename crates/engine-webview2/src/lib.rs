//! Windows-native WebView2 host, isolated from engine-neutral domain types.

use browser_engine::{
    BrowserCommand, BrowserView, EngineCapabilities, EngineDescriptor, EngineKind,
};

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

/// Minimal stateful adapter, with navigation delegated to a UI-thread-bound host.
/// Construct and use it on the engine's owning thread.
pub struct WebView2View<N> {
    navigate: N,
}

impl<N> WebView2View<N> {
    pub fn new(navigate: N) -> Self {
        Self { navigate }
    }
}

impl<N, E> BrowserView for WebView2View<N>
where
    N: FnMut(&str) -> Result<(), E>,
    E: std::error::Error,
{
    type Error = ViewError<E>;

    fn dispatch(&mut self, command: BrowserCommand) -> Result<(), Self::Error> {
        dispatch_navigation(command, &mut self.navigate).map_err(|error| match error {
            NavigationDispatchError::Unsupported => ViewError::Unsupported,
            NavigationDispatchError::Engine(error) => ViewError::Engine(error),
        })
    }
}

#[derive(Debug)]
pub enum ViewError<E> {
    Unsupported,
    Engine(E),
}

impl<E: std::fmt::Display> std::fmt::Display for ViewError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => f.write_str("command is not supported"),
            Self::Engine(error) => write!(f, "engine command failed: {error}"),
        }
    }
}

impl<E: std::error::Error> std::error::Error for ViewError<E> {}

#[cfg(windows)]
pub fn run() -> windows_webview::Result<()> {
    use browser_core::ViewState;
    use browser_engine::BrowserEvent;
    use std::sync::{Arc, Mutex};
    use windows_webview::WebViewWindow;

    WebViewWindow::new("RustBrowser")
        .size(1200, 800)
        .run(|host| {
            let webview = host.webview();
            let completed_view = webview.clone();
            let state = Arc::new(Mutex::new(ViewState::default()));
            if let Ok(mut snapshot) = state.lock() {
                snapshot.apply(BrowserEvent::Ready);
            }
            let completed_state = Arc::clone(&state);
            let title_state = Arc::clone(&state);
            let failure_state = Arc::clone(&state);
            let registrations = vec![
                webview.on_navigation_completed(move |args| {
                    let event = browser_engine::BrowserEvent::NavigationCompleted {
                        url: completed_view.source(),
                        success: args.is_success(),
                    };
                    if let Ok(mut snapshot) = completed_state.lock() {
                        snapshot.apply(event.clone());
                    }
                    tracing::info!(?event, "WebView2 navigation completed");
                })?,
                webview.on_document_title_changed(|title| {
                    let event = browser_engine::BrowserEvent::TitleChanged(title.to_owned());
                    if let Ok(mut snapshot) = title_state.lock() {
                        snapshot.apply(event.clone());
                    }
                    tracing::info!(?event, "WebView2 title changed");
                })?,
                webview.on_process_failed(|args| {
                    if let Ok(mut snapshot) = failure_state.lock() {
                        snapshot.apply(BrowserEvent::ProcessFailed);
                    }
                    tracing::error!(kind = ?args.kind(), "WebView2 process failure");
                })?,
            ];
            host.retain_all(registrations);
            webview.navigate(START_PAGE)?;
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
    #[test]
    fn view_trait_dispatches_multiple_navigation_commands() {
        let mut urls = Vec::new();
        let mut view = WebView2View::new(|url: &str| {
            urls.push(url.to_owned());
            Ok::<(), std::io::Error>(())
        });
        view.dispatch(BrowserCommand::Navigate("https://example.com".into()))
            .expect("first navigation");
        view.dispatch(BrowserCommand::Navigate("https://rust-lang.org".into()))
            .expect("second navigation");
        assert!(matches!(
            view.dispatch(BrowserCommand::Back),
            Err(ViewError::Unsupported)
        ));
        drop(view);
        assert_eq!(urls, ["https://example.com", "https://rust-lang.org"]);
    }

    #[test]
    fn view_trait_propagates_engine_error() {
        let mut view =
            WebView2View::new(|_: &str| Err::<(), _>(std::io::Error::other("navigation failed")));
        assert!(matches!(
            view.dispatch(BrowserCommand::Navigate("https://example.com".into())),
            Err(ViewError::Engine(_))
        ));
    }
}
