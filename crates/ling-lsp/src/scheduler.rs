use std::collections::BTreeMap;
use std::fmt;

use serde_json::Value;

/// Preview marker for deterministic logical LSP scheduling policy.
pub const SCHEDULING_PROTOCOL_VERSION: &str = "ling.lsp.scheduling/0.1";

pub const MAX_INTERACTIVE_BURST: u8 = 8;
pub const MAX_NON_BACKGROUND_BURST: u8 = 16;

/// Logical priority used by the internal LSP work-ordering child boundary.
///
/// These values are not a public request-priority contract. They only provide
/// canonical ordering for work that has already been created in-process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorkPriority {
    Interactive,
    Analysis,
    Background,
}

impl WorkPriority {
    const fn rank(self) -> u8 {
        match self {
            Self::Interactive => 0,
            Self::Analysis => 1,
            Self::Background => 2,
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Interactive => "interactive",
            Self::Analysis => "analysis",
            Self::Background => "background",
        }
    }
}

/// Classifies current methods without changing JSON-RPC wire order.
pub(crate) fn priority_for_method(method: &str) -> WorkPriority {
    match method {
        "workspace/symbol" => WorkPriority::Background,
        "textDocument/diagnostic"
        | "workspace/diagnostic"
        | "textDocument/semanticTokens/full"
        | "textDocument/semanticTokens/full/delta" => WorkPriority::Analysis,
        _ => WorkPriority::Interactive,
    }
}

pub(crate) fn priority_for_body(body: &[u8]) -> WorkPriority {
    serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("method")
                .and_then(Value::as_str)
                .map(priority_for_method)
        })
        .unwrap_or(WorkPriority::Interactive)
}

/// An opaque work item returned by [`InternalWorkQueue::pop`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ScheduledWork {
    id: u64,
    priority: WorkPriority,
    sequence: u64,
}

impl ScheduledWork {
    #[must_use]
    pub(crate) const fn id(self) -> u64 {
        self.id
    }

    #[must_use]
    pub(crate) const fn priority(self) -> WorkPriority {
        self.priority
    }

    #[must_use]
    pub(crate) const fn sequence(self) -> u64 {
        self.sequence
    }
}

/// Failure raised when a queue cannot allocate another local sequence value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorkQueueError {
    SequenceExhausted,
}

impl fmt::Display for WorkQueueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SequenceExhausted => formatter.write_str("LSP work queue sequence exhausted"),
        }
    }
}

impl std::error::Error for WorkQueueError {}

/// A deterministic, in-process ordering queue for future LSP analysis work.
///
/// The queue stores no request or document state and never executes work. The
/// key is `(priority rank, local enqueue sequence)`, so map order cannot leak
/// into the result and equal-priority items remain FIFO.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct InternalWorkQueue {
    next_sequence: u64,
    items: BTreeMap<(u8, u64), ScheduledWork>,
    interactive_streak: u8,
    non_background_streak: u8,
}

impl InternalWorkQueue {
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            next_sequence: 0,
            items: BTreeMap::new(),
            interactive_streak: 0,
            non_background_streak: 0,
        }
    }

    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Adds one opaque item without coalescing or executing it.
    pub(crate) fn enqueue(
        &mut self,
        id: u64,
        priority: WorkPriority,
    ) -> Result<(), WorkQueueError> {
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(WorkQueueError::SequenceExhausted)?;
        self.next_sequence = sequence;
        let item = ScheduledWork {
            id,
            priority,
            sequence,
        };
        self.items.insert((priority.rank(), sequence), item);
        Ok(())
    }

    /// Removes the next item according to the canonical priority/FIFO order.
    pub(crate) fn pop(&mut self) -> Option<ScheduledWork> {
        let key = self.items.keys().next().copied()?;
        self.items.remove(&key)
    }

    /// Removes the next item under the Preview bounded-fairness policy.
    ///
    /// Priority wins until its fixed logical burst is exhausted. No clock,
    /// worker count, or host load participates in selection.
    pub(crate) fn pop_fair(&mut self) -> Option<ScheduledWork> {
        let has_analysis = self.items.keys().any(|(rank, _)| *rank == 1);
        let has_background = self.items.keys().any(|(rank, _)| *rank == 2);
        let forced_rank =
            if has_background && self.non_background_streak >= MAX_NON_BACKGROUND_BURST {
                Some(2)
            } else if has_analysis && self.interactive_streak >= MAX_INTERACTIVE_BURST {
                Some(1)
            } else {
                None
            };
        let key = forced_rank
            .and_then(|rank| {
                self.items
                    .keys()
                    .find(|(item_rank, _)| *item_rank == rank)
                    .copied()
            })
            .or_else(|| self.items.keys().next().copied())?;
        let item = self.items.remove(&key)?;
        match item.priority {
            WorkPriority::Interactive => {
                self.interactive_streak = self.interactive_streak.saturating_add(1);
                self.non_background_streak = self.non_background_streak.saturating_add(1);
            }
            WorkPriority::Analysis => {
                self.interactive_streak = 0;
                self.non_background_streak = self.non_background_streak.saturating_add(1);
            }
            WorkPriority::Background => {
                self.interactive_streak = 0;
                self.non_background_streak = 0;
            }
        }
        Some(item)
    }

    /// Drops all queued items while retaining the local sequence monotonicity.
    pub(crate) fn clear(&mut self) {
        self.items.clear();
        self.interactive_streak = 0;
        self.non_background_streak = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_precedes_fifo_sequence() {
        let mut queue = InternalWorkQueue::new();
        queue
            .enqueue(1, WorkPriority::Background)
            .expect("first item fits");
        queue
            .enqueue(2, WorkPriority::Interactive)
            .expect("second item fits");
        queue
            .enqueue(3, WorkPriority::Interactive)
            .expect("third item fits");
        queue
            .enqueue(4, WorkPriority::Analysis)
            .expect("fourth item fits");
        queue
            .enqueue(5, WorkPriority::Background)
            .expect("fifth item fits");

        let popped = (0..5)
            .map(|_| queue.pop().expect("queue item exists"))
            .collect::<Vec<_>>();
        assert_eq!(
            popped.iter().map(|item| item.id()).collect::<Vec<_>>(),
            vec![2, 3, 4, 1, 5]
        );
        assert_eq!(popped[0].sequence(), 2);
        assert_eq!(popped[1].sequence(), 3);
        assert_eq!(popped[2].priority(), WorkPriority::Analysis);
        assert!(queue.is_empty());
    }

    #[test]
    fn duplicate_identifiers_are_independent_items() {
        let mut queue = InternalWorkQueue::new();
        queue
            .enqueue(7, WorkPriority::Analysis)
            .expect("first duplicate fits");
        queue
            .enqueue(7, WorkPriority::Analysis)
            .expect("second duplicate fits");
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop().expect("first duplicate").sequence(), 1);
        assert_eq!(queue.pop().expect("second duplicate").sequence(), 2);
        assert!(queue.pop().is_none());
    }

    #[test]
    fn clear_is_deterministic_and_preserves_sequence() {
        let mut queue = InternalWorkQueue::new();
        queue
            .enqueue(1, WorkPriority::Background)
            .expect("item fits");
        queue.clear();
        assert!(queue.is_empty());
        queue
            .enqueue(2, WorkPriority::Interactive)
            .expect("item after clear fits");
        assert_eq!(queue.pop().expect("item after clear").sequence(), 2);
    }

    #[test]
    fn fair_pop_bounds_analysis_and_background_starvation() {
        let mut queue = InternalWorkQueue::new();
        for id in 0..40 {
            queue
                .enqueue(id, WorkPriority::Interactive)
                .expect("interactive item fits");
        }
        queue
            .enqueue(100, WorkPriority::Analysis)
            .expect("analysis item fits");
        queue
            .enqueue(200, WorkPriority::Background)
            .expect("background item fits");

        let first_nine = (0..=MAX_INTERACTIVE_BURST)
            .map(|_| queue.pop_fair().expect("fair item"))
            .collect::<Vec<_>>();
        assert_eq!(first_nine.last().map(|item| item.id()), Some(100));

        let until_background = (0..=MAX_NON_BACKGROUND_BURST)
            .map(|_| queue.pop_fair().expect("fair item"))
            .collect::<Vec<_>>();
        assert!(until_background.iter().any(|item| item.id() == 200));
    }

    #[test]
    fn method_classification_is_explicit_and_stable() {
        assert_eq!(
            priority_for_method("textDocument/hover").name(),
            "interactive"
        );
        assert_eq!(
            priority_for_method("textDocument/diagnostic").name(),
            "analysis"
        );
        assert_eq!(priority_for_method("workspace/symbol").name(), "background");
        assert_eq!(priority_for_method("future/method").name(), "interactive");
    }
}
