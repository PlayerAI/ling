/// Stable exit-code values already authorized for the implemented Ling CLI.
///
/// This module is an internal catalog only; command-specific output and future
/// project/transaction failures remain outside this bounded child.
pub(crate) const EXIT_SUCCESS: u8 = 0;
pub(crate) const EXIT_COMPILE_ERROR: u8 = 1;
pub(crate) const EXIT_INVALID_USAGE: u8 = 2;
pub(crate) const EXIT_RUNTIME_FAULT: u8 = 4;
pub(crate) const EXIT_INTERNAL_ERROR: u8 = 5;
pub(crate) const EXIT_SNAPSHOT_MISMATCH: u8 = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_exit_catalog_is_stable_and_has_no_implicit_code() {
        assert_eq!(EXIT_SUCCESS, 0);
        assert_eq!(EXIT_COMPILE_ERROR, 1);
        assert_eq!(EXIT_INVALID_USAGE, 2);
        assert_eq!(EXIT_RUNTIME_FAULT, 4);
        assert_eq!(EXIT_INTERNAL_ERROR, 5);
        assert_eq!(EXIT_SNAPSHOT_MISMATCH, 6);
        assert_ne!(EXIT_RUNTIME_FAULT, EXIT_INTERNAL_ERROR);
        assert_ne!(EXIT_INTERNAL_ERROR, EXIT_SNAPSHOT_MISMATCH);
        assert_eq!(
            [
                EXIT_SUCCESS,
                EXIT_COMPILE_ERROR,
                EXIT_INVALID_USAGE,
                EXIT_RUNTIME_FAULT,
                EXIT_INTERNAL_ERROR,
                EXIT_SNAPSHOT_MISMATCH,
            ],
            [0, 1, 2, 4, 5, 6]
        );
        assert!(
            ![
                EXIT_SUCCESS,
                EXIT_COMPILE_ERROR,
                EXIT_INVALID_USAGE,
                EXIT_RUNTIME_FAULT,
                EXIT_INTERNAL_ERROR,
                EXIT_SNAPSHOT_MISMATCH,
            ]
            .contains(&3)
        );
    }
}
