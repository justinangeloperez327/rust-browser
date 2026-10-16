# Group 4 — Engine abstraction

## Current state

Engine-neutral types, events, commands and capability flags are defined. The application hosts WebView2 via a convenience Windows host. The live command and event adapter has not yet been implemented.

## Dependency correction

The crates.io windows-webview 0.100.0 package does not expose the documented `system` feature. Without this feature, `WebViewWindow` is unavailable. The dependency now targets the Microsoft windows-rs Git repository using the documented `system` feature.

This Git dependency is an interim solution pending a pinned verified revision and repeatable Cargo.lock; a floating branch must not ship in production.

## Next work

1. Confirm this source resolves on the Windows CI runner.
2. Pin a verified Git revision or published version and commit Cargo.lock.
3. Connect host commands to a concrete UI-thread dispatcher.
4. Translate WebView2 navigation and process events to BrowserEvent.
5. Add runtime Windows smoke tests.

No full browser navigation UI or Servo integration is claimed.
