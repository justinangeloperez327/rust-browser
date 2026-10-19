# Group 5 — Navigation

Startup URLs and search queries are normalized in `browser-navigation`. WebView2 now has a concrete UI-thread command dispatcher for Navigate, Back, Forward, Reload, and Stop, and startup navigation uses it.

The dispatcher is available in the engine but not yet connected to a native toolbar or keyboard accelerators. It must only be used on WebView2's owning thread. Calling Back or Forward with no history may return an engine error; callers must handle this.

Examples:

```powershell
cargo run -p rust-browser -- https://example.com
cargo run -p rust-browser -- rust-lang.org
cargo run -p rust-browser -- "Rust browser performance"
```

Remaining: native address bar, live toolbar button wiring, keyboard shortcuts, interactive smoke test.
