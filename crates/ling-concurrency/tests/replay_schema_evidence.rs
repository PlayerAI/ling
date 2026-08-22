//! Internal replay-schema vocabulary evidence.
//!
//! This test-only inventory records proposed field names and ordering. It does
//! not define a wire format, encode payloads, calculate checksums, or replay a
//! runtime.

use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedReplayField {
    CanonicalEnvelope,
    EventId,
    EventKind,
    Ordering,
    Identity,
    Checksum,
    DeterminismClass,
    Toolchain,
    Profile,
    Schema,
    Payload,
    Migration,
    Privacy,
}

impl PlannedReplayField {
    const ALL: [Self; 13] = [
        Self::CanonicalEnvelope,
        Self::EventId,
        Self::EventKind,
        Self::Ordering,
        Self::Identity,
        Self::Checksum,
        Self::DeterminismClass,
        Self::Toolchain,
        Self::Profile,
        Self::Schema,
        Self::Payload,
        Self::Migration,
        Self::Privacy,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::CanonicalEnvelope => 0,
            Self::EventId => 1,
            Self::EventKind => 2,
            Self::Ordering => 3,
            Self::Identity => 4,
            Self::Checksum => 5,
            Self::DeterminismClass => 6,
            Self::Toolchain => 7,
            Self::Profile => 8,
            Self::Schema => 9,
            Self::Payload => 10,
            Self::Migration => 11,
            Self::Privacy => 12,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayFieldInventory {
    fields: Box<[PlannedReplayField]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReplayFieldError {
    Duplicate(PlannedReplayField),
}

impl fmt::Display for ReplayFieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(field) => write!(formatter, "duplicate replay field {field:?}"),
        }
    }
}

impl ReplayFieldInventory {
    fn new(fields: impl IntoIterator<Item = PlannedReplayField>) -> Result<Self, ReplayFieldError> {
        let mut fields = fields.into_iter().collect::<Vec<_>>();
        fields.sort_unstable_by_key(|field| field.rank());
        let mut seen = BTreeSet::new();
        for field in &fields {
            if !seen.insert(*field) {
                return Err(ReplayFieldError::Duplicate(*field));
            }
        }
        Ok(Self {
            fields: fields.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.replay-schema-observation/0");
        bytes.push(self.fields.len() as u8);
        bytes.extend(self.fields.iter().map(|field| field.rank()));
        bytes
    }
}

#[test]
fn proposed_replay_field_inventory_is_complete_and_ordered() {
    let inventory = ReplayFieldInventory::new(PlannedReplayField::ALL)
        .expect("the proposed field inventory has no duplicates");
    assert_eq!(inventory.fields.as_ref(), &PlannedReplayField::ALL);
    assert_eq!(
        inventory
            .fields
            .iter()
            .map(|field| field.rank())
            .collect::<Vec<_>>(),
        (0..13).collect::<Vec<_>>()
    );
}

#[test]
fn replay_field_evidence_is_order_independent_and_duplicate_checked() {
    let reversed = PlannedReplayField::ALL.into_iter().rev();
    let forward = ReplayFieldInventory::new(PlannedReplayField::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ReplayFieldInventory::new(reversed)
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate =
        ReplayFieldInventory::new([PlannedReplayField::EventId, PlannedReplayField::EventId])
            .expect_err("duplicate field vocabulary must be rejected");
    assert_eq!(
        duplicate,
        ReplayFieldError::Duplicate(PlannedReplayField::EventId)
    );
}

#[test]
fn schema_evidence_is_not_a_replay_wire_protocol() {
    let inventory = ReplayFieldInventory::new([
        PlannedReplayField::CanonicalEnvelope,
        PlannedReplayField::Schema,
        PlannedReplayField::Payload,
    ])
    .expect("bounded evidence inventory");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.replay-schema-observation/0")
    );
    assert_eq!(inventory.fields.len(), 3);
}
