# Group 4 — Initial live command dispatch

The WebView2 startup navigation now enters through the engine-neutral `BrowserCommand::Navigate` command. `dispatch_navigation` invokes the concrete WebView2 navigation method on the UI thread, with unit tests covering dispatch and unsupported commands.

Reload, stop, back, and forward remain unsupported and are not advertised in the adapter capability flags. The public `BrowserView` trait still needs a concrete persistent view implementation, an event consumer, and session management. This iteration does not claim Group 4 completion.

The earlier navigation-completion, title and process-failure event handlers remain registered. Manual Windows testing is required to confirm actual page rendering and lifecycle behavior.
