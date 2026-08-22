use std::collections::BTreeMap;
use std::fmt;

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
}

impl InternalWorkQueue {
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            next_sequence: 0,
            items: BTreeMap::new(),
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

    /// Drops all queued items while retaining the local sequence monotonicity.
    pub(crate) fn clear(&mut self) {
        self.items.clear();
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
}
