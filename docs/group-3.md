# Group 3 — WebView2 integration

## Status

Adapter crate and dependency isolation initialized. Actual COM environment/controller creation, host resizing, navigation, event registration and cleanup are **not yet implemented**. Do not treat the current application as a web browser.

## Implementation sequence

1. Ensure the Win32 UI thread initializes a single-threaded COM apartment before creating WebView2.
2. Create a profile-specific WebView2 user-data folder under LocalAppData.
3. Initialize CoreWebView2Environment asynchronously and keep its completion handler alive.
4. Create CoreWebView2Controller targeting the native HWND, then obtain CoreWebView2.
5. Set controller bounds on WM_SIZE / WM_DPICHANGED; use client coordinates.
6. Navigate to a fixed HTTPS start page after controller initialization.
7. Handle initialization failures, renderer crashes and window destruction.
8. Remove event tokens and close controller before COM teardown; never block the UI thread awaiting the completion callback.
9. Add functional Windows smoke tests; confirm the renderer process exits after closing the host.

## Verification

CI must pass formatting, clippy, tests and release compilation. An interactive Windows smoke test is additionally required for real rendering functionality.
