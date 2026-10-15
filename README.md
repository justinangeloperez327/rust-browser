# RustBrowser

A Windows-native browser written in Rust, using Microsoft Edge WebView2 for web rendering. Engine interfaces will allow future experimentation with Servo.

## Status

Group 1: workspace foundation. No browsing or WebView2 integration yet.

## Requirements

- Windows 11 x64
- Rust stable MSVC toolchain (`rustup default stable-x86_64-pc-windows-msvc`)
- Visual Studio C++ build tools / Windows SDK

## Commands

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
cargo run -p rust-browser
```

See `docs/architecture.md` and `docs/roadmap.md` for the design and implementation plan.
