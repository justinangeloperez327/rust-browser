# Group 4 — Command mailbox

The engine-neutral layer now has a bounded FIFO mailbox with explicit backpressure, close semantics and deterministic unit tests. It can stage BrowserCommand values before UI-thread dispatch. It is not thread-safe by itself and does not create a Windows message-loop wakeup; use a thread-safe sender and native message notification when integrating with the host. No new live navigation commands are claimed.
