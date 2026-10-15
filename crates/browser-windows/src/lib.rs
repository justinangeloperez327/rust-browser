//! Native Windows adapter boundary; actual window creation is Group 2.

pub struct PlatformInfo;

impl PlatformInfo {
    #[must_use]
    pub const fn current() -> Self {
        Self
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            "Windows"
        } else {
            "non-Windows development host"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_name_is_nonempty() {
        assert!(!PlatformInfo::current().name().is_empty());
    }
}
