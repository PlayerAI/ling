# TASK-2202 Authority Audit: Task State-Machine Lowering

## Outcome

`TASK-2202` is correctly recorded as `BlockedSpec`. The G2 plan requires every
Task suspension point to lower into a versioned state machine carrying live
locals, continuation state, cancellation, cleanup, Fault, and source-map
edges, with coverage for repeated suspension, match/loop paths, resource
cleanup, and nested scopes. `TASK-2201` has no accepted Task Core input, and
the lifecycle and state-machine ABI are not specified.

No Task lowering pass, continuation layout, state-machine bytecode instruction,
serialization/version marker, cancellation or cleanup edge, source-map rule,
diagnostic allocation, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its lowering checklist does not
  authorize a continuation representation, bytecode revision, or backend ABI.
- `TASK-2201` is `BlockedSpec`, so no accepted Task grammar or Checked Core
  node exists to lower. `docs/SEMANTICS.md` only states future Task
  lifecycle intent and reserves `CreateTask`/`AwaitTask`; v0.0.1 implements
  neither.
- `docs/ROADMAP-1.0.md` requires Task lifecycle, cancellation, cleanup,
  suspension, and deterministic-scheduler decisions before the v0.2 exit.
  `GAP-STRUCTURED-TASK-001` leaves those behaviors open, with candidate
  RFC-0008 not Accepted.
- RFC-C202 is a planning placeholder, and RFC-0001 remains a Draft baseline
  under DEC-0018. Neither fixes state-machine layout, continuation ownership,
  frame liveness, cleanup ordering, or source-map/version migration.
- RFC-0020 defines only explicit host VM cancellation for existing verified
  Seed bytecode. It does not authorize Task suspension edges, state-machine
  instructions, cancellation propagation, or cleanup semantics.

## Current implementation evidence

- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no Task Core or suspension representation. The current checked boundary
  contains no live-local set, continuation state, cleanup region, or Task
  Fault edge.
- `ling-bytecode` has versioned Seed formats and ordinary call/control-flow
  lowering, but no Task state-machine table, continuation serialization,
  suspension opcode, cancellation/cleanup edge, or Task-specific source map.
- `ling-vm` executes verified Seed bytecode with call frames and host-control
  cancellation; those existing frames are not a source Task continuation ABI
  and do not propagate child lifecycle or cleanup obligations.
- No fixture covers repeated suspension, match/loop state capture, nested Task
  scopes, cancellation or Fault edges, cleanup ordering, source-map migration,
  malformed Task bytecode, or interpreter/VM state-machine equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the accepted Task Checked Core input and suspension-point identity, live
   locals and types, continuation/frame ownership, state numbering, and
   source-span/provenance rules;
2. deterministic state-machine lowering and versioning, including entry/resume
   transitions, spawn/join, cancellation, cleanup, Fault, and normal-return
   edges; bytecode encoding, verifier limits, malformed-input rejection, and
   profile/ABI compatibility;
3. borrow, mutable-State, Effect/Capability, resource, and aliasing rules for
   values that cross suspension, plus allocation, recursion, frame, and output
   limits and their failure precedence;
4. interpreter reference semantics versus VM execution, source-map and
   diagnostic projection, Semantic IDs, Audit Source, canonical ordering,
   migration policy, and deterministic behavior under nested scopes; and
5. executable positive/negative/migration/differential fixtures for multiple
   suspension points, match branches, loops, nested scopes, cancellation
   before/after effects, cleanup on success/cancel/Fault, child Fault
   aggregation, malformed/oversized bytecode, Unicode/CRLF/BOM spans, and no
   unchecked-AST execution.

Until these decisions are Accepted, a lowering could drop live state, resume
with invalid ownership, skip cleanup, mis-map cancellation/Faults, or make
interpreter and VM behavior diverge across a bytecode version boundary.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0018, RFC-0001,
RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, bytecode, evaluator, and VM
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, scheduler, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`TASK-2202` can begin only after TASK-2201 and an Accepted RFC-0008 (or
replacement) resolve `GAP-STRUCTURED-TASK-001` and define a versioned
state-machine/continuation ABI. The future lowering must consume checked Task
Core only, preserve live values and source identity, make every cancellation,
cleanup, and Fault edge explicit, and publish interpreter/VM differential
evidence before exposing a new bytecode revision.
