//! Engine-neutral foundational types. Actual engine traits are Group 4.

/// Stable browser-engine identifiers, with an experimental Servo slot.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_webview2_is_production_engine() {
        assert!(EngineKind::WebView2.is_production());
        assert!(!EngineKind::ServoExperimental.is_production());
    }
}
