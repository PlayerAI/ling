//! Internal reference-transport boundary evidence.
//!
//! This test-only inventory names proposed transport and codec boundaries. It
//! does not implement loopback/TCP/QUIC, encode frames, decode business
//! messages, or expose transport Faults.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedTransportBoundary {
    TransportInterface,
    Loopback,
    TcpAdapter,
    QuicAdapter,
    Framing,
    Codec,
    DecoderBudget,
    BusinessDecodeCapability,
    EndpointNegotiation,
    VersionNegotiation,
    TypedFault,
    Timeout,
    Disconnect,
    Partition,
    Backpressure,
    Cancellation,
    Determinism,
    IndependentProcess,
}

impl PlannedTransportBoundary {
    const ALL: [Self; 18] = [
        Self::TransportInterface,
        Self::Loopback,
        Self::TcpAdapter,
        Self::QuicAdapter,
        Self::Framing,
        Self::Codec,
        Self::DecoderBudget,
        Self::BusinessDecodeCapability,
        Self::EndpointNegotiation,
        Self::VersionNegotiation,
        Self::TypedFault,
        Self::Timeout,
        Self::Disconnect,
        Self::Partition,
        Self::Backpressure,
        Self::Cancellation,
        Self::Determinism,
        Self::IndependentProcess,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::TransportInterface => 0,
            Self::Loopback => 1,
            Self::TcpAdapter => 2,
            Self::QuicAdapter => 3,
            Self::Framing => 4,
            Self::Codec => 5,
            Self::DecoderBudget => 6,
            Self::BusinessDecodeCapability => 7,
            Self::EndpointNegotiation => 8,
            Self::VersionNegotiation => 9,
            Self::TypedFault => 10,
            Self::Timeout => 11,
            Self::Disconnect => 12,
            Self::Partition => 13,
            Self::Backpressure => 14,
            Self::Cancellation => 15,
            Self::Determinism => 16,
            Self::IndependentProcess => 17,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TransportBoundaryInventory {
    boundaries: Box<[PlannedTransportBoundary]>,
}

impl TransportBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTransportBoundary>,
    ) -> Result<Self, PlannedTransportBoundary> {
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
        bytes.extend_from_slice(b"ling.remote-transport-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_transport_boundaries_are_complete_and_ordered() {
    let inventory = TransportBoundaryInventory::new(PlannedTransportBoundary::ALL)
        .expect("planned transport boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTransportBoundary::ALL
    );
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
fn transport_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = TransportBoundaryInventory::new(PlannedTransportBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = TransportBoundaryInventory::new(PlannedTransportBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TransportBoundaryInventory::new([
        PlannedTransportBoundary::Codec,
        PlannedTransportBoundary::Codec,
    ])
    .expect_err("duplicate transport boundary must be rejected");
    assert_eq!(duplicate, PlannedTransportBoundary::Codec);
}

#[test]
fn transport_boundary_evidence_has_no_transport_authority() {
    let inventory = TransportBoundaryInventory::new([
        PlannedTransportBoundary::Loopback,
        PlannedTransportBoundary::Codec,
        PlannedTransportBoundary::TypedFault,
    ])
    .expect("bounded transport evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.remote-transport-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
