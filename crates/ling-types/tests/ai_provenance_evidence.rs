use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedAiProvenanceBoundary {
    AiProvenance,
    Version,
    AgentIdentity,
    AgentVersion,
    ToolIdentity,
    ToolVersion,
    InputSemanticSnapshot,
    ProgramId,
    SemanticId,
    SourceId,
    SourceSpan,
    Task,
    Goal,
    ChangedSemanticNode,
    PreservedContract,
    NewObligation,
    VerificationCommand,
    VerificationResult,
    HumanReview,
    HumanReviewerIdentity,
    HumanApproval,
    AutomatedAction,
    HumanAction,
    TraceabilityOnly,
    CorrectnessClaimProhibited,
    ProofClaimProhibited,
    ApprovalInferenceProhibited,
    PromptDisclosureScope,
    SourceDisclosureScope,
    PrivateConversationExcluded,
    SecretExcluded,
    CredentialExcluded,
    PiiExcluded,
    Redaction,
    RetentionPolicy,
    AccessControl,
    BundleLinkage,
    ArtifactLinkage,
    EvidenceLinkage,
    TamperEvidence,
    Signature,
    TrustBoundary,
    TcbIdentity,
    Unknown,
    Unsupported,
    Incomplete,
    Contradictory,
    Stale,
    Malformed,
    Corrupt,
    Migration,
    FailClosed,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    RedactionFixture,
    PrivateConversationFixture,
    ApprovalFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedAiProvenanceBoundary {
    const ALL: [Self; 60] = [
        Self::AiProvenance,
        Self::Version,
        Self::AgentIdentity,
        Self::AgentVersion,
        Self::ToolIdentity,
        Self::ToolVersion,
        Self::InputSemanticSnapshot,
        Self::ProgramId,
        Self::SemanticId,
        Self::SourceId,
        Self::SourceSpan,
        Self::Task,
        Self::Goal,
        Self::ChangedSemanticNode,
        Self::PreservedContract,
        Self::NewObligation,
        Self::VerificationCommand,
        Self::VerificationResult,
        Self::HumanReview,
        Self::HumanReviewerIdentity,
        Self::HumanApproval,
        Self::AutomatedAction,
        Self::HumanAction,
        Self::TraceabilityOnly,
        Self::CorrectnessClaimProhibited,
        Self::ProofClaimProhibited,
        Self::ApprovalInferenceProhibited,
        Self::PromptDisclosureScope,
        Self::SourceDisclosureScope,
        Self::PrivateConversationExcluded,
        Self::SecretExcluded,
        Self::CredentialExcluded,
        Self::PiiExcluded,
        Self::Redaction,
        Self::RetentionPolicy,
        Self::AccessControl,
        Self::BundleLinkage,
        Self::ArtifactLinkage,
        Self::EvidenceLinkage,
        Self::TamperEvidence,
        Self::Signature,
        Self::TrustBoundary,
        Self::TcbIdentity,
        Self::Unknown,
        Self::Unsupported,
        Self::Incomplete,
        Self::Contradictory,
        Self::Stale,
        Self::Malformed,
        Self::Corrupt,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::RedactionFixture,
        Self::PrivateConversationFixture,
        Self::ApprovalFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AiProvenanceInventory {
    boundaries: Box<[PlannedAiProvenanceBoundary]>,
}

impl AiProvenanceInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedAiProvenanceBoundary>,
    ) -> Result<Self, PlannedAiProvenanceBoundary> {
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
        let mut bytes = b"ling.ai-provenance-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_ai_provenance_boundaries_are_complete_and_ordered() {
    let inventory = AiProvenanceInventory::new(PlannedAiProvenanceBoundary::ALL)
        .expect("planned AI-provenance boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedAiProvenanceBoundary::ALL
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
fn ai_provenance_evidence_is_order_independent_and_duplicate_checked() {
    let forward = AiProvenanceInventory::new(PlannedAiProvenanceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = AiProvenanceInventory::new(PlannedAiProvenanceBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = AiProvenanceInventory::new([
        PlannedAiProvenanceBoundary::AiProvenance,
        PlannedAiProvenanceBoundary::AiProvenance,
    ])
    .expect_err("duplicate AI-provenance boundary must be rejected");
    assert_eq!(duplicate, PlannedAiProvenanceBoundary::AiProvenance);
}

#[test]
fn ai_provenance_evidence_has_no_correctness_or_approval_authority() {
    let inventory = AiProvenanceInventory::new([
        PlannedAiProvenanceBoundary::AiProvenance,
        PlannedAiProvenanceBoundary::HumanApproval,
        PlannedAiProvenanceBoundary::TraceabilityOnly,
        PlannedAiProvenanceBoundary::CorrectnessClaimProhibited,
        PlannedAiProvenanceBoundary::ProofClaimProhibited,
        PlannedAiProvenanceBoundary::ApprovalInferenceProhibited,
        PlannedAiProvenanceBoundary::PrivateConversationExcluded,
        PlannedAiProvenanceBoundary::SecretExcluded,
        PlannedAiProvenanceBoundary::ProtocolInventory,
    ])
    .expect("bounded AI-provenance evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ai-provenance-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 9);
}
