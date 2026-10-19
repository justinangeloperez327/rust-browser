//! Application-owned browser state; no WebView2 or Win32 types are exposed here.

use browser_engine::BrowserEvent;

/// Root application descriptor.
#[derive(Debug, Default, Clone, Copy)]
pub struct BrowserApplication;

impl BrowserApplication {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        "RustBrowser"
    }
}

/// Engine-agnostic state for a single browser view.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ViewState {
    pub ready: bool,
    pub loading: bool,
    pub url: Option<String>,
    pub title: Option<String>,
    pub failed: bool,
    pub closed: bool,
}

impl ViewState {
    /// Apply a lifecycle event without requiring access to native engine objects.
    pub fn apply(&mut self, event: BrowserEvent) {
        if self.closed {
            return;
        }
        match event {
            BrowserEvent::Ready => {
                self.ready = true;
                self.failed = false;
            }
            BrowserEvent::NavigationStarted { url } => {
                self.url = Some(url);
                self.loading = true;
            }
            BrowserEvent::NavigationCompleted { url, success } => {
                self.url = Some(url);
                self.loading = false;
                self.failed = !success;
            }
            BrowserEvent::TitleChanged(title) => self.title = Some(title),
            BrowserEvent::ProcessFailed => {
                self.failed = true;
                self.loading = false;
                self.ready = false;
            }
            BrowserEvent::Closed => {
                self.closed = true;
                self.loading = false;
                self.ready = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_has_stable_name() {
        assert_eq!(BrowserApplication::new().name(), "RustBrowser");
    }

    #[test]
    fn navigation_updates_loading_and_url() {
        let mut state = ViewState::default();
        state.apply(BrowserEvent::Ready);
        state.apply(BrowserEvent::NavigationStarted {
            url: "https://example.com".into(),
        });
        assert!(state.loading);
        state.apply(BrowserEvent::NavigationCompleted {
            url: "https://example.com".into(),
            success: true,
        });
        assert!(!state.loading);
        assert!(!state.failed);
        assert_eq!(state.url.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn process_failure_resets_readiness() {
        let mut state = ViewState::default();
        state.apply(BrowserEvent::Ready);
        state.apply(BrowserEvent::ProcessFailed);
        assert!(state.failed);
        assert!(!state.ready);
    }

    #[test]
    fn closed_view_cannot_be_reactivated() {
        let mut state = ViewState::default();
        state.apply(BrowserEvent::Closed);
        state.apply(BrowserEvent::Ready);
        assert!(state.closed);
        assert!(!state.ready);
    }
    #[test]
    fn failed_navigation_sets_failure_state() {
        let mut state = ViewState::default();
        state.apply(BrowserEvent::Ready);
        state.apply(BrowserEvent::NavigationCompleted {
            url: "https://invalid.example".into(),
            success: false,
        });
        assert!(state.failed);
        assert!(!state.loading);
    }

    #[test]
    fn title_updates_do_not_override_current_url() {
        let mut state = ViewState::default();
        state.apply(BrowserEvent::NavigationStarted {
            url: "https://example.com".into(),
        });
        state.apply(BrowserEvent::TitleChanged("Example".into()));
        assert_eq!(state.url.as_deref(), Some("https://example.com"));
        assert_eq!(state.title.as_deref(), Some("Example"));
    }

}
