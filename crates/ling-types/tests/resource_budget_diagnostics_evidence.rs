use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedResourceBudgetDiagnosticsBoundary {
    ResourceBudgetDiagnostics,
    BudgetFact,
    UsageFact,
    Contributor,
    PathProvenance,
    Assumption,
    Unknown,
    Estimate,
    Proof,
    TargetCompiler,
    MachineField,
    LocalizedText,
    BudgetUnitLimit,
    Overflow,
    Unsupported,
    TargetMismatch,
    RuntimeFallback,
    SemanticId,
    SourceSpan,
    Ordering,
    Determinism,
    DiagnosticCode,
    DiagnosticFacts,
    Severity,
    Localization,
    SchemaVersion,
    Migration,
    PreviewContainer,
    RepairCandidate,
    TransformationBoundary,
    WorkspaceEdit,
    SemanticTransaction,
    SnapshotVersion,
    StaleResult,
    Cancellation,
    Confirmation,
    Rollback,
    OwnershipPreservation,
    EffectPreservation,
    ResourcePreservation,
    SourceMapPreservation,
    CheckedTypedCore,
    CriticalProfile,
    MemoryBudgetDependency,
    TerminationDependency,
    ContributorLimit,
    DiagnosticSizeLimit,
    PositiveFixture,
    NegativeFixture,
    BoundaryFixture,
    UnknownAssumptionFixture,
    ProvenanceFixture,
    LocalizationFixture,
    TargetMigrationFixture,
    TransactionFixture,
    RepairEquivalenceFixture,
    UnicodeFixture,
    DeterminismFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedResourceBudgetDiagnosticsBoundary {
    const ALL: [Self; 60] = [
        Self::ResourceBudgetDiagnostics,
        Self::BudgetFact,
        Self::UsageFact,
        Self::Contributor,
        Self::PathProvenance,
        Self::Assumption,
        Self::Unknown,
        Self::Estimate,
        Self::Proof,
        Self::TargetCompiler,
        Self::MachineField,
        Self::LocalizedText,
        Self::BudgetUnitLimit,
        Self::Overflow,
        Self::Unsupported,
        Self::TargetMismatch,
        Self::RuntimeFallback,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Ordering,
        Self::Determinism,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::Severity,
        Self::Localization,
        Self::SchemaVersion,
        Self::Migration,
        Self::PreviewContainer,
        Self::RepairCandidate,
        Self::TransformationBoundary,
        Self::WorkspaceEdit,
        Self::SemanticTransaction,
        Self::SnapshotVersion,
        Self::StaleResult,
        Self::Cancellation,
        Self::Confirmation,
        Self::Rollback,
        Self::OwnershipPreservation,
        Self::EffectPreservation,
        Self::ResourcePreservation,
        Self::SourceMapPreservation,
        Self::CheckedTypedCore,
        Self::CriticalProfile,
        Self::MemoryBudgetDependency,
        Self::TerminationDependency,
        Self::ContributorLimit,
        Self::DiagnosticSizeLimit,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::BoundaryFixture,
        Self::UnknownAssumptionFixture,
        Self::ProvenanceFixture,
        Self::LocalizationFixture,
        Self::TargetMigrationFixture,
        Self::TransactionFixture,
        Self::RepairEquivalenceFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResourceBudgetDiagnosticsInventory {
    boundaries: Box<[PlannedResourceBudgetDiagnosticsBoundary]>,
}

impl ResourceBudgetDiagnosticsInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedResourceBudgetDiagnosticsBoundary>,
    ) -> Result<Self, PlannedResourceBudgetDiagnosticsBoundary> {
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
        let mut bytes = b"ling.resource-budget-diagnostics-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_resource_budget_diagnostic_boundaries_are_complete_and_ordered() {
    let inventory =
        ResourceBudgetDiagnosticsInventory::new(PlannedResourceBudgetDiagnosticsBoundary::ALL)
            .expect("planned resource-budget diagnostic boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedResourceBudgetDiagnosticsBoundary::ALL
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
fn resource_budget_diagnostics_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        ResourceBudgetDiagnosticsInventory::new(PlannedResourceBudgetDiagnosticsBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = ResourceBudgetDiagnosticsInventory::new(
        PlannedResourceBudgetDiagnosticsBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ResourceBudgetDiagnosticsInventory::new([
        PlannedResourceBudgetDiagnosticsBoundary::ResourceBudgetDiagnostics,
        PlannedResourceBudgetDiagnosticsBoundary::ResourceBudgetDiagnostics,
    ])
    .expect_err("duplicate resource-budget diagnostic boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedResourceBudgetDiagnosticsBoundary::ResourceBudgetDiagnostics
    );
}

#[test]
fn resource_budget_diagnostics_evidence_has_no_diagnostic_authority() {
    let inventory = ResourceBudgetDiagnosticsInventory::new([
        PlannedResourceBudgetDiagnosticsBoundary::ResourceBudgetDiagnostics,
        PlannedResourceBudgetDiagnosticsBoundary::BudgetFact,
        PlannedResourceBudgetDiagnosticsBoundary::Unknown,
        PlannedResourceBudgetDiagnosticsBoundary::RepairCandidate,
        PlannedResourceBudgetDiagnosticsBoundary::DiagnosticCode,
        PlannedResourceBudgetDiagnosticsBoundary::ProtocolInventory,
    ])
    .expect("bounded resource-budget diagnostic evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.resource-budget-diagnostics-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
