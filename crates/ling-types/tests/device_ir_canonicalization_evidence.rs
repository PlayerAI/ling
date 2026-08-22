use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeviceIrCanonicalizationBoundary {
    CanonicalizationSchema,
    SchemaIdentity,
    CanonicalVersion,
    ExtensionVersion,
    IrIdentity,
    NodeOrdering,
    BlockOrdering,
    OperationOrdering,
    DependencyOrdering,
    CanonicalConstants,
    ConstantPool,
    CanonicalBytes,
    IntegerEncoding,
    FloatingEncoding,
    OpaqueEncoding,
    ShapeEncoding,
    LayoutEncoding,
    NumericMode,
    EffectEncoding,
    OwnershipEncoding,
    SynchronizationEncoding,
    FaultEncoding,
    CapabilityEncoding,
    SourceMapEncoding,
    SourceSpan,
    Utf8Spans,
    SemanticId,
    DomainSeparation,
    TargetIndependentHash,
    TargetSpecializationHash,
    TargetProfile,
    TargetFeatureSet,
    RequiredFeature,
    OptionalFeature,
    UnsupportedTarget,
    ExtensionPolicy,
    UnknownFieldReject,
    CorruptionReject,
    Redaction,
    Migration,
    VersionCompatibility,
    Determinism,
    Unicode17,
    TypedCoreInput,
    VerifiedDerivative,
    CanonicalPrecondition,
    CanonicalPostcondition,
    Provenance,
    ResourceLimit,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    MigrationFixture,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedDeviceIrCanonicalizationBoundary {
    const ALL: [Self; 60] = [
        Self::CanonicalizationSchema,
        Self::SchemaIdentity,
        Self::CanonicalVersion,
        Self::ExtensionVersion,
        Self::IrIdentity,
        Self::NodeOrdering,
        Self::BlockOrdering,
        Self::OperationOrdering,
        Self::DependencyOrdering,
        Self::CanonicalConstants,
        Self::ConstantPool,
        Self::CanonicalBytes,
        Self::IntegerEncoding,
        Self::FloatingEncoding,
        Self::OpaqueEncoding,
        Self::ShapeEncoding,
        Self::LayoutEncoding,
        Self::NumericMode,
        Self::EffectEncoding,
        Self::OwnershipEncoding,
        Self::SynchronizationEncoding,
        Self::FaultEncoding,
        Self::CapabilityEncoding,
        Self::SourceMapEncoding,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::DomainSeparation,
        Self::TargetIndependentHash,
        Self::TargetSpecializationHash,
        Self::TargetProfile,
        Self::TargetFeatureSet,
        Self::RequiredFeature,
        Self::OptionalFeature,
        Self::UnsupportedTarget,
        Self::ExtensionPolicy,
        Self::UnknownFieldReject,
        Self::CorruptionReject,
        Self::Redaction,
        Self::Migration,
        Self::VersionCompatibility,
        Self::Determinism,
        Self::Unicode17,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::CanonicalPrecondition,
        Self::CanonicalPostcondition,
        Self::Provenance,
        Self::ResourceLimit,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::MigrationFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostPathExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeviceIrCanonicalizationInventory {
    boundaries: Box<[PlannedDeviceIrCanonicalizationBoundary]>,
}

impl DeviceIrCanonicalizationInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeviceIrCanonicalizationBoundary>,
    ) -> Result<Self, PlannedDeviceIrCanonicalizationBoundary> {
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
        let mut bytes = b"ling.device-ir-canonicalization-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_device_ir_canonicalization_boundaries_are_complete_and_ordered() {
    let inventory =
        DeviceIrCanonicalizationInventory::new(PlannedDeviceIrCanonicalizationBoundary::ALL)
            .expect("planned Device IR canonicalization boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeviceIrCanonicalizationBoundary::ALL
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
fn device_ir_canonicalization_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        DeviceIrCanonicalizationInventory::new(PlannedDeviceIrCanonicalizationBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = DeviceIrCanonicalizationInventory::new(
        PlannedDeviceIrCanonicalizationBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeviceIrCanonicalizationInventory::new([
        PlannedDeviceIrCanonicalizationBoundary::CanonicalizationSchema,
        PlannedDeviceIrCanonicalizationBoundary::CanonicalizationSchema,
    ])
    .expect_err("duplicate Device IR canonicalization boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedDeviceIrCanonicalizationBoundary::CanonicalizationSchema
    );
}

#[test]
fn device_ir_canonicalization_evidence_has_no_canonicalization_authority() {
    let inventory = DeviceIrCanonicalizationInventory::new([
        PlannedDeviceIrCanonicalizationBoundary::CanonicalizationSchema,
        PlannedDeviceIrCanonicalizationBoundary::NodeOrdering,
        PlannedDeviceIrCanonicalizationBoundary::CanonicalBytes,
        PlannedDeviceIrCanonicalizationBoundary::TargetIndependentHash,
        PlannedDeviceIrCanonicalizationBoundary::TargetSpecializationHash,
        PlannedDeviceIrCanonicalizationBoundary::Migration,
        PlannedDeviceIrCanonicalizationBoundary::BilingualDiagnostic,
        PlannedDeviceIrCanonicalizationBoundary::ProtocolInventory,
    ])
    .expect("bounded Device IR canonicalization evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.device-ir-canonicalization-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
