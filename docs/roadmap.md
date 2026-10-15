# Implementation roadmap

1. Workspace, bootstrap, documentation, Windows CI.
2. Win32 window, DPI support and message loop.
3. WebView2 environment, controller and clean lifecycle.
4. Engine-neutral traits and UI dispatch.
5. Navigation, address bar and keyboard commands.
6. Tabs and multiple windows.
7. Tab suspension, discard and restore.
8. History, bookmarks and SQLite migrations.
9. Downloads and failure handling.
10. Profiles and private mode.
11. Security, tracking controls and permissions.
12. Session journal and crash recovery.
13. Settings and accessibility.
14. CPU, memory and GPU telemetry.
15. Startup and resource optimization.
16. Reproducible cross-browser benchmarks.
17. Stress testing and leak detection.
18. ZIP and Windows installer.
19. Regression tests and security audit.
20. Release candidate and checksums.

## Group 1 acceptance

- Cargo workspace build in debug/release on Windows.
- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo test --workspace` passes.
- Bootstrap starts and exits cleanly.
- CI runs successfully on Windows.

Group 1 is not considered verified until the CI run is green.
