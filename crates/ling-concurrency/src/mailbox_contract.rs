//! Checked-only local mailbox capacity and admission classification.

use std::error::Error;
use std::fmt;

pub const CHECKED_LOCAL_MAILBOX_VERSION: &str = "ling.checked-local-mailbox/1";
pub const MAX_LOCAL_MAILBOX_CAPACITY: u32 = 65_535;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MailboxCapacity(u32);

impl MailboxCapacity {
    pub fn new(value: u32) -> Result<Self, MailboxContractError> {
        if !(1..=MAX_LOCAL_MAILBOX_CAPACITY).contains(&value) {
            return Err(MailboxContractError::CapacityOutOfRange { value });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MailboxOverflowPolicy {
    Reject,
}

impl MailboxOverflowPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reject => "Reject",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Reject => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MailboxAdmission {
    Accepted,
    Full,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalMailboxContract {
    capacity: MailboxCapacity,
    overflow: MailboxOverflowPolicy,
    canonical_bytes: Box<[u8]>,
}

impl LocalMailboxContract {
    #[must_use]
    pub fn new(capacity: MailboxCapacity, overflow: MailboxOverflowPolicy) -> Self {
        let mut canonical_bytes = Vec::new();
        push_text(&mut canonical_bytes, CHECKED_LOCAL_MAILBOX_VERSION);
        canonical_bytes.extend_from_slice(&capacity.get().to_be_bytes());
        canonical_bytes.push(overflow.tag());
        Self {
            capacity,
            overflow,
            canonical_bytes: canonical_bytes.into_boxed_slice(),
        }
    }

    #[must_use]
    pub const fn capacity(&self) -> MailboxCapacity {
        self.capacity
    }

    #[must_use]
    pub const fn overflow(&self) -> MailboxOverflowPolicy {
        self.overflow
    }

    pub fn classify_admission(
        &self,
        queued_messages: u32,
    ) -> Result<MailboxAdmission, MailboxContractError> {
        if queued_messages > self.capacity.get() {
            return Err(MailboxContractError::QueueLengthExceedsCapacity {
                queued_messages,
                capacity: self.capacity,
            });
        }
        Ok(if queued_messages == self.capacity.get() {
            MailboxAdmission::Full
        } else {
            MailboxAdmission::Accepted
        })
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MailboxContractError {
    CapacityOutOfRange {
        value: u32,
    },
    QueueLengthExceedsCapacity {
        queued_messages: u32,
        capacity: MailboxCapacity,
    },
}

impl fmt::Display for MailboxContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityOutOfRange { value } => write!(
                formatter,
                "mailbox capacity {value} is outside 1..={MAX_LOCAL_MAILBOX_CAPACITY}"
            ),
            Self::QueueLengthExceedsCapacity {
                queued_messages,
                capacity,
            } => write!(
                formatter,
                "mailbox queue length {queued_messages} exceeds capacity {}",
                capacity.get()
            ),
        }
    }
}

impl Error for MailboxContractError {}

fn push_text(output: &mut Vec<u8>, value: &str) {
    let length = u32::try_from(value.len()).expect("mailbox canonical domain is bounded");
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_and_reject_admission_boundaries_are_exact() {
        assert!(MailboxCapacity::new(0).is_err());
        assert!(MailboxCapacity::new(MAX_LOCAL_MAILBOX_CAPACITY + 1).is_err());
        let contract = LocalMailboxContract::new(
            MailboxCapacity::new(2).expect("valid capacity"),
            MailboxOverflowPolicy::Reject,
        );
        assert_eq!(
            contract.classify_admission(0),
            Ok(MailboxAdmission::Accepted)
        );
        assert_eq!(
            contract.classify_admission(1),
            Ok(MailboxAdmission::Accepted)
        );
        assert_eq!(contract.classify_admission(2), Ok(MailboxAdmission::Full));
        assert!(matches!(
            contract.classify_admission(3),
            Err(MailboxContractError::QueueLengthExceedsCapacity { .. })
        ));
    }

    #[test]
    fn maximum_capacity_classification_is_bounded_and_deterministic() {
        let contract = LocalMailboxContract::new(
            MailboxCapacity::new(MAX_LOCAL_MAILBOX_CAPACITY).expect("maximum is valid"),
            MailboxOverflowPolicy::Reject,
        );
        for queued in 0..MAX_LOCAL_MAILBOX_CAPACITY {
            assert_eq!(
                contract.classify_admission(queued),
                Ok(MailboxAdmission::Accepted)
            );
        }
        assert_eq!(
            contract.classify_admission(MAX_LOCAL_MAILBOX_CAPACITY),
            Ok(MailboxAdmission::Full)
        );
    }

    #[test]
    fn canonical_bytes_encode_only_version_capacity_and_policy() {
        let first = LocalMailboxContract::new(
            MailboxCapacity::new(16).expect("valid capacity"),
            MailboxOverflowPolicy::Reject,
        );
        let second = LocalMailboxContract::new(
            MailboxCapacity::new(16).expect("valid capacity"),
            MailboxOverflowPolicy::Reject,
        );
        let changed = LocalMailboxContract::new(
            MailboxCapacity::new(17).expect("valid capacity"),
            MailboxOverflowPolicy::Reject,
        );
        assert_eq!(first.canonical_bytes(), second.canonical_bytes());
        assert_ne!(first.canonical_bytes(), changed.canonical_bytes());
    }
}
