# Group 4 — Engine abstraction (initial implementation)

## Implemented

- Engine descriptor and capability types independent of native COM.
- Browser navigation commands and lifecycle event values.
- BrowserView dispatch trait, intended for a UI-thread owner.
- WebView2 capability declaration and unit tests.
- Removed the unsupported `system` Cargo feature from `windows-webview`.

## Incomplete

The prototype currently starts through `WebViewWindow::run`. It does not yet expose a full live BrowserView adapter or route native WebView2 events through BrowserEvent. Group 4 is only complete when the host's navigation and lifecycle are wired to the traits and the Windows CI build passes.

Future Servo support must not assume suspension, downloads, and profiles are uniformly implemented by all engines.

## Verification

- Run Windows CI for dependency resolution, clippy, tests and release build.
- On a Windows desktop, launch and close the WebView2 window.
- Confirm no orphaned browser processes remain after exit.
