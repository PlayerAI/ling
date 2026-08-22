use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCriticalProfileBoundary {
    CriticalProfile,
    MachineReadable,
    SchemaVersion,
    LanguageVersion,
    SpecificationVersion,
    CompilerRange,
    StandardLibrarySet,
    TargetArchitecture,
    Scheduler,
    AllowedEffects,
    AllowedCapabilities,
    MemoryPolicy,
    NumericPolicy,
    ConcurrencyPolicy,
    FfiPackages,
    VerificationRequirements,
    ForbiddenCapabilities,
    Assumed,
    Unknown,
    Proved,
    ProfileIdentity,
    CanonicalBytes,
    RequiredField,
    OptionalField,
    Default,
    UnknownField,
    Migration,
    Composition,
    Override,
    ConflictPrecedence,
    ProjectConfig,
    CliConfig,
    BuildEvidence,
    SemanticIdBinding,
    Reproducibility,
    StaticObligation,
    RuntimeCheck,
    ProofObligation,
    BoundEvidence,
    NonClaim,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    MigrationFixture,
    CompositionFixture,
    ConflictFixture,
    TargetFixture,
    EffectFixture,
    MemoryFixture,
    NumericFixture,
    ConcurrencyFixture,
    IndependentCheckerFixture,
    UnicodeFixture,
    DeterminismFixture,
    DiagnosticCode,
    ProtocolInventory,
}

impl PlannedCriticalProfileBoundary {
    const ALL: [Self; 60] = [
        Self::CriticalProfile,
        Self::MachineReadable,
        Self::SchemaVersion,
        Self::LanguageVersion,
        Self::SpecificationVersion,
        Self::CompilerRange,
        Self::StandardLibrarySet,
        Self::TargetArchitecture,
        Self::Scheduler,
        Self::AllowedEffects,
        Self::AllowedCapabilities,
        Self::MemoryPolicy,
        Self::NumericPolicy,
        Self::ConcurrencyPolicy,
        Self::FfiPackages,
        Self::VerificationRequirements,
        Self::ForbiddenCapabilities,
        Self::Assumed,
        Self::Unknown,
        Self::Proved,
        Self::ProfileIdentity,
        Self::CanonicalBytes,
        Self::RequiredField,
        Self::OptionalField,
        Self::Default,
        Self::UnknownField,
        Self::Migration,
        Self::Composition,
        Self::Override,
        Self::ConflictPrecedence,
        Self::ProjectConfig,
        Self::CliConfig,
        Self::BuildEvidence,
        Self::SemanticIdBinding,
        Self::Reproducibility,
        Self::StaticObligation,
        Self::RuntimeCheck,
        Self::ProofObligation,
        Self::BoundEvidence,
        Self::NonClaim,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MigrationFixture,
        Self::CompositionFixture,
        Self::ConflictFixture,
        Self::TargetFixture,
        Self::EffectFixture,
        Self::MemoryFixture,
        Self::NumericFixture,
        Self::ConcurrencyFixture,
        Self::IndependentCheckerFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DiagnosticCode,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CriticalProfileInventory {
    boundaries: Box<[PlannedCriticalProfileBoundary]>,
}

impl CriticalProfileInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCriticalProfileBoundary>,
    ) -> Result<Self, PlannedCriticalProfileBoundary> {
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
        let mut bytes = b"ling.critical-profile-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_critical_profile_boundaries_are_complete_and_ordered() {
    let inventory = CriticalProfileInventory::new(PlannedCriticalProfileBoundary::ALL)
        .expect("planned Critical Profile boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCriticalProfileBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..60).collect::<Vec<_>>()
    );
}

#[test]
fn critical_profile_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CriticalProfileInventory::new(PlannedCriticalProfileBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        CriticalProfileInventory::new(PlannedCriticalProfileBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CriticalProfileInventory::new([
        PlannedCriticalProfileBoundary::CriticalProfile,
        PlannedCriticalProfileBoundary::CriticalProfile,
    ])
    .expect_err("duplicate Critical Profile boundary must be rejected");
    assert_eq!(duplicate, PlannedCriticalProfileBoundary::CriticalProfile);
}

#[test]
fn critical_profile_evidence_has_no_profile_authority() {
    let inventory = CriticalProfileInventory::new([
        PlannedCriticalProfileBoundary::CriticalProfile,
        PlannedCriticalProfileBoundary::MachineReadable,
        PlannedCriticalProfileBoundary::ForbiddenCapabilities,
        PlannedCriticalProfileBoundary::Assumed,
        PlannedCriticalProfileBoundary::Proved,
        PlannedCriticalProfileBoundary::IndependentCheckerFixture,
        PlannedCriticalProfileBoundary::NonClaim,
        PlannedCriticalProfileBoundary::ProtocolInventory,
    ])
    .expect("bounded Critical Profile evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.critical-profile-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
