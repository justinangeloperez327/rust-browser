# RustBrowser

Windows-native lightweight browser project written in Rust, initially hosted by Microsoft Edge WebView2. A future engine-neutral abstraction is planned for Servo experiments.

## Status

Group 3: first WebView2 rendering host wired to the application entry point. Browser controls, tabs, custom profiles, telemetry and optimizations are not implemented.

## Requirements

- Windows 11 x64
- Rust stable MSVC toolchain
- Visual Studio C++ build tools and Windows SDK
- Microsoft Edge WebView2 Evergreen Runtime

## Development

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
cargo run -p rust-browser
```

The current application opens a native WebView2 window and loads `https://example.com`. See `docs/group-3.md` for manual smoke tests and known limitations.
