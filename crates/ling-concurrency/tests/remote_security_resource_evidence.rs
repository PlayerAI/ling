//! Internal security and resource-boundary evidence.
//!
//! This test-only inventory names proposed security and resource boundaries.
//! It does not implement quotas, authentication, authorization, replay
//! protection, schema gates, decoder behavior, or remote runtime semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedSecurityResourceBoundary {
    FrameLimit,
    MessageLimit,
    DecoderDepth,
    DecoderAllocation,
    MailboxIngress,
    ConnectionLimit,
    InFlightRetry,
    ReplayWindow,
    RateLimit,
    AuthenticationHook,
    AuthorizationHook,
    TrustRoot,
    CapabilityIssuance,
    CapabilityAttenuation,
    CapabilityRevocation,
    EndpointBinding,
    PrivacyBoundary,
    ReplayProtection,
    UnknownSchema,
    MalformedSchema,
    OversizedInput,
    ResourceExhaustion,
    DuplicateReplay,
    RateExhaustion,
    DecoderFuzz,
    UnicodeSourceSpans,
    InterpreterVmDifferential,
    RuntimeDifferential,
    LoopbackTransport,
    IndependentTransport,
    BusinessCodeBoundary,
}

impl PlannedSecurityResourceBoundary {
    const ALL: [Self; 31] = [
        Self::FrameLimit,
        Self::MessageLimit,
        Self::DecoderDepth,
        Self::DecoderAllocation,
        Self::MailboxIngress,
        Self::ConnectionLimit,
        Self::InFlightRetry,
        Self::ReplayWindow,
        Self::RateLimit,
        Self::AuthenticationHook,
        Self::AuthorizationHook,
        Self::TrustRoot,
        Self::CapabilityIssuance,
        Self::CapabilityAttenuation,
        Self::CapabilityRevocation,
        Self::EndpointBinding,
        Self::PrivacyBoundary,
        Self::ReplayProtection,
        Self::UnknownSchema,
        Self::MalformedSchema,
        Self::OversizedInput,
        Self::ResourceExhaustion,
        Self::DuplicateReplay,
        Self::RateExhaustion,
        Self::DecoderFuzz,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmDifferential,
        Self::RuntimeDifferential,
        Self::LoopbackTransport,
        Self::IndependentTransport,
        Self::BusinessCodeBoundary,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::FrameLimit => 0,
            Self::MessageLimit => 1,
            Self::DecoderDepth => 2,
            Self::DecoderAllocation => 3,
            Self::MailboxIngress => 4,
            Self::ConnectionLimit => 5,
            Self::InFlightRetry => 6,
            Self::ReplayWindow => 7,
            Self::RateLimit => 8,
            Self::AuthenticationHook => 9,
            Self::AuthorizationHook => 10,
            Self::TrustRoot => 11,
            Self::CapabilityIssuance => 12,
            Self::CapabilityAttenuation => 13,
            Self::CapabilityRevocation => 14,
            Self::EndpointBinding => 15,
            Self::PrivacyBoundary => 16,
            Self::ReplayProtection => 17,
            Self::UnknownSchema => 18,
            Self::MalformedSchema => 19,
            Self::OversizedInput => 20,
            Self::ResourceExhaustion => 21,
            Self::DuplicateReplay => 22,
            Self::RateExhaustion => 23,
            Self::DecoderFuzz => 24,
            Self::UnicodeSourceSpans => 25,
            Self::InterpreterVmDifferential => 26,
            Self::RuntimeDifferential => 27,
            Self::LoopbackTransport => 28,
            Self::IndependentTransport => 29,
            Self::BusinessCodeBoundary => 30,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SecurityResourceBoundaryInventory {
    boundaries: Box<[PlannedSecurityResourceBoundary]>,
}

impl SecurityResourceBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSecurityResourceBoundary>,
    ) -> Result<Self, PlannedSecurityResourceBoundary> {
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
        bytes.extend_from_slice(b"ling.remote-security-resource-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_security_resource_boundaries_are_complete_and_ordered() {
    let inventory = SecurityResourceBoundaryInventory::new(PlannedSecurityResourceBoundary::ALL)
        .expect("planned security/resource boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSecurityResourceBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..31).collect::<Vec<_>>()
    );
}

#[test]
fn security_resource_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SecurityResourceBoundaryInventory::new(PlannedSecurityResourceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = SecurityResourceBoundaryInventory::new(
        PlannedSecurityResourceBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SecurityResourceBoundaryInventory::new([
        PlannedSecurityResourceBoundary::AuthenticationHook,
        PlannedSecurityResourceBoundary::AuthenticationHook,
    ])
    .expect_err("duplicate security/resource boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedSecurityResourceBoundary::AuthenticationHook
    );
}

#[test]
fn security_resource_evidence_has_no_security_or_resource_authority() {
    let inventory = SecurityResourceBoundaryInventory::new([
        PlannedSecurityResourceBoundary::FrameLimit,
        PlannedSecurityResourceBoundary::AuthenticationHook,
        PlannedSecurityResourceBoundary::UnknownSchema,
        PlannedSecurityResourceBoundary::DecoderFuzz,
    ])
    .expect("bounded security/resource evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.remote-security-resource-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
