# RustBrowser

Windows-native browser project written in Rust with Microsoft Edge WebView2 rendering. The application core uses engine-neutral types for future Servo experimentation.

## Status

Groups 1–3 scaffold the app and native WebView2 prototype. Group 4 introduces engine capabilities, commands, events and descriptor contracts. A full live command dispatcher, tab manager and browsing UI are not yet implemented.

## Requirements

- Windows 11 x64
- Stable Rust MSVC toolchain
- Visual Studio C++ build tools and Windows SDK
- Microsoft Edge WebView2 Evergreen Runtime

## Build

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
cargo run -p rust-browser
```

See `docs/architecture.md`, `docs/group-4.md` and `docs/roadmap.md`.
