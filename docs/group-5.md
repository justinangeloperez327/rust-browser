# Group 5 — Navigation (first functional increment)

The browser can now launch an HTTP(S) URL, bare hostname, or encoded search phrase from the command line.

Examples:

```powershell
cargo run -p rust-browser -- https://example.com
cargo run -p rust-browser -- rust-lang.org
cargo run -p rust-browser -- "Rust browser performance"
```

All launch inputs are normalized by browser-navigation. Dangerous/unsupported schemes are rejected. The implementation is an initial parser, not an exhaustive URL security validator; a standards-compliant URL parser is planned.

The runtime WebView2 window navigates to the resolved destination. A native address bar, Back, Forward, Reload, Stop and keyboard shortcuts are **not yet implemented**. Manual Windows smoke testing is required.
