use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedTrustedCompilerRouteBoundary {
    TrustedCompilerRoute,
    Version,
    RestrictedVerifiedBackend,
    TranslationValidation,
    ProofProducingLowering,
    ControlledCSubsetBridge,
    MachineCodeVerification,
    CriticalCore,
    LimitedTarget,
    RouteSelection,
    SelectionCriteria,
    RejectedAlternative,
    Compatibility,
    Migration,
    CheckedTypedCoreInput,
    DerivedVerifiedRepresentation,
    NativeIr,
    IrValidity,
    TypeLayout,
    CallingConvention,
    OwnershipResource,
    EffectsCapabilities,
    Ffi,
    FaultUnwinding,
    ThreadReentry,
    StartupShutdown,
    TargetPrimitivePackage,
    TargetIdentity,
    ToolchainIdentity,
    ProfileIdentity,
    BuildIdentity,
    ArtifactIdentity,
    SourceBinaryMapping,
    Equivalence,
    ProofObligation,
    Certificate,
    IndependentChecker,
    TrustBoundary,
    TcbIdentity,
    Assumption,
    OptimizationBoundary,
    FailClosed,
    UnsupportedConstruct,
    InvalidLowering,
    AbiMismatch,
    TargetMismatch,
    ProofFailure,
    CertificateFailure,
    VerifierUnavailable,
    UnsafeBridge,
    Unavailable,
    Unsupported,
    DifferentialFixture,
    TranslationFixture,
    ProofFixture,
    ReproducibilityFixture,
    UnicodeFixture,
    SourceSpanFixture,
    ProtocolInventory,
    SupportMatrix,
}

impl PlannedTrustedCompilerRouteBoundary {
    const ALL: [Self; 60] = [
        Self::TrustedCompilerRoute,
        Self::Version,
        Self::RestrictedVerifiedBackend,
        Self::TranslationValidation,
        Self::ProofProducingLowering,
        Self::ControlledCSubsetBridge,
        Self::MachineCodeVerification,
        Self::CriticalCore,
        Self::LimitedTarget,
        Self::RouteSelection,
        Self::SelectionCriteria,
        Self::RejectedAlternative,
        Self::Compatibility,
        Self::Migration,
        Self::CheckedTypedCoreInput,
        Self::DerivedVerifiedRepresentation,
        Self::NativeIr,
        Self::IrValidity,
        Self::TypeLayout,
        Self::CallingConvention,
        Self::OwnershipResource,
        Self::EffectsCapabilities,
        Self::Ffi,
        Self::FaultUnwinding,
        Self::ThreadReentry,
        Self::StartupShutdown,
        Self::TargetPrimitivePackage,
        Self::TargetIdentity,
        Self::ToolchainIdentity,
        Self::ProfileIdentity,
        Self::BuildIdentity,
        Self::ArtifactIdentity,
        Self::SourceBinaryMapping,
        Self::Equivalence,
        Self::ProofObligation,
        Self::Certificate,
        Self::IndependentChecker,
        Self::TrustBoundary,
        Self::TcbIdentity,
        Self::Assumption,
        Self::OptimizationBoundary,
        Self::FailClosed,
        Self::UnsupportedConstruct,
        Self::InvalidLowering,
        Self::AbiMismatch,
        Self::TargetMismatch,
        Self::ProofFailure,
        Self::CertificateFailure,
        Self::VerifierUnavailable,
        Self::UnsafeBridge,
        Self::Unavailable,
        Self::Unsupported,
        Self::DifferentialFixture,
        Self::TranslationFixture,
        Self::ProofFixture,
        Self::ReproducibilityFixture,
        Self::UnicodeFixture,
        Self::SourceSpanFixture,
        Self::ProtocolInventory,
        Self::SupportMatrix,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TrustedCompilerRouteInventory {
    boundaries: Box<[PlannedTrustedCompilerRouteBoundary]>,
}

impl TrustedCompilerRouteInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTrustedCompilerRouteBoundary>,
    ) -> Result<Self, PlannedTrustedCompilerRouteBoundary> {
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
        let mut bytes = b"ling.trusted-compiler-route-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_trusted_compiler_route_boundaries_are_complete_and_ordered() {
    let inventory = TrustedCompilerRouteInventory::new(PlannedTrustedCompilerRouteBoundary::ALL)
        .expect("planned trusted-compiler-route boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTrustedCompilerRouteBoundary::ALL
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
fn trusted_compiler_route_evidence_is_order_independent_and_duplicate_checked() {
    let forward = TrustedCompilerRouteInventory::new(PlannedTrustedCompilerRouteBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = TrustedCompilerRouteInventory::new(
        PlannedTrustedCompilerRouteBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TrustedCompilerRouteInventory::new([
        PlannedTrustedCompilerRouteBoundary::TrustedCompilerRoute,
        PlannedTrustedCompilerRouteBoundary::TrustedCompilerRoute,
    ])
    .expect_err("duplicate trusted-compiler-route boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedTrustedCompilerRouteBoundary::TrustedCompilerRoute
    );
}

#[test]
fn trusted_compiler_route_evidence_selects_no_route_or_support_claim() {
    let inventory = TrustedCompilerRouteInventory::new([
        PlannedTrustedCompilerRouteBoundary::TrustedCompilerRoute,
        PlannedTrustedCompilerRouteBoundary::RestrictedVerifiedBackend,
        PlannedTrustedCompilerRouteBoundary::TranslationValidation,
        PlannedTrustedCompilerRouteBoundary::ProofProducingLowering,
        PlannedTrustedCompilerRouteBoundary::ControlledCSubsetBridge,
        PlannedTrustedCompilerRouteBoundary::MachineCodeVerification,
        PlannedTrustedCompilerRouteBoundary::CheckedTypedCoreInput,
        PlannedTrustedCompilerRouteBoundary::SupportMatrix,
        PlannedTrustedCompilerRouteBoundary::ProtocolInventory,
    ])
    .expect("bounded trusted-compiler-route evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.trusted-compiler-route-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 9);
}
