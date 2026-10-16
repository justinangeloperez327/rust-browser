# WebView2 event wiring (Group 4)

The single-window WebView2 prototype now attempts to retain three event registrations for its entire host lifetime: navigation completion, document-title changes and renderer/browser process failures.

Navigation completion and title change are translated to engine-neutral `BrowserEvent` values before tracing. Process failure is logged with its native kind. An event-bus consumer, command dispatcher, automatic crash recovery and multi-tab state synchronization are **not** implemented yet.

Windows CI must confirm the adapter compiles with the selected windows-rs source. Manual interactive validation is still required for navigation and event behavior. No performance improvements or resource guarantees are claimed.
