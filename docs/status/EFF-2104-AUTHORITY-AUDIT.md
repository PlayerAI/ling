# EFF-2104 Authority Audit: Interpreter and VM Handler Execution

## Outcome

`EFF-2104` is ready for implementation under Accepted DEC-0261. EFF-2103 now
publishes checked Handler Core, RFC-0006 supplies the first-order Effect model,
and DEC-0261 fixes deep lexical dispatch, delimited continuation behavior,
State/Fault/Capability/cancellation boundaries, the interpreter oracle,
`ling.bytecode/1.3`, verifier rules, VM continuation ABI, and differential
evidence for the currently executable `Console.write` producer.

Clock/Random clauses remain checked but dormant until separate source/Core
producer authority exists. Task/Actor crossings, dynamic/user operations,
Fault catching, rollback, and Stable behavior remain outside EFF-2104.

## Normative traceability

- The G2 execution package is non-normative; its interpreter-first order and
  differential list do not authorize handler execution semantics or a VM ABI.
- EFF-2101 through EFF-2103 provide accepted model, solver, source, and checked
  Core authority. DEC-0260 supplies exact operation/binding/type/effect/Audit
  lowering and DEC-0261 supplies the previously missing runtime/ABI contract.
- `docs/SEMANTICS.md`, DEC-0010, and DEC-0261 keep evaluation on checked
  `ProgramSnapshot` data and jointly define current Handler, State, Capability,
  continuation, Fault, and cancellation composition.
- DEC-0013 and RFC-0020 retain their existing runtime-failure and host-control
  contracts; DEC-0261 composes handlers with them without making Fault or
  cancellation catchable Effects.
- `GAP-EFFECT-HANDLER-001` is resolved for the Experimental model by RFC-0006
  and DEC-0261; execution and differential implementation evidence remains to
  be produced. `GAP-EFFECT-STATE-MASKING-001` is resolved by the accepted
  visible-State rule, while its runtime evidence remains to be implemented.
- Accepted DEC-0088 remains historical rejection evidence for unchecked input;
  DEC-0261 never weakens the no-unchecked-AST execution boundary.

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

## Accepted implementation contract

Accepted DEC-0261 defines:

1. exact current operation dispatch and handler-stack semantics, delimited
   continuation capture/lifetime, Once/Many invocation, deep nested handlers,
   propagation, and the existing unmatched Console host boundary;
2. the checked Typed Core contract, State/Capability/Cell identity rules,
   mutable-State and Fault interaction, host cancellation without cleanup or
   rollback, and source-span/provenance mapping;
3. interpreter reference semantics and VM lowering/ABI: instruction/table
   encoding, bytecode/schema versioning, verifier rules, resource limits,
   deterministic ordering, malformed input rejection, and profile boundaries;
4. equivalence for interpreter versus VM results, host events, resume counts,
   Fault categories, cancellation, committed effects/mutations, source spans,
   Program IDs, and canonical bytecode; and
5. executable positive, negative, malformed, resource, cancellation, and
   differential fixtures for direct/nested/transitive handlers, resume, Fault,
   mutable State, Unicode/CRLF/BOM spans, deterministic output, and the
   no-unchecked-AST boundary.

Implementation must now follow those accepted rules exactly and publish no
partial public execution claim before interpreter, 1.3 verifier/VM, malformed
input, cancellation/resource, and differential evidence all pass.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-eval`, `crates/ling-bytecode`, and `crates/ling-vm`.

The authority milestone changes specifications and plans `ling.bytecode/1.3`;
it does not itself claim an implemented evaluator, writer, reader, verifier, or
VM. Current code continues to reject handler execution until implementation
evidence is complete.

## Intentionally deferred

The bounded rejection-gate child remains complete under DEC-0088. EFF-2104 may
now implement only DEC-0261's checked interpreter and verified bytecode/VM
slice. Clock/Random producers, Task/Actor lifecycle, Replay, Remote, Native,
GPU, FFI, migrations, and Stable compatibility remain deferred.
