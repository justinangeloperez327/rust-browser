# Group 5 — Navigation

RustBrowser accepts an HTTP(S) URL, hostname or search query as a launch argument. Input resolution uses the `url` crate.

The WebView2 host now handles native accelerator keys:

| Shortcut | Command |
| --- | --- |
| F5 | Reload |
| Escape | Stop loading |
| Alt+Left | Back |
| Alt+Right | Forward |

Keyboard accelerators are registered with WebView2's controller on its UI thread, and retained for the host lifecycle. Calls can fail (for example if no back history exists); failures are logged, not silently assumed successful.

Examples:

```powershell
cargo run -p rust-browser -- https://example.com
cargo run -p rust-browser -- rust-lang.org
cargo run -p rust-browser -- "Rust browser performance"
```

Remaining: editable native address bar, clickable toolbar, additional keyboard shortcuts, desktop smoke test. The commands and shortcut wiring require a successful Windows CI run and interactive validation.
