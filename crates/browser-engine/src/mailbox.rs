//! Bounded command mailbox for handing navigation requests to the engine UI thread.
//! This is transport-neutral; WebView2 consumption will be wired separately.

use std::collections::VecDeque;

use crate::BrowserCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MailboxFull;

impl std::fmt::Display for MailboxFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("browser command mailbox is full")
    }
}

impl std::error::Error for MailboxFull {}

/// A bounded, non-blocking command queue.
/// The creator is responsible for dispatching queued commands on the engine UI thread.
#[derive(Debug)]
pub struct CommandMailbox {
    capacity: usize,
    pending: VecDeque<BrowserCommand>,
    closed: bool,
}

impl CommandMailbox {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            pending: VecDeque::new(),
            closed: false,
        }
    }

    pub fn enqueue(&mut self, command: BrowserCommand) -> Result<(), MailboxFull> {
        if self.closed || self.pending.len() >= self.capacity {
            return Err(MailboxFull);
        }
        self.pending.push_back(command);
        Ok(())
    }

    pub fn take_next(&mut self) -> Option<BrowserCommand> {
        self.pending.pop_front()
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.pending.clear();
    }

    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_preserves_command_order() {
        let mut queue = CommandMailbox::new(2);
        queue.enqueue(BrowserCommand::Back).unwrap();
        queue.enqueue(BrowserCommand::Forward).unwrap();
        assert_eq!(queue.take_next(), Some(BrowserCommand::Back));
        assert_eq!(queue.take_next(), Some(BrowserCommand::Forward));
    }

    #[test]
    fn full_queue_rejects_excess_commands() {
        let mut queue = CommandMailbox::new(1);
        queue.enqueue(BrowserCommand::Reload).unwrap();
        assert!(queue.enqueue(BrowserCommand::Stop).is_err());
        assert_eq!(queue.pending_count(), 1);
    }

    #[test]
    fn close_clears_and_rejects_requests() {
        let mut queue = CommandMailbox::new(2);
        queue.enqueue(BrowserCommand::Reload).unwrap();
        queue.close();
        assert_eq!(queue.take_next(), None);
        assert!(queue.enqueue(BrowserCommand::Back).is_err());
    }
}
