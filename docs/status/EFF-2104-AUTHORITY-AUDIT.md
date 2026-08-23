# EFF-2104 Authority Audit: Interpreter and VM Handler Execution

## Outcome

`EFF-2104` is authorized for completion under Accepted DEC-0262. EFF-2103 now
publishes checked Handler Core, RFC-0006 supplies the first-order Effect model,
DEC-0261 fixed deep lexical dispatch and delimited continuation behavior, and
DEC-0262 supersedes its incomplete bytecode design with the accepted
irrefutable-input and `ling.bytecode/1.4` Cell/State contract. Together the
retained runtime rules and refinements fix
State/Fault/Capability/cancellation boundaries, the interpreter oracle,
verifier rules, VM continuation ABI, and differential
evidence for the currently executable `Console.write` producer.

Clock/Random clauses remain checked but dormant until separate source/Core
producer authority exists. Task/Actor crossings, dynamic/user operations,
Fault catching, rollback, and Stable behavior remain outside EFF-2104.

## Normative traceability

- The G2 execution package is non-normative; its interpreter-first order and
  differential list do not authorize handler execution semantics or a VM ABI.
- EFF-2101 through EFF-2103 provide accepted model, solver, source, and checked
  Core authority. DEC-0260 supplies exact operation/binding/type/effect/Audit
  lowering; DEC-0262 retains DEC-0261's runtime rules and supplies the complete
  bytecode 1.4 Cell/State contract.
- `docs/SEMANTICS.md`, DEC-0010, and DEC-0262 keep evaluation on checked
  `ProgramSnapshot` data and jointly define current Handler, State, Capability,
  continuation, Fault, and cancellation composition.
- DEC-0013 and RFC-0020 retain their existing runtime-failure and host-control
  contracts; DEC-0261 composes handlers with them without making Fault or
  cancellation catchable Effects.
- `GAP-EFFECT-HANDLER-001` is resolved for the Experimental model by RFC-0006
  and DEC-0261. DEC-0262 resolves the narrower specification gap
  `GAP-EFFECT-HANDLER-BYTECODE-001` by selecting bytecode 1.4 Cells, explicit
  State rows, and total binding/wildcard operation inputs. Implementation and
  conformance evidence remain incomplete, so EFF-2104 remains In Progress.
- Accepted DEC-0088 remains historical rejection evidence for unchecked input;
  DEC-0261 never weakens the no-unchecked-AST execution boundary.

## Current implementation evidence

- The committed interpreter milestone executes checked Handler Core through an
  explicit continuation machine, including deep/nested dispatch, zero/one
  resume, Once cardinality, shared lexical Cells, Fault/committed-output, and
  original UTF-8 span evidence.
- The current working tree implements the bounded immutable/irrefutable
  `ling.bytecode/1.3` slice: exact Handle encoding/decoding/disassembly,
  independent verification, unmasked Capability preflight, private VM
  continuations, deep restoration, malformed records, limits, cancellation,
  and interpreter/VM differential fixtures.
- The 1.3 lowerer fails atomically for mutable Handler captures and refutable
  clause parameters. Those failures are evidence of the wire gap, not a claim
  that the accepted source behavior is unsupported or that EFF-2104 is Done.
- The rejection-gate child covers only the negative compile boundary and
  diagnostic serialization; it has no checked snapshot, runtime, bytecode, VM,
  Fault, cancellation, or differential behavior.

## Accepted implementation contract

Accepted DEC-0262 retains the applicable DEC-0261 rules and refines them with:

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

Implementation must follow those accepted rules exactly. EFF-2104 cannot be
marked Done until bytecode 1.4 is implemented, the VM preserves shared Cell
identity and checked clause-pattern behavior, all differential evidence
passes, and the completion commit is recorded.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-eval`, `crates/ling-bytecode`, and `crates/ling-vm`.

Accepted DEC-0262 plans `ling.bytecode/1.4`; the committed interpreter and
bytecode/VM 1.3 slice provide implementation evidence without making a
complete EFF-2104 or current-1.4 claim. Bytecode 1.0 through 1.3 remain
unchanged, and older readers must reject 1.4.

## Intentionally deferred

The bounded rejection-gate child remains complete under DEC-0088. EFF-2104 may
now implement DEC-0262's checked pattern gate and verified bytecode 1.4/VM
slice. Clock/Random producers, Task/Actor lifecycle, Replay, Remote, Native,
GPU, FFI, migrations, and Stable compatibility remain deferred.
