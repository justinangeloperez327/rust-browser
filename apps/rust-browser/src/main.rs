use browser_core::BrowserApplication;
use browser_windows::PlatformInfo;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = BrowserApplication::new();
    let platform = PlatformInfo::current();
    info!(
        app = app.name(),
        platform = platform.name(),
        "RustBrowser starting"
    );
    let address = std::env::args().nth(1);
    let target = match address {
        Some(input) => browser_navigation::resolve_address(&input)?,
        None => engine_webview2::START_PAGE.to_owned(),
    };
    engine_webview2::run(&target).map_err(|err| format!("WebView2 hosting failed: {err}"))?;
    Ok(())
}
