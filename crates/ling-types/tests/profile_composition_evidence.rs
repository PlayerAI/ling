use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedProfileCompositionBoundary {
    ProfileComposition,
    LayerIdentity,
    BaseProfile,
    TargetProfile,
    MissionConstraints,
    SchemaVersion,
    CompilerRange,
    StandardLibrarySet,
    TargetArchitecture,
    Scheduler,
    CapabilityPolicy,
    EffectPolicy,
    MemoryPolicy,
    NumericPolicy,
    ConcurrencyPolicy,
    FfiPolicy,
    VerificationRequirements,
    FieldPresence,
    DefaultPolicy,
    UnknownFieldPolicy,
    MergeOperator,
    Precedence,
    Override,
    Intersection,
    Subtraction,
    Monotonicity,
    ConflictClass,
    IncompatibleTarget,
    IncompatiblePackage,
    ImpossibleConstraint,
    IdentityChange,
    Migration,
    CanonicalOrdering,
    CanonicalBytes,
    EffectiveProfile,
    ProfileDigest,
    BuildIdentity,
    CacheKey,
    ProgramId,
    SemanticGraph,
    ReplayIdentity,
    ReproducibleBuild,
    ConfigPrecedence,
    ProjectConfig,
    CliConfig,
    TargetScope,
    MissionScope,
    SourceVisibility,
    DiagnosticCode,
    DiagnosticFacts,
    SourceSpan,
    PositiveFixture,
    NegativeFixture,
    LayerOrderFixture,
    ConflictFixture,
    IdentityMigrationFixture,
    CacheReplayFixture,
    DifferentialFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedProfileCompositionBoundary {
    const ALL: [Self; 60] = [
        Self::ProfileComposition,
        Self::LayerIdentity,
        Self::BaseProfile,
        Self::TargetProfile,
        Self::MissionConstraints,
        Self::SchemaVersion,
        Self::CompilerRange,
        Self::StandardLibrarySet,
        Self::TargetArchitecture,
        Self::Scheduler,
        Self::CapabilityPolicy,
        Self::EffectPolicy,
        Self::MemoryPolicy,
        Self::NumericPolicy,
        Self::ConcurrencyPolicy,
        Self::FfiPolicy,
        Self::VerificationRequirements,
        Self::FieldPresence,
        Self::DefaultPolicy,
        Self::UnknownFieldPolicy,
        Self::MergeOperator,
        Self::Precedence,
        Self::Override,
        Self::Intersection,
        Self::Subtraction,
        Self::Monotonicity,
        Self::ConflictClass,
        Self::IncompatibleTarget,
        Self::IncompatiblePackage,
        Self::ImpossibleConstraint,
        Self::IdentityChange,
        Self::Migration,
        Self::CanonicalOrdering,
        Self::CanonicalBytes,
        Self::EffectiveProfile,
        Self::ProfileDigest,
        Self::BuildIdentity,
        Self::CacheKey,
        Self::ProgramId,
        Self::SemanticGraph,
        Self::ReplayIdentity,
        Self::ReproducibleBuild,
        Self::ConfigPrecedence,
        Self::ProjectConfig,
        Self::CliConfig,
        Self::TargetScope,
        Self::MissionScope,
        Self::SourceVisibility,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::SourceSpan,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::LayerOrderFixture,
        Self::ConflictFixture,
        Self::IdentityMigrationFixture,
        Self::CacheReplayFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProfileCompositionInventory {
    boundaries: Box<[PlannedProfileCompositionBoundary]>,
}

impl ProfileCompositionInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedProfileCompositionBoundary>,
    ) -> Result<Self, PlannedProfileCompositionBoundary> {
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
        let mut bytes = b"ling.profile-composition-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_profile_composition_boundaries_are_complete_and_ordered() {
    let inventory = ProfileCompositionInventory::new(PlannedProfileCompositionBoundary::ALL)
        .expect("planned profile composition boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedProfileCompositionBoundary::ALL
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
fn profile_composition_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ProfileCompositionInventory::new(PlannedProfileCompositionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ProfileCompositionInventory::new(PlannedProfileCompositionBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ProfileCompositionInventory::new([
        PlannedProfileCompositionBoundary::ProfileComposition,
        PlannedProfileCompositionBoundary::ProfileComposition,
    ])
    .expect_err("duplicate profile composition boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedProfileCompositionBoundary::ProfileComposition
    );
}

#[test]
fn profile_composition_evidence_has_no_composition_authority() {
    let inventory = ProfileCompositionInventory::new([
        PlannedProfileCompositionBoundary::ProfileComposition,
        PlannedProfileCompositionBoundary::LayerIdentity,
        PlannedProfileCompositionBoundary::MergeOperator,
        PlannedProfileCompositionBoundary::ConflictClass,
        PlannedProfileCompositionBoundary::CanonicalBytes,
        PlannedProfileCompositionBoundary::EffectiveProfile,
        PlannedProfileCompositionBoundary::ProgramId,
        PlannedProfileCompositionBoundary::DiagnosticCode,
    ])
    .expect("bounded profile composition evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.profile-composition-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
