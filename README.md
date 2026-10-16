# RustBrowser

RustBrowser is a Windows-native browser project written in Rust, targeting WebView2 rendering with an engine-neutral core for eventual Servo experimentation.

## Current status

Group 2 native window foundation committed. Group 3 WebView2 adapter crate is initialized, **but the application does not host the web renderer yet**.

## Requirements

- Windows 11 x64
- Rust stable MSVC toolchain
- Visual Studio C++ build tools and Windows SDK
- Microsoft Edge WebView2 Evergreen Runtime (needed when rendering is integrated)

## Development

```powershell
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
cargo run -p rust-browser
```

Read `docs/architecture.md`, `docs/roadmap.md`, and `docs/group-3.md`.
