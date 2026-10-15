use browser_core::BrowserApplication;
use browser_windows::PlatformInfo;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let app = BrowserApplication::new();
    let platform = PlatformInfo::current();
    info!(app = app.name(), platform = platform.name(), "RustBrowser bootstrap");
    println!("{} bootstrap ready on {}", app.name(), platform.name());
    // The native window and WebView2 integration arrive in Groups 2 and 3.
}
