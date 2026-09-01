# SUP-2403 Implementation Report

## Outcome

SUP-2403 is complete under Accepted DEC-0278. Implementation commit
`d175501b007cf59928c47f9221f5a451d4fd1963` adds one crate-private,
`cfg(test)` executable evidence matrix to `ling-eval`. The matrix exercises the
real checked-Core Actor/Supervisor runtime already implemented under Accepted
DEC-0274 through DEC-0277; it adds no production transition or public API.

Every public Actor-bearing execution route continues to stop at
`L-ACTOR-0002`. The matrix is in-memory test evidence, not a fixture schema,
Replay log, compatibility protocol, or backend claim.

## Normative clauses covered

- DEC-0278 clauses 1-3: `actor_supervisor_evidence.rs` is private test-only
  code and registers exactly the eight authorized case families.
- Clauses 2, 6-8: every executable outcome is asserted by focused tests that
  construct successful immutable `CheckedProgram` values and directly drive
  the private DEC-0274/DEC-0276/DEC-0277 runtime through finite explicit
  operations and bounded in-memory projections.
- Clauses 4, 10-11: the matrix covers ContainOne single/sequential Faults,
  fresh restart, initializer Fault, exact circuit-window behavior, stop and
  cancellation cleanup, invalid evidence, overflow, and resource fallback.
- Clauses 5 and 13-15: bounded source-inventory assertions prove that restore,
  escalation, group/dynamic recovery, serialization, Replay, and a public
  Supervisor module are absent. No placeholder production surface was added.
- Clause 12: containment and restart assertions reconstruct Unicode/BOM/CRLF
  inputs across source identity and insertion-order differences while
  preserving authoritative original UTF-8 Fault spans.
- Clauses 16-17: focused and full repository gates pass, evidence is bound to
  the implementation commit above, and unsupported public or future work
  remains explicitly deferred.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `contain-one-single-fault`
2. `contain-one-sequential-faults`
3. `restart-fresh-incarnation`
4. `restart-initializer-fault`
5. `budget-open-half-open`
6. `parent-stop-cancel-mailbox-cleanup`
7. `invalid-or-resource-root-fallback`
8. `unicode-reconstruction-determinism`

Each matrix test directly invokes one or more existing focused assertions that
execute the real runtime and validate the complete required outcome. The case
registry additionally proves that every authorized family occurs exactly once.
This is executable reuse permitted by DEC-0278 clause 16, not a list of test
names or a second model of Supervisor behavior.

The negative inventory checks only the private production/module source. It
proves the Supervisor module is not exported and that restore, escalation,
group restart, dynamic child addition, serialization, Replay, and Serde entry
points remain absent. The check does not manufacture unreachable variants or
public placeholders.

## Executed verification

Commands executed locally on 2026-09-01:

- `cargo test -p ling-eval --lib --locked --offline` — passed: 61 tests,
  including all eight matrix cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 61 unit,
  12 Actor runtime, 13 local scheduler, 20 Task runtime, and 14 Task scheduler
  tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test -p ling-cli --test actor_boundary --locked --offline` — passed:
  10 tests, including the unchanged public `L-ACTOR-0002` boundary.
- `cargo test --workspace --all-targets --locked --offline` — passed after the
  DEC-0278 governance-count assertions were synchronized to the accepted
  authority and lifecycle registries.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo xtask governance check-all`, `cargo xtask status verify`,
  `cargo xtask docs verify`, and `cargo xtask rc0 verify` — passed.
- `cargo clippy -p xtask --all-targets --locked --offline -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` — passed.

## Compatibility impact

- Diagnostics, schemas, Semantic IDs, protocols, package/ABI versions, stored
  data, dependencies, and migration behavior: unchanged.
- Source and tooling: unchanged; no Ling value, Effect, Capability, Actor or
  Supervisor operation, query, metric, CLI/LSP/editor route, or public Rust API
  was added.
- Runtime and determinism: production code is unchanged. The matrix observes
  only explicit logical ticks, canonical identities/order, bounded counters,
  accepted Fault projections, and original spans; it makes no cross-process,
  Replay, platform, backend, wall-clock, scheduling, or performance claim.
- Unicode remains 17.0.0. Original UTF-8 byte spans remain authoritative.

## Specification gaps and deferred work

No conflict was found inside DEC-0278's scoped private evidence contract.
`GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open for broader/public supervision,
and `GAP-DETERMINISTIC-REPLAY-001` remains Open for REP-2501 onward.

State snapshot/restore, mailbox transfer, escalation, concurrent/group or
dynamic/nested recovery, public fixtures/queries/Fault channels, Replay,
cross-process/backend differential evidence, remote execution, migration,
fairness, liveness, stress/performance guarantees, and Stable compatibility
remain intentionally deferred pending their own Accepted authority.
