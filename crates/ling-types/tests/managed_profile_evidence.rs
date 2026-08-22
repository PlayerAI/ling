//! Internal Managed Profile-policy boundary evidence.
//!
//! This test-only inventory names proposed Profile and `no_gc` boundaries. It
//! does not implement profile checking, syntax, capabilities, allocation
//! restrictions, runtime assertions, diagnostics, or runtime semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedProfileBoundary {
    ProfileIdentity,
    ProfileVersion,
    TargetManifest,
    InheritanceCompatibility,
    ExploreManaged,
    NativeManagedIsland,
    CriticalManagedRestriction,
    NoGcAnnotation,
    NoGcTransitiveCall,
    NoGcClosure,
    NoGcGeneric,
    NoGcImport,
    NoGcCallback,
    NoGcTask,
    NoGcActor,
    NoGcFfi,
    AllocationLegality,
    CollectorSafepoint,
    ResourceDrop,
    DynamicCodeRestriction,
    ReflectionRestriction,
    UnboundedAllocation,
    UnboundedRecursion,
    MailboxBound,
    ForbiddenCapability,
    NativeIslandTransition,
    PinHandleAbi,
    CrossProfileCall,
    CrossProfileTransfer,
    CriticalBoundedness,
    CriticalTimingFault,
    CriticalTarget,
    CriticalSecurityTcb,
    UnsupportedBeforeExecution,
    RuntimeAssertion,
    ProfileDiagnostic,
    Migration,
    SupportMatrix,
    TypedCoreProjection,
    SemanticGraphProjection,
    AuditSourceProjection,
    SemanticId,
    UnicodeSourceSpans,
    DeterministicDifferential,
}

impl PlannedProfileBoundary {
    const ALL: [Self; 44] = [
        Self::ProfileIdentity,
        Self::ProfileVersion,
        Self::TargetManifest,
        Self::InheritanceCompatibility,
        Self::ExploreManaged,
        Self::NativeManagedIsland,
        Self::CriticalManagedRestriction,
        Self::NoGcAnnotation,
        Self::NoGcTransitiveCall,
        Self::NoGcClosure,
        Self::NoGcGeneric,
        Self::NoGcImport,
        Self::NoGcCallback,
        Self::NoGcTask,
        Self::NoGcActor,
        Self::NoGcFfi,
        Self::AllocationLegality,
        Self::CollectorSafepoint,
        Self::ResourceDrop,
        Self::DynamicCodeRestriction,
        Self::ReflectionRestriction,
        Self::UnboundedAllocation,
        Self::UnboundedRecursion,
        Self::MailboxBound,
        Self::ForbiddenCapability,
        Self::NativeIslandTransition,
        Self::PinHandleAbi,
        Self::CrossProfileCall,
        Self::CrossProfileTransfer,
        Self::CriticalBoundedness,
        Self::CriticalTimingFault,
        Self::CriticalTarget,
        Self::CriticalSecurityTcb,
        Self::UnsupportedBeforeExecution,
        Self::RuntimeAssertion,
        Self::ProfileDiagnostic,
        Self::Migration,
        Self::SupportMatrix,
        Self::TypedCoreProjection,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::SemanticId,
        Self::UnicodeSourceSpans,
        Self::DeterministicDifferential,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ProfileIdentity => 0,
            Self::ProfileVersion => 1,
            Self::TargetManifest => 2,
            Self::InheritanceCompatibility => 3,
            Self::ExploreManaged => 4,
            Self::NativeManagedIsland => 5,
            Self::CriticalManagedRestriction => 6,
            Self::NoGcAnnotation => 7,
            Self::NoGcTransitiveCall => 8,
            Self::NoGcClosure => 9,
            Self::NoGcGeneric => 10,
            Self::NoGcImport => 11,
            Self::NoGcCallback => 12,
            Self::NoGcTask => 13,
            Self::NoGcActor => 14,
            Self::NoGcFfi => 15,
            Self::AllocationLegality => 16,
            Self::CollectorSafepoint => 17,
            Self::ResourceDrop => 18,
            Self::DynamicCodeRestriction => 19,
            Self::ReflectionRestriction => 20,
            Self::UnboundedAllocation => 21,
            Self::UnboundedRecursion => 22,
            Self::MailboxBound => 23,
            Self::ForbiddenCapability => 24,
            Self::NativeIslandTransition => 25,
            Self::PinHandleAbi => 26,
            Self::CrossProfileCall => 27,
            Self::CrossProfileTransfer => 28,
            Self::CriticalBoundedness => 29,
            Self::CriticalTimingFault => 30,
            Self::CriticalTarget => 31,
            Self::CriticalSecurityTcb => 32,
            Self::UnsupportedBeforeExecution => 33,
            Self::RuntimeAssertion => 34,
            Self::ProfileDiagnostic => 35,
            Self::Migration => 36,
            Self::SupportMatrix => 37,
            Self::TypedCoreProjection => 38,
            Self::SemanticGraphProjection => 39,
            Self::AuditSourceProjection => 40,
            Self::SemanticId => 41,
            Self::UnicodeSourceSpans => 42,
            Self::DeterministicDifferential => 43,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProfileBoundaryInventory {
    boundaries: Box<[PlannedProfileBoundary]>,
}

impl ProfileBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedProfileBoundary>,
    ) -> Result<Self, PlannedProfileBoundary> {
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
        bytes.extend_from_slice(b"ling.managed-profile-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_profile_boundaries_are_complete_and_ordered() {
    let inventory = ProfileBoundaryInventory::new(PlannedProfileBoundary::ALL)
        .expect("planned Profile boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedProfileBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..44).collect::<Vec<_>>()
    );
}

#[test]
fn profile_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ProfileBoundaryInventory::new(PlannedProfileBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ProfileBoundaryInventory::new(PlannedProfileBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ProfileBoundaryInventory::new([
        PlannedProfileBoundary::ProfileIdentity,
        PlannedProfileBoundary::ProfileIdentity,
    ])
    .expect_err("duplicate Profile boundary must be rejected");
    assert_eq!(duplicate, PlannedProfileBoundary::ProfileIdentity);
}

#[test]
fn profile_evidence_has_no_checker_or_syntax_authority() {
    let inventory = ProfileBoundaryInventory::new([
        PlannedProfileBoundary::ExploreManaged,
        PlannedProfileBoundary::NativeManagedIsland,
        PlannedProfileBoundary::CriticalManagedRestriction,
        PlannedProfileBoundary::NoGcAnnotation,
        PlannedProfileBoundary::RuntimeAssertion,
        PlannedProfileBoundary::ProfileDiagnostic,
    ])
    .expect("bounded Profile evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.managed-profile-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
