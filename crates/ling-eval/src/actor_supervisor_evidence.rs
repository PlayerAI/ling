//! SUP-2403 executable evidence matrix authorized by Accepted DEC-0278.
//!
//! Each case reuses a focused assertion that directly drives the real private
//! checked-Core Actor/Supervisor runtime. This module adds no production path,
//! serialized fixture, scheduler, or public observation surface.

use super::tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    ContainOneSingleFault,
    ContainOneSequentialFaults,
    RestartFreshIncarnation,
    RestartInitializerFault,
    BudgetOpenHalfOpen,
    ParentStopCancelMailboxCleanup,
    InvalidOrResourceRootFallback,
    UnicodeReconstructionDeterminism,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 8] = [
    (
        EvidenceCase::ContainOneSingleFault,
        "contain-one-single-fault",
    ),
    (
        EvidenceCase::ContainOneSequentialFaults,
        "contain-one-sequential-faults",
    ),
    (
        EvidenceCase::RestartFreshIncarnation,
        "restart-fresh-incarnation",
    ),
    (
        EvidenceCase::RestartInitializerFault,
        "restart-initializer-fault",
    ),
    (EvidenceCase::BudgetOpenHalfOpen, "budget-open-half-open"),
    (
        EvidenceCase::ParentStopCancelMailboxCleanup,
        "parent-stop-cancel-mailbox-cleanup",
    ),
    (
        EvidenceCase::InvalidOrResourceRootFallback,
        "invalid-or-resource-root-fallback",
    ),
    (
        EvidenceCase::UnicodeReconstructionDeterminism,
        "unicode-reconstruction-determinism",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 8);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0278 case occurs exactly once");
    assert_eq!(matches[0].1, expected_name);
}

fn assert_unsupported_surfaces_absent() {
    const SUPERVISOR_SOURCE: &str = include_str!("actor_supervisor.rs");
    const LIB_SOURCE: &str = include_str!("lib.rs");

    assert!(LIB_SOURCE.contains("mod actor_supervisor;"));
    assert!(!LIB_SOURCE.contains("pub mod actor_supervisor"));
    assert!(!LIB_SOURCE.contains("pub use actor_supervisor"));

    for forbidden_production_function in [
        "fn restore(",
        "fn escalate(",
        "fn restart_all(",
        "fn add_child(",
        "fn serialize(",
        "fn replay(",
    ] {
        assert!(
            !SUPERVISOR_SOURCE.contains(forbidden_production_function),
            "unsupported DEC-0278 surface must remain absent: {forbidden_production_function}"
        );
    }
    assert!(!SUPERVISOR_SOURCE.contains("serde::"));
    assert!(!SUPERVISOR_SOURCE.contains("Serialize"));
}

#[test]
fn contain_one_single_fault() {
    assert_registered(
        EvidenceCase::ContainOneSingleFault,
        "contain-one-single-fault",
    );
    tests::contain_one_fault_seals_only_the_child_and_preserves_the_sibling();
}

#[test]
fn contain_one_sequential_faults() {
    assert_registered(
        EvidenceCase::ContainOneSequentialFaults,
        "contain-one-sequential-faults",
    );
    tests::sequential_faults_can_contain_all_children_without_restart_or_double_cleanup();
}

#[test]
fn restart_fresh_incarnation() {
    assert_registered(
        EvidenceCase::RestartFreshIncarnation,
        "restart-fresh-incarnation",
    );
    tests::budgeted_restart_waits_for_backoff_and_publishes_fresh_initializer_state();
    tests::simultaneous_due_slots_restart_in_canonical_actor_type_order();
}

#[test]
fn restart_initializer_fault() {
    assert_registered(
        EvidenceCase::RestartInitializerFault,
        "restart-initializer-fault",
    );
    tests::initializer_fault_consumes_attempt_and_half_open_failure_reopens_once();
}

#[test]
fn budget_open_half_open() {
    assert_registered(EvidenceCase::BudgetOpenHalfOpen, "budget-open-half-open");
    tests::exact_window_boundary_opens_and_half_open_probe_closes_the_circuit();
}

#[test]
fn parent_stop_cancel_mailbox_cleanup() {
    assert_registered(
        EvidenceCase::ParentStopCancelMailboxCleanup,
        "parent-stop-cancel-mailbox-cleanup",
    );
    tests::stop_and_owner_cancellation_cleanup_each_live_child_once();
    tests::pending_restart_is_cancelled_without_new_actor_or_double_cleanup();
}

#[test]
fn invalid_or_resource_root_fallback() {
    assert_registered(
        EvidenceCase::InvalidOrResourceRootFallback,
        "invalid-or-resource-root-fallback",
    );
    tests::every_stale_duplicate_or_inconsistent_report_uses_root_fallback();
    tests::restart_configuration_clock_and_overflow_fail_at_the_defined_boundaries();
    tests::restart_preflight_exhaustion_is_terminal_and_attempt_free();
    assert_unsupported_surfaces_absent();
}

#[test]
fn unicode_reconstruction_determinism() {
    assert_registered(
        EvidenceCase::UnicodeReconstructionDeterminism,
        "unicode-reconstruction-determinism",
    );
    tests::unicode_bom_crlf_reconstruction_preserves_containment_projection_and_span();
    tests::unicode_bom_crlf_reconstruction_preserves_restart_projection_and_span();
}
