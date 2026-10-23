# Group 5 Windows Manual Verification

The Windows CI pipeline verifies formatting, compilation, linting, tests, and the release build. It does **not** exercise a live WebView2 window or native input controls.

On a Windows 10/11 machine with the Microsoft Edge WebView2 Runtime installed:

1. Run `cargo run -p rust-browser`. Confirm the toolbar is above the webpage, not overlaying it.
2. Type `https://example.com` in the address bar and press Enter. Confirm navigation.
3. Use Ctrl+L to select the address, type another valid URL or search phrase, and press Enter. Check the target is correctly resolved.
4. Navigate through two or three pages. Test Back and Forward buttons as well as Alt+Left and Alt+Right.
5. Test Reload and F5. During a slow navigation, test Stop and Escape.
6. Follow a link inside a webpage. Confirm the address field updates to the new URL after navigation completes.
7. Begin typing in the address bar while a page is loading. Confirm navigation completion does not replace the unsubmitted text.
8. Resize, maximize, restore, and move the window between displays. Confirm toolbar controls remain accessible and the webpage stays beneath the toolbar without clipped or stale content.
9. Close the browser during a page load and after several navigations. Confirm it exits without a crash or a lingering process.

Record the Windows version, WebView2 Runtime version, observed results, and any reproduction steps before marking Group 5 complete.
