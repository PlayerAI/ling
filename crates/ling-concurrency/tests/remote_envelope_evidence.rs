//! Internal transport-neutral envelope boundary evidence.
//!
//! This test-only inventory names proposed envelope fields and boundaries. It
//! does not define a wire format, serialize payloads, calculate checksums, or
//! implement transport behavior.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedEnvelopeBoundary {
    ProtocolVersion,
    SenderSemanticType,
    ReceiverSemanticType,
    MessageSchema,
    MessageId,
    CorrelationId,
    Deadline,
    Cancellation,
    DeliveryPolicy,
    AuthenticationMetadata,
    Payload,
    PayloadChecksum,
    ExtensionFields,
    IdentityBinding,
    IncarnationBinding,
    Integrity,
    ResourceLimits,
    Migration,
}

impl PlannedEnvelopeBoundary {
    const ALL: [Self; 18] = [
        Self::ProtocolVersion,
        Self::SenderSemanticType,
        Self::ReceiverSemanticType,
        Self::MessageSchema,
        Self::MessageId,
        Self::CorrelationId,
        Self::Deadline,
        Self::Cancellation,
        Self::DeliveryPolicy,
        Self::AuthenticationMetadata,
        Self::Payload,
        Self::PayloadChecksum,
        Self::ExtensionFields,
        Self::IdentityBinding,
        Self::IncarnationBinding,
        Self::Integrity,
        Self::ResourceLimits,
        Self::Migration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ProtocolVersion => 0,
            Self::SenderSemanticType => 1,
            Self::ReceiverSemanticType => 2,
            Self::MessageSchema => 3,
            Self::MessageId => 4,
            Self::CorrelationId => 5,
            Self::Deadline => 6,
            Self::Cancellation => 7,
            Self::DeliveryPolicy => 8,
            Self::AuthenticationMetadata => 9,
            Self::Payload => 10,
            Self::PayloadChecksum => 11,
            Self::ExtensionFields => 12,
            Self::IdentityBinding => 13,
            Self::IncarnationBinding => 14,
            Self::Integrity => 15,
            Self::ResourceLimits => 16,
            Self::Migration => 17,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EnvelopeBoundaryInventory {
    boundaries: Box<[PlannedEnvelopeBoundary]>,
}

impl EnvelopeBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedEnvelopeBoundary>,
    ) -> Result<Self, PlannedEnvelopeBoundary> {
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
        bytes.extend_from_slice(b"ling.remote-envelope-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_envelope_boundaries_are_complete_and_ordered() {
    let inventory = EnvelopeBoundaryInventory::new(PlannedEnvelopeBoundary::ALL)
        .expect("planned envelope boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedEnvelopeBoundary::ALL);
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
fn envelope_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = EnvelopeBoundaryInventory::new(PlannedEnvelopeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = EnvelopeBoundaryInventory::new(PlannedEnvelopeBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = EnvelopeBoundaryInventory::new([
        PlannedEnvelopeBoundary::MessageId,
        PlannedEnvelopeBoundary::MessageId,
    ])
    .expect_err("duplicate envelope boundary must be rejected");
    assert_eq!(duplicate, PlannedEnvelopeBoundary::MessageId);
}

#[test]
fn envelope_boundary_evidence_has_no_wire_protocol_authority() {
    let inventory = EnvelopeBoundaryInventory::new([
        PlannedEnvelopeBoundary::ProtocolVersion,
        PlannedEnvelopeBoundary::Payload,
        PlannedEnvelopeBoundary::PayloadChecksum,
    ])
    .expect("bounded envelope evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.remote-envelope-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
