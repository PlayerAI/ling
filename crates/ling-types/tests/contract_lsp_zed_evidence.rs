use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractLspZedBoundary {
    ContractLspZed,
    LspVersion,
    CapabilityNegotiation,
    Hover,
    StatusText,
    Diagnostic,
    Counterexample,
    FailedInstance,
    CodeLens,
    ProofLink,
    EvidenceLink,
    GutterStatus,
    AuditProjection,
    ImplicitCondition,
    Rename,
    TextEdit,
    ContractReference,
    Snapshot,
    SemanticTransaction,
    StaleEdit,
    Conflict,
    Utf8ByteSpan,
    LspPosition,
    Utf16Projection,
    Crlf,
    Bom,
    Unicode17,
    StableId,
    Provenance,
    Invalidation,
    UnknownData,
    StaleData,
    CorruptData,
    UnverifiableData,
    ClientFallback,
    Redaction,
    ValuePrivacy,
    Request,
    Response,
    Ordering,
    Cancellation,
    Workspace,
    Document,
    Incremental,
    JsonSchema,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    StaleVersionFixture,
    UnicodePositionFixture,
    CrlfBomFixture,
    IncrementalFixture,
    RenameConflictFixture,
    EvidenceLinkFixture,
    DeterminismFixture,
    ClientCapabilityFixture,
    ZedExtension,
    ProtocolInventory,
}

impl PlannedContractLspZedBoundary {
    const ALL: [Self; 60] = [
        Self::ContractLspZed,
        Self::LspVersion,
        Self::CapabilityNegotiation,
        Self::Hover,
        Self::StatusText,
        Self::Diagnostic,
        Self::Counterexample,
        Self::FailedInstance,
        Self::CodeLens,
        Self::ProofLink,
        Self::EvidenceLink,
        Self::GutterStatus,
        Self::AuditProjection,
        Self::ImplicitCondition,
        Self::Rename,
        Self::TextEdit,
        Self::ContractReference,
        Self::Snapshot,
        Self::SemanticTransaction,
        Self::StaleEdit,
        Self::Conflict,
        Self::Utf8ByteSpan,
        Self::LspPosition,
        Self::Utf16Projection,
        Self::Crlf,
        Self::Bom,
        Self::Unicode17,
        Self::StableId,
        Self::Provenance,
        Self::Invalidation,
        Self::UnknownData,
        Self::StaleData,
        Self::CorruptData,
        Self::UnverifiableData,
        Self::ClientFallback,
        Self::Redaction,
        Self::ValuePrivacy,
        Self::Request,
        Self::Response,
        Self::Ordering,
        Self::Cancellation,
        Self::Workspace,
        Self::Document,
        Self::Incremental,
        Self::JsonSchema,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::StaleVersionFixture,
        Self::UnicodePositionFixture,
        Self::CrlfBomFixture,
        Self::IncrementalFixture,
        Self::RenameConflictFixture,
        Self::EvidenceLinkFixture,
        Self::DeterminismFixture,
        Self::ClientCapabilityFixture,
        Self::ZedExtension,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContractLspZedInventory {
    boundaries: Box<[PlannedContractLspZedBoundary]>,
}

impl ContractLspZedInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractLspZedBoundary>,
    ) -> Result<Self, PlannedContractLspZedBoundary> {
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
        let mut bytes = b"ling.contract-lsp-zed-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_lsp_zed_boundaries_are_complete_and_ordered() {
    let inventory = ContractLspZedInventory::new(PlannedContractLspZedBoundary::ALL)
        .expect("planned Contract LSP/Zed boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractLspZedBoundary::ALL
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
fn contract_lsp_zed_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractLspZedInventory::new(PlannedContractLspZedBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ContractLspZedInventory::new(PlannedContractLspZedBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractLspZedInventory::new([
        PlannedContractLspZedBoundary::ContractLspZed,
        PlannedContractLspZedBoundary::ContractLspZed,
    ])
    .expect_err("duplicate Contract LSP/Zed boundary must be rejected");
    assert_eq!(duplicate, PlannedContractLspZedBoundary::ContractLspZed);
}

#[test]
fn contract_lsp_zed_evidence_has_no_editor_authority() {
    let inventory = ContractLspZedInventory::new([
        PlannedContractLspZedBoundary::ContractLspZed,
        PlannedContractLspZedBoundary::Hover,
        PlannedContractLspZedBoundary::SemanticTransaction,
        PlannedContractLspZedBoundary::Utf16Projection,
        PlannedContractLspZedBoundary::DiagnosticCode,
        PlannedContractLspZedBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract LSP/Zed evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-lsp-zed-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
