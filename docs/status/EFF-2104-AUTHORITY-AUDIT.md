# EFF-2104 Authority Audit: Interpreter and VM Handler Execution

## Outcome

`EFF-2104` is correctly recorded as `BlockedSpec`. The G2 plan requires an
interpreter reference path followed by VM execution and a differential corpus
for single/nested handlers, propagation, resume, handler Faults, mutable State,
and cancellation. RFC-0006 now accepts the Experimental Effect model and the
EFF-2103 bounded source/Core/HIR children preserve checked-only boundaries, but
operation dispatch, continuation representation, runtime residual-row
behavior, bytecode encoding, and runtime failure contracts remain unaccepted.

No handler evaluator, continuation object, VM instruction, bytecode version,
runtime Fault mapping, differential fixture, diagnostic allocation, or
placeholder G2 API was added. Accepted DEC-0088 and the bounded
`EFF-2104-REJECTION-GATE` child add only a negative CLI compilation fixture
proving unresolved handlers are rejected before checked snapshot publication
or execution.

## Normative traceability

- The G2 execution package is non-normative; its interpreter-first order and
  differential list do not authorize handler execution semantics or a VM ABI.
- EFF-2101 and EFF-2102 and the bounded EFF-2103 children have accepted model
  authority, but no source handler has yet become checked Typed Core. Without
  an accepted runtime/ABI contract there is no valid handler input for either
  backend.
- `docs/SEMANTICS.md` and DEC-0010 keep Seed evaluation on checked
  `ProgramSnapshot` data and define current State/Capability behavior, but do
  not define handler dispatch, continuation lifetime, or residual effects.
- DEC-0013 defines current main/runtime failure classes and exit behavior;
  RFC-0020 defines VM host cancellation only. Neither defines handler Faults,
  operation effects, or a new bytecode instruction.
- `GAP-EFFECT-HANDLER-001` is resolved for the Experimental model by RFC-0006,
  while matching execution, resumption runtime, and differential evidence
  remain open. `GAP-EFFECT-STATE-MASKING-001` is resolved by the accepted
  visible-State rule, but runtime State interaction remains unimplemented.
- Accepted DEC-0088 authorizes only the existing `L-EFFECT-0004` rejection
  through the shared CLI compiler; it does not authorize handler execution.

## Current implementation evidence

- `ling-eval` evaluates checked Seed `ProgramSnapshot` expressions and has no
  handler stack, operation dispatch, continuation/resume object, or residual
  Effect result.
- `ling-bytecode`/`ling-vm` lower and execute the accepted Seed bytecode
  instruction set. No handler instruction, effect operation table, continuation
  ABI, bytecode schema version, or source-mapped handler Fault exists.
- Existing VM cancellation and runtime Fault code is a separate host-control
  boundary; it cannot be reused as compiler Effect handling or cancellation
  semantics.
- No differential fixture covers a handler in either backend, nested dispatch,
  propagation, resume cardinality, Fault/mutable State, cancellation,
  deterministic residual rows, or malformed/unsupported handler bytecode.
- The rejection-gate child covers only the negative compile boundary and
  diagnostic serialization; it has no checked snapshot, runtime, bytecode, VM,
  Fault, cancellation, or differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. operation dispatch and handler-stack semantics, continuation capture and
   lifetime, resume linearity/multiplicity, tail/resumption recursion, nested
   handlers, propagation, and unhandled-effect behavior;
2. the checked Typed Core contract, residual Effect Row/result representation,
   State/Capability/aliasing rules, mutable-State and Fault interaction,
   cancellation and cleanup semantics, and source-span/provenance mapping;
3. interpreter reference semantics and VM lowering/ABI: instruction/table
   encoding, bytecode/schema versioning, verifier rules, resource limits,
   deterministic ordering, malformed input rejection, and profile boundaries;
4. equivalence relation for interpreter versus VM results, residual rows,
   Fault categories, cancellation, committed external effects, diagnostics,
   Semantic IDs, Audit Source, and migration; and
5. executable positive/negative/migration/differential fixtures for single and
   nested handlers, propagation, resume, handler Faults, mutable State,
   cancellation, missing/unhandled operations, malformed bytecode, resource
   limits, Unicode/CRLF/BOM spans, deterministic output, and no unchecked-AST
   execution.

Until the runtime and VM decisions are Accepted, a runtime could resume with
the wrong continuation, duplicate or lose effects, diverge between interpreter
and VM, or execute handler data that was never checked.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-eval`, `crates/ling-bytecode`, and `crates/ling-vm`.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

The bounded rejection-gate child is complete under DEC-0088. Public
`EFF-2104` can begin after the accepted EFF-2101 through EFF-2103 model slices
and explicit bytecode/VM authority. The future
implementation must use checked Typed Core only, establish the interpreter as
the reference, version and verify any VM encoding, and publish differential
evidence before exposing handler execution.
