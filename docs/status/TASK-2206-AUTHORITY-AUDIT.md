# TASK-2206 Authority Audit: Task Conformance and Stress Tests

## Outcome

`TASK-2206` is correctly recorded as `BlockedSpec`. The G2 plan requires
conformance and stress evidence for parent early exit, Resource release during
child cancellation, simultaneous child Faults, timeout versus normal
completion, nested scopes, invalid detach rejection, one million short tasks
under resource limits, and scheduler shutdown without lost cleanup. The Task
syntax/Core, state-machine, lifecycle, deterministic scheduler, and production
scheduler contracts needed to define expected outcomes are not accepted.

No Task conformance corpus, stress harness, resource-release oracle, Fault or
timeout precedence rule, detach diagnostic, million-task benchmark, scheduler
shutdown fixture, diagnostic allocation, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its test list does not authorize
  Task semantics, a stress-test resource budget, or a public conformance
  protocol.
- TASK-2201 through TASK-2205 are `BlockedSpec`, and
  `GAP-STRUCTURED-TASK-001` leaves parent/child lifetime, cancellation,
  Fault aggregation, detach, suspension, cleanup, and deterministic scheduler
  behavior open. RFC-0008/RFC-C202 is not Accepted.
- `docs/SEMANTICS.md` provides only future Task intent and does not settle
  timeout races, simultaneous Fault ordering, Resource cleanup visibility,
  invalid detach behavior, or shutdown guarantees. v0.0.1 excludes Task.
- `docs/ROADMAP-1.0.md` requires concurrency conformance, stress, and no
  unclassified host panic/deadlock at the v0.2 exit, but is an engineering
  gate, not a semantic oracle.
- DEC-0019/DEC-0021 cover internal compiler-query equivalence and bounded
  scheduling only; RFC-0020 covers host VM cancellation only. None defines
  Task conformance outcomes or stress protocol/versioning.

## Current implementation evidence

- The repository's conformance suites cover the accepted Seed syntax,
  project/diagnostic behavior, bytecode, and VM differential slices; no Task
  source, checked Task Core, runtime, scheduler, or Task fixture exists.
- `ling-eval` and `ling-vm` expose no Task parent/child tree, Resource
  lifecycle, timeout/Clock, detach, scheduler shutdown, or aggregated child
  Fault semantics against which a stress test could compare.
- Existing VM step/frame/heap limits and host cancellation are Seed execution
  controls. They cannot establish the one-million-task bound, cleanup order,
  cancellation race precedence, or production scheduler shutdown behavior.
- No stable trace, corpus schema, stress-result schema, or diagnostic contract
  defines how minimized failures, host panics/deadlocks, resource exhaustion,
  or deterministic replay would be reported.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the checked Task Core/runtime input and conformance oracle for values,
   Effects, Faults, cancellation, scope close, cleanup, detach, and resource
   ownership;
2. precedence and deterministic projections for parent exit, child
   cancellation, simultaneous child Faults, timeout/Clock versus normal
   completion, nested scopes, invalid detach, and scheduler shutdown;
3. Resource semantics and limits, including what the one-million-task test
   measures, allocation/step/worker quotas, cleanup guarantees, failure
   precedence, host panic/deadlock containment, and benchmark reproducibility;
4. deterministic test-scheduler and production-scheduler boundaries, seed and
   trace identity, interpreter/VM equivalence, diagnostics, source spans,
   Semantic IDs, Audit Source, schema/version migration, and privacy policy;
   and
5. executable positive/negative/migration/differential fixtures for every
   listed lifecycle race, nested scopes, valid/invalid detach, cleanup after
   cancellation/Fault/shutdown, one-million bounded tasks, repeated seeds,
   malformed traces, Unicode/CRLF/BOM spans, canonical output, and no
   unchecked-AST execution.

Until these decisions are Accepted, tests could bless the wrong Fault order or
cleanup behavior, mistake host exhaustion for a language result, or make a
non-reproducible stress failure part of compatibility claims.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0019, DEC-0021,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current conformance, evaluator, bytecode, VM, and fuzz test suites.

No compiler, interpreter, VM, bytecode, scheduler, stress corpus, trace
protocol, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`TASK-2206` can begin only after TASK-2201 through TASK-2205 and an Accepted
RFC-0008 (or replacement) resolve `GAP-STRUCTURED-TASK-001` and define the
conformance/stress oracle. The future tests must exercise checked Task Core
only, use bounded reproducible resources and traces, distinguish test from
production scheduling, retain minimized failures, and publish interpreter/VM
and cleanup/resource evidence before claiming Task compatibility.
