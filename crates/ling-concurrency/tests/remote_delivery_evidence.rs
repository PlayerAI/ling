//! Internal remote-delivery boundary evidence.
//!
//! This test-only inventory names proposed delivery and failure boundaries.
//! It does not choose a delivery guarantee, retry policy, ordering rule, or
//! implement a transport/runtime.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedDeliveryBoundary {
    DeliveryClass,
    AtMostOnce,
    AtLeastOnce,
    IdempotentRetry,
    ExactlyOnceBoundary,
    DeliveryIdentity,
    IdempotenceKey,
    Deduplication,
    Ordering,
    Causality,
    Timeout,
    Disconnect,
    Duplicate,
    Reorder,
    StaleIncarnation,
    RemoteRestart,
    SchemaMismatch,
    CapabilityRevocation,
}

impl PlannedDeliveryBoundary {
    const ALL: [Self; 18] = [
        Self::DeliveryClass,
        Self::AtMostOnce,
        Self::AtLeastOnce,
        Self::IdempotentRetry,
        Self::ExactlyOnceBoundary,
        Self::DeliveryIdentity,
        Self::IdempotenceKey,
        Self::Deduplication,
        Self::Ordering,
        Self::Causality,
        Self::Timeout,
        Self::Disconnect,
        Self::Duplicate,
        Self::Reorder,
        Self::StaleIncarnation,
        Self::RemoteRestart,
        Self::SchemaMismatch,
        Self::CapabilityRevocation,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::DeliveryClass => 0,
            Self::AtMostOnce => 1,
            Self::AtLeastOnce => 2,
            Self::IdempotentRetry => 3,
            Self::ExactlyOnceBoundary => 4,
            Self::DeliveryIdentity => 5,
            Self::IdempotenceKey => 6,
            Self::Deduplication => 7,
            Self::Ordering => 8,
            Self::Causality => 9,
            Self::Timeout => 10,
            Self::Disconnect => 11,
            Self::Duplicate => 12,
            Self::Reorder => 13,
            Self::StaleIncarnation => 14,
            Self::RemoteRestart => 15,
            Self::SchemaMismatch => 16,
            Self::CapabilityRevocation => 17,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeliveryBoundaryInventory {
    boundaries: Box<[PlannedDeliveryBoundary]>,
}

impl DeliveryBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeliveryBoundary>,
    ) -> Result<Self, PlannedDeliveryBoundary> {
        let mut boundaries = boundaries.into_iter().collect::<Vec<_>>();
        boundaries.sort_unstable_by_key(|boundary| boundary.rank());
        let mut seen = BTreeSet::new();
        for boundary in &boundaries {
            if !seen.insert(*boundary) {
                return Err(*boundary);
            }
        }
        Ok(Self {
            boundaries: boundaries.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.remote-delivery-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_delivery_boundaries_are_complete_and_ordered() {
    let inventory = DeliveryBoundaryInventory::new(PlannedDeliveryBoundary::ALL)
        .expect("planned delivery boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedDeliveryBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..18).collect::<Vec<_>>()
    );
}

#[test]
fn delivery_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeliveryBoundaryInventory::new(PlannedDeliveryBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = DeliveryBoundaryInventory::new(PlannedDeliveryBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeliveryBoundaryInventory::new([
        PlannedDeliveryBoundary::DeliveryIdentity,
        PlannedDeliveryBoundary::DeliveryIdentity,
    ])
    .expect_err("duplicate delivery boundary must be rejected");
    assert_eq!(duplicate, PlannedDeliveryBoundary::DeliveryIdentity);
}

#[test]
fn delivery_boundary_evidence_has_no_delivery_authority() {
    let inventory = DeliveryBoundaryInventory::new([
        PlannedDeliveryBoundary::DeliveryClass,
        PlannedDeliveryBoundary::Timeout,
        PlannedDeliveryBoundary::ExactlyOnceBoundary,
    ])
    .expect("bounded delivery evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.remote-delivery-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
