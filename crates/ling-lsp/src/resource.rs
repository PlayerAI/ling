use std::fmt;

/// A deterministic in-process UTF-8 byte budget for the LSP resource child.
///
/// This value accounts arithmetic units only. It does not observe allocator
/// or process memory and is not connected to a public protocol response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ByteBudget {
    limit: usize,
    used: usize,
}

impl ByteBudget {
    #[must_use]
    pub(crate) const fn new(limit: usize) -> Self {
        Self { limit, used: 0 }
    }

    #[must_use]
    pub(crate) const fn limit(self) -> usize {
        self.limit
    }

    #[must_use]
    pub(crate) const fn used(self) -> usize {
        self.used
    }

    #[must_use]
    pub(crate) const fn remaining(self) -> usize {
        self.limit - self.used
    }

    /// Reserves UTF-8 bytes without changing state on failure.
    pub(crate) fn try_reserve(&mut self, amount: usize) -> Result<(), ByteBudgetError> {
        let remaining = self.remaining();
        if amount > remaining {
            return Err(ByteBudgetError::LimitExceeded {
                requested: amount,
                remaining,
            });
        }
        self.used += amount;
        Ok(())
    }

    /// Releases previously reserved bytes without underflow.
    pub(crate) fn release(&mut self, amount: usize) -> Result<(), ByteBudgetError> {
        if amount > self.used {
            return Err(ByteBudgetError::ReleaseExceedsUsage {
                requested: amount,
                used: self.used,
            });
        }
        self.used -= amount;
        Ok(())
    }
}

/// Typed arithmetic failures for the internal byte budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ByteBudgetError {
    LimitExceeded { requested: usize, remaining: usize },
    ReleaseExceedsUsage { requested: usize, used: usize },
}

impl fmt::Display for ByteBudgetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitExceeded {
                requested,
                remaining,
            } => write!(
                formatter,
                "LSP byte budget exceeded: requested {requested}, remaining {remaining}"
            ),
            Self::ReleaseExceedsUsage { requested, used } => write!(
                formatter,
                "LSP byte budget release exceeds usage: requested {requested}, used {used}"
            ),
        }
    }
}

impl std::error::Error for ByteBudgetError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_boundary_and_failed_reserve_are_stable() {
        let mut budget = ByteBudget::new(8);
        assert_eq!(budget.limit(), 8);
        assert_eq!(budget.remaining(), 8);
        budget.try_reserve(5).expect("reserve below limit");
        assert_eq!(budget.used(), 5);
        assert_eq!(budget.remaining(), 3);
        budget.try_reserve(3).expect("exact boundary fits");
        assert_eq!(budget.used(), 8);
        assert_eq!(budget.remaining(), 0);
        assert_eq!(
            budget.try_reserve(1),
            Err(ByteBudgetError::LimitExceeded {
                requested: 1,
                remaining: 0,
            })
        );
        assert_eq!(budget.used(), 8);
    }

    #[test]
    fn release_is_checked_and_zero_is_a_noop() {
        let mut budget = ByteBudget::new(4);
        budget.try_reserve(3).expect("reserve fits");
        budget.release(0).expect("zero release is valid");
        assert_eq!(budget.used(), 3);
        budget.release(2).expect("release fits");
        assert_eq!(budget.used(), 1);
        assert_eq!(
            budget.release(2),
            Err(ByteBudgetError::ReleaseExceedsUsage {
                requested: 2,
                used: 1,
            })
        );
        assert_eq!(budget.used(), 1);
    }

    #[test]
    fn independent_budgets_do_not_share_usage() {
        let mut left = ByteBudget::new(2);
        let right = ByteBudget::new(2);
        left.try_reserve(2).expect("left reserve fits");
        assert_eq!(left.used(), 2);
        assert_eq!(right.used(), 0);
        assert_eq!(right.remaining(), 2);
    }
}
