# Group 5 — Navigation

The browser accepts launch URLs and search phrases through the command line. Its address resolver now uses the standard `url` crate and form-url-encoding rather than custom authority parsing.

Examples:

```powershell
cargo run -p rust-browser -- https://example.com
cargo run -p rust-browser -- rust-lang.org
cargo run -p rust-browser -- "Rust browser performance"
```

Explicit unsupported URI schemes are rejected. Network access is still governed by WebView2; URL validation is not a security sandbox.

Next: native editable address bar, Back / Forward / Reload / Stop and keyboard shortcuts. No interactive UI navigation controls are claimed by this change.
