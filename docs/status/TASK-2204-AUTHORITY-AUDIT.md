# TASK-2204 Authority Audit: Deterministic Task Test Scheduler

## Outcome

`TASK-2204` is correctly recorded as `BlockedSpec`. The G2 plan proposes a
test-only scheduler with seeded scheduling, a virtual clock, controlled wake
order, bounded interleaving exploration, and trace export to reproduce races,
cancellation, and cleanup paths without making production scheduling part of
Ling semantics. The Task lifecycle, suspension, cancellation, Clock, and trace
contracts required to define that scheduler are not accepted.

No Task scheduler, virtual-clock type, seed-to-order algorithm, wake queue,
interleaving explorer, trace schema, scheduler diagnostic, production API, or
placeholder G2 surface was added.

## Normative traceability

- The G2 execution package is non-normative; its test-scheduler checklist does
  not authorize Task execution, a replay protocol, or a public scheduling
  guarantee.
- TASK-2201 through TASK-2203 are `BlockedSpec`, and
  `GAP-STRUCTURED-TASK-001` leaves scope, suspension, cancellation, cleanup,
  Fault, and deterministic-scheduler behavior open. RFC-0008/RFC-C202 is not
  Accepted.
- Accepted DEC-0019 and DEC-0021 authorize deterministic internal compiler
  query scheduling only. They explicitly keep Structured Task cancellation and
  scheduler semantics separate and do not authorize a Task scheduler, virtual
  clock, interleaving trace, or public protocol.
- `docs/ROADMAP-1.0.md` requires deterministic scheduling evidence as a v0.2
  exit condition but does not define seed mapping, wake-order ties, clock
  advancement, fairness, or replay equivalence.
- RFC-0020 defines host-owned VM cancellation only; it does not provide Task
  cancellation, Clock, wake, cleanup, or test scheduling semantics.

## Current implementation evidence

- The current compiler query scheduler is an internal implementation boundary
  for immutable source/query jobs. It is not a Ling Task runtime and exposes
  no virtual clock, child lifecycle, wake order, or interleaving trace.
- `ling-eval` and `ling-vm` execute checked Seed programs without Task scopes,
  suspension points, child tokens, scheduler queues, or cleanup callbacks.
- Existing VM resource limits and host cancellation can bound an execution,
  but they do not reproduce source-level scheduling races or distinguish
  deterministic test order from production scheduling.
- No fixture or schema covers seed reproducibility, virtual-time semantics,
  controlled wake order, bounded interleavings, cancellation/cleanup traces,
  trace corruption, or equivalence to a Task interpreter/VM runtime.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the boundary between test-only scheduling and production semantics, the
   supported Task Core/runtime input, and the scheduler's ownership of scopes,
   children, suspension, cancellation, and cleanup;
2. deterministic seed interpretation, ready/wake queue ordering and tie-breaks,
   virtual-clock units/overflow/deadline rules, controlled wake injection,
   fairness assumptions, and bounded interleaving exploration/shrinking;
3. trace event vocabulary, stable Task/scope identities, source spans,
   cancellation/Fault/cleanup ordering, export format and versioning, privacy,
   corruption handling, and the exact replay/equivalence relation;
4. resource and recursion limits, failure precedence, host panic/deadlock
   containment, diagnostic codes, Semantic IDs, Audit Source, Unicode/CRLF/BOM
   span behavior, and migration policy; and
5. executable positive/negative/migration/differential fixtures for nested
   scopes, multiple wake orders, virtual timeouts, cancellation before/after
   effects, cleanup and Fault races, bounded exploration, repeated seeds,
   malformed/oversized traces, deterministic output, and no unchecked-AST
   execution, plus evidence that production scheduling remains non-semantic.

Until these decisions are Accepted, a scheduler could encode accidental wake
order as language behavior, mis-handle virtual time or cancellation races, or
publish a trace that cannot be replayed safely and deterministically.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0019, DEC-0021,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current database, evaluator, bytecode, VM, and test crates.

No compiler, interpreter, VM, bytecode, scheduler, trace protocol,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`TASK-2204` can begin only after TASK-2201 through TASK-2203 and an Accepted
RFC-0008 (or replacement) resolve `GAP-STRUCTURED-TASK-001` and define the
test/production boundary. The future scheduler must drive checked Task Core
only, use reproducible bounded seeds and virtual time, export a versioned
test-only trace, and prove cancellation/cleanup/interpreter/VM equivalence
without turning production scheduling order into language semantics.
