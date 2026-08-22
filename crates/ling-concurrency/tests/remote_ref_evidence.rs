//! Internal RemoteRef and endpoint boundary evidence.
//!
//! This test-only inventory names proposed remote-actor identity boundaries.
//! It does not define a RemoteRef type, serialize references, authenticate
//! endpoints, send effects, or implement delivery semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedRemoteBoundary {
    LocalReferenceSeparation,
    RemoteReferenceIdentity,
    EndpointIdentity,
    RemoteActorIdentity,
    ProtocolVersion,
    CapabilityToken,
    EndpointAuthority,
    ProtocolNegotiation,
    NetworkEffect,
    ActorSendEffect,
    DeliveryOutcome,
    FaultOutcome,
    Incarnation,
    SerializationBoundary,
}

impl PlannedRemoteBoundary {
    const ALL: [Self; 14] = [
        Self::LocalReferenceSeparation,
        Self::RemoteReferenceIdentity,
        Self::EndpointIdentity,
        Self::RemoteActorIdentity,
        Self::ProtocolVersion,
        Self::CapabilityToken,
        Self::EndpointAuthority,
        Self::ProtocolNegotiation,
        Self::NetworkEffect,
        Self::ActorSendEffect,
        Self::DeliveryOutcome,
        Self::FaultOutcome,
        Self::Incarnation,
        Self::SerializationBoundary,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::LocalReferenceSeparation => 0,
            Self::RemoteReferenceIdentity => 1,
            Self::EndpointIdentity => 2,
            Self::RemoteActorIdentity => 3,
            Self::ProtocolVersion => 4,
            Self::CapabilityToken => 5,
            Self::EndpointAuthority => 6,
            Self::ProtocolNegotiation => 7,
            Self::NetworkEffect => 8,
            Self::ActorSendEffect => 9,
            Self::DeliveryOutcome => 10,
            Self::FaultOutcome => 11,
            Self::Incarnation => 12,
            Self::SerializationBoundary => 13,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RemoteBoundaryInventory {
    boundaries: Box<[PlannedRemoteBoundary]>,
}

impl RemoteBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedRemoteBoundary>,
    ) -> Result<Self, PlannedRemoteBoundary> {
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
        bytes.extend_from_slice(b"ling.remote-ref-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_remote_boundaries_are_complete_and_ordered() {
    let inventory = RemoteBoundaryInventory::new(PlannedRemoteBoundary::ALL)
        .expect("planned remote boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedRemoteBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..14).collect::<Vec<_>>()
    );
}

#[test]
fn remote_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = RemoteBoundaryInventory::new(PlannedRemoteBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = RemoteBoundaryInventory::new(PlannedRemoteBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = RemoteBoundaryInventory::new([
        PlannedRemoteBoundary::EndpointIdentity,
        PlannedRemoteBoundary::EndpointIdentity,
    ])
    .expect_err("duplicate remote boundary must be rejected");
    assert_eq!(duplicate, PlannedRemoteBoundary::EndpointIdentity);
}

#[test]
fn remote_boundary_evidence_has_no_remote_protocol_authority() {
    let inventory = RemoteBoundaryInventory::new([
        PlannedRemoteBoundary::LocalReferenceSeparation,
        PlannedRemoteBoundary::EndpointIdentity,
        PlannedRemoteBoundary::SerializationBoundary,
    ])
    .expect("bounded remote evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.remote-ref-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
