# Group 3 — WebView2 hosting

## Implementation

The application now calls `engine_webview2::run()`. On Windows, this starts a native window through the official `windows-webview` system host and navigates to `https://example.com`. The host owns COM initialization, the window, message loop, WebView2 environment/controller, resize handling, and normal cleanup.

This is intentionally an initial single-window implementation. The custom Win32 window from Group 2 remains isolated in `browser-windows` but is no longer called by the entry point. Group 4 will establish a common host abstraction and unify window lifecycle, avoiding competing native windows/message loops.

The first page loads using WebView2 Chromium rendering if the runtime is installed. On missing runtime or initialization failure, startup returns an error.

## Tested by CI

Formatting, Clippy, tests and release compilation on a Windows runner.

## Manual Windows smoke test

1. Confirm the Microsoft Edge WebView2 Evergreen Runtime is installed.
2. Run `cargo run -p rust-browser`.
3. Confirm a native RustBrowser window opens and `https://example.com` renders.
4. Resize, minimize, maximize and restore.
5. Close the window and check there are no orphan RustBrowser-owned WebView2 processes.
6. Relaunch and confirm the start page loads again.

**Manual renderer tests and GPU/RAM benchmarking have not yet been performed.**

## Deferred

Address bar, tab management, history, explicit profile directories, environment sharing, and recoverable process-failure events are future groups. This prototype must not be described as a complete browser.
