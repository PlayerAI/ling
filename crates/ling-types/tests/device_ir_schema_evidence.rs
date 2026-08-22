use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeviceIrSchemaBoundary {
    IrSchema,
    IrVersion,
    IrIdentity,
    Workgroup,
    Grid,
    ScalarType,
    VectorType,
    TensorType,
    TypeShape,
    AddressSpace,
    MemoryOp,
    Load,
    Store,
    MemoryBounds,
    MemoryEffects,
    ControlFlow,
    Block,
    Branch,
    Loop,
    Barrier,
    Atomic,
    MemoryOrder,
    Shape,
    Layout,
    Index,
    NumericMode,
    FloatingMode,
    IntegerMode,
    OverflowMode,
    SourceMap,
    SourceSpan,
    Utf8Spans,
    CapabilitySet,
    RequiredFeature,
    OptionalFeature,
    UnsupportedTarget,
    Fault,
    Cancellation,
    TypedCoreInput,
    VerifiedDerivative,
    OwnershipWitness,
    SynchronizationWitness,
    SemanticId,
    CanonicalOrdering,
    CanonicalConstants,
    TargetIndependentHash,
    TargetSpecializationHash,
    SchemaVersion,
    VersionCompatibility,
    CorruptionReject,
    Migration,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedDeviceIrSchemaBoundary {
    const ALL: [Self; 60] = [
        Self::IrSchema,
        Self::IrVersion,
        Self::IrIdentity,
        Self::Workgroup,
        Self::Grid,
        Self::ScalarType,
        Self::VectorType,
        Self::TensorType,
        Self::TypeShape,
        Self::AddressSpace,
        Self::MemoryOp,
        Self::Load,
        Self::Store,
        Self::MemoryBounds,
        Self::MemoryEffects,
        Self::ControlFlow,
        Self::Block,
        Self::Branch,
        Self::Loop,
        Self::Barrier,
        Self::Atomic,
        Self::MemoryOrder,
        Self::Shape,
        Self::Layout,
        Self::Index,
        Self::NumericMode,
        Self::FloatingMode,
        Self::IntegerMode,
        Self::OverflowMode,
        Self::SourceMap,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::CapabilitySet,
        Self::RequiredFeature,
        Self::OptionalFeature,
        Self::UnsupportedTarget,
        Self::Fault,
        Self::Cancellation,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::OwnershipWitness,
        Self::SynchronizationWitness,
        Self::SemanticId,
        Self::CanonicalOrdering,
        Self::CanonicalConstants,
        Self::TargetIndependentHash,
        Self::TargetSpecializationHash,
        Self::SchemaVersion,
        Self::VersionCompatibility,
        Self::CorruptionReject,
        Self::Migration,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
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
struct DeviceIrSchemaInventory {
    boundaries: Box<[PlannedDeviceIrSchemaBoundary]>,
}

impl DeviceIrSchemaInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeviceIrSchemaBoundary>,
    ) -> Result<Self, PlannedDeviceIrSchemaBoundary> {
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
        let mut bytes = b"ling.device-ir-schema-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_device_ir_schema_boundaries_are_complete_and_ordered() {
    let inventory = DeviceIrSchemaInventory::new(PlannedDeviceIrSchemaBoundary::ALL)
        .expect("planned Device IR schema boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeviceIrSchemaBoundary::ALL
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
fn device_ir_schema_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeviceIrSchemaInventory::new(PlannedDeviceIrSchemaBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DeviceIrSchemaInventory::new(PlannedDeviceIrSchemaBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeviceIrSchemaInventory::new([
        PlannedDeviceIrSchemaBoundary::IrSchema,
        PlannedDeviceIrSchemaBoundary::IrSchema,
    ])
    .expect_err("duplicate Device IR schema boundary must be rejected");
    assert_eq!(duplicate, PlannedDeviceIrSchemaBoundary::IrSchema);
}

#[test]
fn device_ir_schema_evidence_has_no_schema_authority() {
    let inventory = DeviceIrSchemaInventory::new([
        PlannedDeviceIrSchemaBoundary::IrSchema,
        PlannedDeviceIrSchemaBoundary::Workgroup,
        PlannedDeviceIrSchemaBoundary::MemoryOp,
        PlannedDeviceIrSchemaBoundary::NumericMode,
        PlannedDeviceIrSchemaBoundary::CapabilitySet,
        PlannedDeviceIrSchemaBoundary::SourceMap,
        PlannedDeviceIrSchemaBoundary::BilingualDiagnostic,
        PlannedDeviceIrSchemaBoundary::ProtocolInventory,
    ])
    .expect("bounded Device IR schema evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.device-ir-schema-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
