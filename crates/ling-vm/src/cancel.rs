use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Monotonic host-owned cancellation request for the Experimental VM control API.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    requested: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a token with no cancellation request.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation. Repeated requests are idempotent.
    pub fn cancel(&self) {
        self.requested.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::CancellationToken;

    #[test]
    fn cancellation_is_monotonic_and_clone_shared() {
        let token = CancellationToken::new();
        let clone = token.clone();
        assert!(!token.is_cancelled());
        clone.cancel();
        assert!(token.is_cancelled());
        token.cancel();
        assert!(clone.is_cancelled());
    }
}
