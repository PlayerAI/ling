use ling_lsp::{CancellationError, CancellationToken};

#[test]
fn cancellation_is_clone_shared_monotonic_and_idempotent() {
    let token = CancellationToken::new();
    let worker = token.clone();

    assert!(!token.is_cancelled());
    assert_eq!(token.check(), Ok(()));
    assert_eq!(worker.check(), Ok(()));

    worker.cancel();
    assert!(token.is_cancelled());
    assert!(worker.is_cancelled());
    assert_eq!(token.check(), Err(CancellationError::Cancelled));
    assert_eq!(worker.check(), Err(CancellationError::Cancelled));

    token.cancel();
    worker.cancel();
    assert_eq!(token.check(), Err(CancellationError::Cancelled));
    assert_eq!(worker.check(), Err(CancellationError::Cancelled));
}

#[test]
fn independent_tokens_do_not_share_cancellation_state() {
    let cancelled = CancellationToken::new();
    let active = CancellationToken::new();

    cancelled.cancel();
    assert_eq!(cancelled.check(), Err(CancellationError::Cancelled));
    assert_eq!(active.check(), Ok(()));
}
