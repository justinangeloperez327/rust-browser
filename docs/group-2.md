# Group 2 — Native Win32 bootstrap

The native window is created directly on the main STA-compatible UI thread, with a synchronous GetMessageW/DispatchMessageW loop. No engine or renderer is hosted yet. Per-monitor-v2 DPI awareness is requested before the first window. WM_DPICHANGED applies the suggested bounds.

## Design boundaries

- Win32 unsafe FFI is contained within the private native module in `browser-windows`.
- The rest of the workspace keeps safe Rust APIs.
- Browser UI chrome (tabs, toolbar, address bar) and WebView2 embedding follow in Groups 3–6.
- The initial 1200×800 native window is intentionally empty, rather than implying a functional browser.

## Verification

On Windows run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p rust-browser
```

Expected: a native window appears, supports resize and Windows controls, and closes cleanly. Interactive UI and visual testing are still required.
