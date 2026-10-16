# Group 4 — BrowserView integration

The startup navigation now flows through a concrete `WebView2View<N>` implementing the engine-neutral `BrowserView` trait. The adapter takes a UI-thread-bound navigation callback and preserves engine errors separately from unsupported commands. Tests cover repeat dispatch, unsupported commands and engine failure.

The current adapter only implements `Navigate`. Back, Forward, Reload and Stop are explicitly unsupported. The adapter exists for startup navigation only, not as a persistent externally accessible command queue; live UI command dispatch and centralized event consumption are still outstanding.

The host retains navigation-completed, title-changed, and process-failed subscriptions. Interactive Windows verification remains necessary.
