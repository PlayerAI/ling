# TASK-2202 Authority Audit: Task State-Machine Lowering

## Outcome

`TASK-2202` is `Done` under Accepted DEC-0265. Accepted `DEC-0092` closes
the bounded publish-disabled `TASK-2202-STATE-MACHINE-MODEL` identity-graph
child, while DEC-0265 authorizes the checked-only lowering slice. The G2 plan requires every
Task suspension point to lower into a versioned state machine carrying live
locals, continuation state, cancellation, cleanup, Fault, and source-map
edges, with coverage for repeated suspension, match/loop paths, resource
cleanup, and nested scopes. `TASK-2201` now provides the Accepted checked Task
Core input. DEC-0265 fixes its non-executable state-machine representation;
the later executable lifecycle/backend ABI remain unavailable until their
decisions are Accepted. Implementation commit
`450ec1bad6403a03a702713d80464fa6bbd83172` completes only that checked
lowering boundary.

No state-machine bytecode instruction, executable continuation layout, runtime
cancellation or cleanup behavior, diagnostic allocation, public schema, or
placeholder G2 execution API was added. The checked machine records structural
state and edge obligations but cannot execute them.

## Normative traceability

- The G2 execution package is non-normative; its lowering checklist does not
  authorize a continuation representation, bytecode revision, or backend ABI.
- `TASK-2201` is complete under Accepted DEC-0264 and provides immutable
  checked scope/spawn/suspension graphs plus live-set evidence. DEC-0264
  explicitly forbids executable lowering, so it does not authorize a
  continuation ABI, Task bytecode, or runtime lifecycle.
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
- Accepted `DEC-0092` is intentionally narrower than the missing authority: it
  validates only opaque state/local/transition identities and deterministic
  checked-data bytes.
- Accepted `DEC-0265` supplies the TASK-2202 boundary: it fixes
  `ling.task-machine/0.1`, typed suspension frames, branch-aware control-flow
  edges, reasoned cleanup exits, validation, canonical bytes, source maps, and
  the continuing non-executable boundary.

## Current implementation evidence

- `ling-effects` now atomically lowers successful `CheckedTaskCore` values to
  validated immutable `ling.task-machine/0.1` machines owned by
  `CheckedProgram`. Suspension states carry exact checked continuation, scope,
  awaited-task, sorted typed frame, and source-span evidence.
- Normal evaluation-order edges preserve sequences, branches, matches, and
  nested lexical-scope returns. Every active state has explicit structural
  cancel/Fault exits and every reasoned cleanup state has one matching terminal
  edge.
- `ling-concurrency::StateMachineModel` is used only as a structural projection;
  neither representation is connected to bytecode, the verifier, interpreter,
  VM, or a scheduler and neither defines executable suspension semantics.
- `ling-bytecode` has versioned Seed formats and ordinary call/control-flow
  lowering, but no Task state-machine table, continuation serialization,
  suspension opcode, cancellation/cleanup edge, or Task-specific source map.
- `ling-vm` executes verified Seed bytecode with call frames and host-control
  cancellation; those existing frames are not a source Task continuation ABI
  and do not propagate child lifecycle or cleanup obligations.
- TASK-2202 fixtures cover zero/repeated suspension, branch and match topology,
  nested scope-local continuation, exact frames and spans, explicit structural
  exits, malformed internal models, deterministic bytes, and a synthetic
  checked-loop model boundary. Executable cancellation, cleanup ordering,
  Task-bytecode migration, malformed Task bytecode, and interpreter/VM
  equivalence remain unavailable.

## Accepted implementation contract

DEC-0265 defines the required TASK-2202 contract:

1. the accepted Task Checked Core input and suspension-point identity, live
   locals and types, continuation/frame ownership, state numbering, and
   source-span/provenance rules;
2. deterministic internal state-machine lowering and versioning, including
   entry/continue/resume, cancellation, cleanup, Fault, and normal-return
   edges, malformed-input rejection, and canonical compatibility;
3. exact reuse of DEC-0264 suspension-safe typed live bindings without choosing
   allocation, ABI offsets, runtime resource limits, or failure precedence;
4. source-map projection, canonical ordering, nested-scope and branch behavior,
   and preservation of existing Semantic Graph/Audit and execution rejection;
5. positive/negative/determinism fixtures for multiple suspension points,
   match branches, nested scopes, the synthetic loop boundary, structural
   return/cancel/Fault cleanup, Unicode/CRLF/BOM spans, and no unchecked-AST
   consumption or execution.

Runtime lifecycle, bytecode/VM execution, propagation, cleanup code, Fault
aggregation, precedence, scheduling, and resources remain outside TASK-2202
and require later Accepted authority.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, Accepted DEC-0264 and DEC-0265,
`docs/status/TASK-2201-IMPLEMENTATION-REPORT.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0018, RFC-0001,
RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, bytecode, evaluator, and VM
crates.

The compiler gains only the internal checked lowering and immutable accessors.
Interpreter, VM, bytecode, diagnostic, schema, Semantic ID, public source-span,
runtime, scheduler, and Unicode 17.0.0 behavior did not change.

## Intentionally deferred

The bounded checked-only lowering is complete. TASK-2203 must obtain Accepted
lifecycle authority before consuming these machines for execution. Runtime
join/cancel/cleanup/Fault semantics, interpreter/VM differential evidence, and
any new bytecode revision remain later Accepted work.
