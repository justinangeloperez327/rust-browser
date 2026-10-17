# WebView2 lifecycle bridge

WebView2 startup navigation uses the directly supported host API while the experimental BrowserView adapter remains separately unit-tested. The native host now owns a synchronized ViewState snapshot. Document-title, navigation-completed and process-failed callbacks apply engine-neutral BrowserEvent values to the snapshot.

This snapshot is not yet exposed to a toolbar or a multi-tab application state manager. Navigation-started and window-closed events are not wired. This is an incremental integration and does not finish Group 4.

CI is required to verify the native event-handler closure types and dependency graph. Interactive smoke testing is still outstanding.
