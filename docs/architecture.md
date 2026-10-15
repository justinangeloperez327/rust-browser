# RustBrowser architecture

## Scope

Windows 11 x64 browser with Rust native shell, WebView2 engine, and an engine-neutral application core. Servo remains an optional experiment after v1.

## Dependency direction

- `apps/rust-browser` wires modules and owns startup/shutdown.
- `browser-core` owns application state and commands; must not expose COM.
- `browser-engine` defines engine-neutral types (Group 1), capabilities, events, and traits (Group 4).
- `browser-windows` isolates native Windows interactions; actual HWND/UI thread is Group 2.
- `engine-webview2` (Group 3) owns COM objects, controllers and registered event handlers.

## Runtime constraints

WebView2 controllers are bound to the UI thread's message pump. Async tasks must dispatch UI operations rather than moving COM objects to Tokio worker threads. Share WebView2 environments only for compatible user-data folders/profile settings. Record browser subprocess resource use in benchmarks.

## Security

Preserve WebView2 sandbox and runtime servicing. Gate permissions and external URL schemes, isolate private profiles, and treat all web content as untrusted.

## Resource lifecycle

Tab states: created, active, background, suspended, discarded and closed. Eligible tabs can suspend; discarding releases their engine view and requires restoration. Protect audio, calls, uploads, forms and permission prompts. Never present inferred tab savings as measured values.

## Non-goals for Group 1

No web rendering, native window, runtime performance claims, browser permissions, or persistent browsing profiles.
