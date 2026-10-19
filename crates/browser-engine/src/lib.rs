//! Engine-neutral contracts for RustBrowser.
//! Native engine implementations handle UI-thread affinity separately.

pub mod mailbox;

/// Supported engine implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    WebView2,
    ServoExperimental,
}

impl EngineKind {
    #[must_use]
    pub const fn is_production(self) -> bool {
        matches!(self, Self::WebView2)
    }
}

/// Capabilities must be queried rather than assumed across rendering engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EngineCapabilities {
    pub navigation: bool,
    pub reload: bool,
    pub history: bool,
    pub suspension: bool,
    pub downloads: bool,
    pub private_profiles: bool,
}

/// Navigation and view commands dispatched to the owning engine UI thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserCommand {
    Navigate(String),
    Reload,
    Stop,
    Back,
    Forward,
}

/// Engine lifecycle notifications independent of native COM types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserEvent {
    Ready,
    NavigationStarted { url: String },
    NavigationCompleted { url: String, success: bool },
    TitleChanged(String),
    ProcessFailed,
    Closed,
}

/// An engine declaration shared by every future adapter.
pub trait EngineDescriptor {
    fn kind(&self) -> EngineKind;
    fn capabilities(&self) -> EngineCapabilities;
}

/// Navigation is intentionally separate from the descriptor, allowing
/// thread-bound implementations to dispatch commands via their native host.
pub trait BrowserView {
    type Error: std::error::Error;
    fn dispatch(&mut self, command: BrowserCommand) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_webview2_is_production() {
        assert!(EngineKind::WebView2.is_production());
        assert!(!EngineKind::ServoExperimental.is_production());
    }

    #[test]
    fn default_capabilities_do_not_claim_support() {
        let capabilities = EngineCapabilities::default();
        assert!(!capabilities.navigation);
        assert!(!capabilities.suspension);
    }

    #[test]
    fn navigation_command_preserves_target() {
        assert_eq!(
            BrowserCommand::Navigate("https://example.com".into()),
            BrowserCommand::Navigate("https://example.com".into())
        );
    }
}
