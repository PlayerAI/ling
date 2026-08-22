//! Internal Managed/Native/FFI boundary evidence.
//!
//! This test-only inventory names proposed interop boundaries. It does not
//! implement handles, pinning, callbacks, thread attachment, ABI, foreign
//! ownership, collection during FFI, or runtime semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedFfiBoundary {
    Pin,
    Unpin,
    PinNesting,
    HandleTable,
    HandleGeneration,
    StaleHandle,
    NoRawPointerEscape,
    BorrowedView,
    CallbackRoot,
    ThreadAttachment,
    ThreadDetachment,
    ForeignValueOwnership,
    ForeignManagedOwnership,
    ForeignResourceOwnership,
    OpaqueHandleOwnership,
    OwnershipTransfer,
    OwnershipBorrowReturn,
    ForeignRelease,
    CleanupFailure,
    FinalizerSeparation,
    GcDuringFfi,
    CallbackAllocation,
    CallbackCollection,
    CallbackBlocking,
    CallbackCancellation,
    CallbackFault,
    CallbackReentry,
    ActorTurnInvariant,
    TaskInvariant,
    AbiLayout,
    CallingConvention,
    AlignmentEndianness,
    FaultUnwind,
    TargetPrimitive,
    CapabilityTcb,
    FfiSchemaVersion,
    ExploreProfile,
    NativeIslandProfile,
    CriticalNoGc,
    InterpreterVmNativeDifferential,
    UnicodeSourceSpans,
    DeterministicEvidence,
    SecuritySanitizer,
}

impl PlannedFfiBoundary {
    const ALL: [Self; 43] = [
        Self::Pin,
        Self::Unpin,
        Self::PinNesting,
        Self::HandleTable,
        Self::HandleGeneration,
        Self::StaleHandle,
        Self::NoRawPointerEscape,
        Self::BorrowedView,
        Self::CallbackRoot,
        Self::ThreadAttachment,
        Self::ThreadDetachment,
        Self::ForeignValueOwnership,
        Self::ForeignManagedOwnership,
        Self::ForeignResourceOwnership,
        Self::OpaqueHandleOwnership,
        Self::OwnershipTransfer,
        Self::OwnershipBorrowReturn,
        Self::ForeignRelease,
        Self::CleanupFailure,
        Self::FinalizerSeparation,
        Self::GcDuringFfi,
        Self::CallbackAllocation,
        Self::CallbackCollection,
        Self::CallbackBlocking,
        Self::CallbackCancellation,
        Self::CallbackFault,
        Self::CallbackReentry,
        Self::ActorTurnInvariant,
        Self::TaskInvariant,
        Self::AbiLayout,
        Self::CallingConvention,
        Self::AlignmentEndianness,
        Self::FaultUnwind,
        Self::TargetPrimitive,
        Self::CapabilityTcb,
        Self::FfiSchemaVersion,
        Self::ExploreProfile,
        Self::NativeIslandProfile,
        Self::CriticalNoGc,
        Self::InterpreterVmNativeDifferential,
        Self::UnicodeSourceSpans,
        Self::DeterministicEvidence,
        Self::SecuritySanitizer,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::Pin => 0,
            Self::Unpin => 1,
            Self::PinNesting => 2,
            Self::HandleTable => 3,
            Self::HandleGeneration => 4,
            Self::StaleHandle => 5,
            Self::NoRawPointerEscape => 6,
            Self::BorrowedView => 7,
            Self::CallbackRoot => 8,
            Self::ThreadAttachment => 9,
            Self::ThreadDetachment => 10,
            Self::ForeignValueOwnership => 11,
            Self::ForeignManagedOwnership => 12,
            Self::ForeignResourceOwnership => 13,
            Self::OpaqueHandleOwnership => 14,
            Self::OwnershipTransfer => 15,
            Self::OwnershipBorrowReturn => 16,
            Self::ForeignRelease => 17,
            Self::CleanupFailure => 18,
            Self::FinalizerSeparation => 19,
            Self::GcDuringFfi => 20,
            Self::CallbackAllocation => 21,
            Self::CallbackCollection => 22,
            Self::CallbackBlocking => 23,
            Self::CallbackCancellation => 24,
            Self::CallbackFault => 25,
            Self::CallbackReentry => 26,
            Self::ActorTurnInvariant => 27,
            Self::TaskInvariant => 28,
            Self::AbiLayout => 29,
            Self::CallingConvention => 30,
            Self::AlignmentEndianness => 31,
            Self::FaultUnwind => 32,
            Self::TargetPrimitive => 33,
            Self::CapabilityTcb => 34,
            Self::FfiSchemaVersion => 35,
            Self::ExploreProfile => 36,
            Self::NativeIslandProfile => 37,
            Self::CriticalNoGc => 38,
            Self::InterpreterVmNativeDifferential => 39,
            Self::UnicodeSourceSpans => 40,
            Self::DeterministicEvidence => 41,
            Self::SecuritySanitizer => 42,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FfiBoundaryInventory {
    boundaries: Box<[PlannedFfiBoundary]>,
}

impl FfiBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFfiBoundary>,
    ) -> Result<Self, PlannedFfiBoundary> {
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
        bytes.extend_from_slice(b"ling.managed-ffi-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_ffi_boundaries_are_complete_and_ordered() {
    let inventory = FfiBoundaryInventory::new(PlannedFfiBoundary::ALL)
        .expect("planned FFI boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedFfiBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..43).collect::<Vec<_>>()
    );
}

#[test]
fn ffi_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FfiBoundaryInventory::new(PlannedFfiBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FfiBoundaryInventory::new(PlannedFfiBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FfiBoundaryInventory::new([
        PlannedFfiBoundary::HandleTable,
        PlannedFfiBoundary::HandleTable,
    ])
    .expect_err("duplicate FFI boundary must be rejected");
    assert_eq!(duplicate, PlannedFfiBoundary::HandleTable);
}

#[test]
fn ffi_evidence_has_no_abi_or_pointer_authority() {
    let inventory = FfiBoundaryInventory::new([
        PlannedFfiBoundary::Pin,
        PlannedFfiBoundary::HandleGeneration,
        PlannedFfiBoundary::NoRawPointerEscape,
        PlannedFfiBoundary::GcDuringFfi,
        PlannedFfiBoundary::AbiLayout,
        PlannedFfiBoundary::SecuritySanitizer,
    ])
    .expect("bounded FFI evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.managed-ffi-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
