use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelEffectBoundary {
    EffectRowSchema,
    CapabilityRowSchema,
    TypedCoreInput,
    DefinitionEffects,
    BindingEffects,
    ModuleCapabilities,
    EffectInference,
    CapabilityClosure,
    EntryPoint,
    HandlerEffect,
    AllowedEffect,
    ForbiddenEffect,
    IoEffect,
    NetworkEffect,
    TaskEffect,
    ActorEffect,
    DeviceEffect,
    ResourceEffect,
    ManagedEffect,
    AllocationEffect,
    MutationEffect,
    CallPropagation,
    RecursionPropagation,
    TraitDispatch,
    CapabilityPreflight,
    MissingCapability,
    ExcessCapability,
    EffectMismatch,
    CapabilityMismatch,
    ProfileScope,
    TargetScope,
    KernelProfile,
    DeviceProfile,
    RejectionCategory,
    DiagnosticCode,
    DiagnosticFacts,
    Utf8Spans,
    SemanticId,
    Unicode17,
    CanonicalOrdering,
    CrossModule,
    CrossPackage,
    GraphProjection,
    AuditProjection,
    PositiveFixture,
    NegativeFixture,
    UnknownEffect,
    UnknownCapability,
    DuplicateEntry,
    VersionCompatibility,
    Migration,
    CheckedTypedCore,
    VerifiedDerivative,
    BackendBoundary,
    HostOutputExclusion,
    ProtocolInventory,
    PublicSchemaBoundary,
    CpuReference,
    Determinism,
    Fallback,
}

impl PlannedKernelEffectBoundary {
    const ALL: [Self; 60] = [
        Self::EffectRowSchema,
        Self::CapabilityRowSchema,
        Self::TypedCoreInput,
        Self::DefinitionEffects,
        Self::BindingEffects,
        Self::ModuleCapabilities,
        Self::EffectInference,
        Self::CapabilityClosure,
        Self::EntryPoint,
        Self::HandlerEffect,
        Self::AllowedEffect,
        Self::ForbiddenEffect,
        Self::IoEffect,
        Self::NetworkEffect,
        Self::TaskEffect,
        Self::ActorEffect,
        Self::DeviceEffect,
        Self::ResourceEffect,
        Self::ManagedEffect,
        Self::AllocationEffect,
        Self::MutationEffect,
        Self::CallPropagation,
        Self::RecursionPropagation,
        Self::TraitDispatch,
        Self::CapabilityPreflight,
        Self::MissingCapability,
        Self::ExcessCapability,
        Self::EffectMismatch,
        Self::CapabilityMismatch,
        Self::ProfileScope,
        Self::TargetScope,
        Self::KernelProfile,
        Self::DeviceProfile,
        Self::RejectionCategory,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::Unicode17,
        Self::CanonicalOrdering,
        Self::CrossModule,
        Self::CrossPackage,
        Self::GraphProjection,
        Self::AuditProjection,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnknownEffect,
        Self::UnknownCapability,
        Self::DuplicateEntry,
        Self::VersionCompatibility,
        Self::Migration,
        Self::CheckedTypedCore,
        Self::VerifiedDerivative,
        Self::BackendBoundary,
        Self::HostOutputExclusion,
        Self::ProtocolInventory,
        Self::PublicSchemaBoundary,
        Self::CpuReference,
        Self::Determinism,
        Self::Fallback,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::EffectRowSchema => 0,
            Self::CapabilityRowSchema => 1,
            Self::TypedCoreInput => 2,
            Self::DefinitionEffects => 3,
            Self::BindingEffects => 4,
            Self::ModuleCapabilities => 5,
            Self::EffectInference => 6,
            Self::CapabilityClosure => 7,
            Self::EntryPoint => 8,
            Self::HandlerEffect => 9,
            Self::AllowedEffect => 10,
            Self::ForbiddenEffect => 11,
            Self::IoEffect => 12,
            Self::NetworkEffect => 13,
            Self::TaskEffect => 14,
            Self::ActorEffect => 15,
            Self::DeviceEffect => 16,
            Self::ResourceEffect => 17,
            Self::ManagedEffect => 18,
            Self::AllocationEffect => 19,
            Self::MutationEffect => 20,
            Self::CallPropagation => 21,
            Self::RecursionPropagation => 22,
            Self::TraitDispatch => 23,
            Self::CapabilityPreflight => 24,
            Self::MissingCapability => 25,
            Self::ExcessCapability => 26,
            Self::EffectMismatch => 27,
            Self::CapabilityMismatch => 28,
            Self::ProfileScope => 29,
            Self::TargetScope => 30,
            Self::KernelProfile => 31,
            Self::DeviceProfile => 32,
            Self::RejectionCategory => 33,
            Self::DiagnosticCode => 34,
            Self::DiagnosticFacts => 35,
            Self::Utf8Spans => 36,
            Self::SemanticId => 37,
            Self::Unicode17 => 38,
            Self::CanonicalOrdering => 39,
            Self::CrossModule => 40,
            Self::CrossPackage => 41,
            Self::GraphProjection => 42,
            Self::AuditProjection => 43,
            Self::PositiveFixture => 44,
            Self::NegativeFixture => 45,
            Self::UnknownEffect => 46,
            Self::UnknownCapability => 47,
            Self::DuplicateEntry => 48,
            Self::VersionCompatibility => 49,
            Self::Migration => 50,
            Self::CheckedTypedCore => 51,
            Self::VerifiedDerivative => 52,
            Self::BackendBoundary => 53,
            Self::HostOutputExclusion => 54,
            Self::ProtocolInventory => 55,
            Self::PublicSchemaBoundary => 56,
            Self::CpuReference => 57,
            Self::Determinism => 58,
            Self::Fallback => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelEffectInventory {
    boundaries: Box<[PlannedKernelEffectBoundary]>,
}

impl KernelEffectInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelEffectBoundary>,
    ) -> Result<Self, PlannedKernelEffectBoundary> {
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
        bytes.extend_from_slice(b"ling.kernel-effect-capability-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_effect_boundaries_are_complete_and_ordered() {
    let inventory = KernelEffectInventory::new(PlannedKernelEffectBoundary::ALL)
        .expect("planned Kernel Effect boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelEffectBoundary::ALL
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
fn kernel_effect_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelEffectInventory::new(PlannedKernelEffectBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelEffectInventory::new(PlannedKernelEffectBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelEffectInventory::new([
        PlannedKernelEffectBoundary::EffectRowSchema,
        PlannedKernelEffectBoundary::EffectRowSchema,
    ])
    .expect_err("duplicate Kernel Effect boundary must be rejected");
    assert_eq!(duplicate, PlannedKernelEffectBoundary::EffectRowSchema);
}

#[test]
fn kernel_effect_evidence_has_no_checker_authority() {
    let inventory = KernelEffectInventory::new([
        PlannedKernelEffectBoundary::EffectRowSchema,
        PlannedKernelEffectBoundary::CapabilityPreflight,
        PlannedKernelEffectBoundary::ForbiddenEffect,
        PlannedKernelEffectBoundary::CheckedTypedCore,
        PlannedKernelEffectBoundary::CpuReference,
        PlannedKernelEffectBoundary::DiagnosticCode,
        PlannedKernelEffectBoundary::Unicode17,
        PlannedKernelEffectBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel Effect evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-effect-capability-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
