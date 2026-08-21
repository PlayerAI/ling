# EFF-2104 Authority Audit: Interpreter and VM Handler Execution

## Outcome

`EFF-2104` is correctly recorded as `BlockedSpec`. The G2 plan requires an
interpreter reference path followed by VM execution and a differential corpus
for single/nested handlers, propagation, resume, handler Faults, mutable State,
and cancellation. The handler Typed Core and Effect semantics are not
accepted, so no operation dispatch, continuation representation, residual-row
behavior, bytecode encoding, or runtime failure contract can be implemented.

No handler evaluator, continuation object, VM instruction, bytecode version,
runtime Fault mapping, differential fixture, diagnostic allocation, or
placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its interpreter-first order and
  differential list do not authorize handler execution semantics or a VM ABI.
- EFF-2101 through EFF-2103 are `BlockedSpec`; without an accepted Effect Row,
  handler Core, and resume model there is no valid checked input for either
  backend.
- `docs/SEMANTICS.md` and DEC-0010 keep Seed evaluation on checked
  `ProgramSnapshot` data and define current State/Capability behavior, but do
  not define handler dispatch, continuation lifetime, or residual effects.
- DEC-0013 defines current main/runtime failure classes and exit behavior;
  RFC-0020 defines VM host cancellation only. Neither defines handler Faults,
  operation effects, or a new bytecode instruction.
- `GAP-EFFECT-HANDLER-001` leaves matching, elimination, nesting, resumption,
  Capability interaction, unhandled failure, and differential evidence open.
  `GAP-EFFECT-STATE-MASKING-001` leaves mutable-State visibility/escape open.

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

Until these decisions are Accepted, a runtime could resume with the wrong
continuation, duplicate or lose effects, diverge between interpreter and VM,
or execute handler data that was never checked.

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

`EFF-2104` can begin after EFF-2101 through EFF-2103 and an Accepted RFC-0006
(or replacement), followed by explicit bytecode/VM authority. The future
implementation must use checked Typed Core only, establish the interpreter as
the reference, version and verify any VM encoding, and publish differential
evidence before exposing handler execution.
