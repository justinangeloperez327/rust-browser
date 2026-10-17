# Group 4 — Engine-neutral event state

`ViewState::apply` now reduces `BrowserEvent` into a view lifecycle snapshot. It tracks readiness, loading, URL, title, failure and terminal closure; tests cover navigation completion, process failure and closure.

This is a **pure domain implementation**. The WebView2 callbacks are not yet connected to this reducer, and no active tab or UI is subscribed. It deliberately avoids native COM code until the pending Windows build is verified.
