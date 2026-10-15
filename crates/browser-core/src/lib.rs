//! Application-owned browser state; no WebView2 or Win32 types are exposed here.

/// Root application descriptor. Runtime state is introduced in later groups.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_has_stable_name() {
        assert_eq!(BrowserApplication::new().name(), "RustBrowser");
    }
}
