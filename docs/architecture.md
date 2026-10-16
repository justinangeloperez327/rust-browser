# RustBrowser architecture

## Scope

Windows 11 x64 browser built in Rust using a native Windows host, WebView2 rendering, and a future engine-neutral interface for Servo experimentation.

## Dependency direction

- `apps/rust-browser`: startup, logging and application wiring.
- `browser-core`: engine-independent application state.
- `browser-engine`: engine-neutral data types; capability-based interfaces planned in Group 4.
- `engine-webview2`: native hosted rendering via `windows-webview`.
- `browser-windows`: original lower-level Win32 window bootstrap; currently not used by main.

## Native lifecycle

For the first single-view prototype, `WebViewWindow::run` owns the parent native window, STA COM apartment, WebView2 controller, resize handling, UI message loop and shutdown. Avoid mixing this convenience loop with the custom Win32 message loop. Group 4 will consolidate the host boundary and introduce a typed capability model.

## Future resource management

Tabs: created, active, background, suspended, discarded, closed. Avoid discarding active media, uploads or unsaved work. Measure the entire process tree rather than only the Rust executable.

## Security

Do not disable the WebView2 sandbox. Treat all page content as untrusted. Explicit browser permissions, privacy profiles, safe download handling and external scheme controls remain outstanding tasks.
