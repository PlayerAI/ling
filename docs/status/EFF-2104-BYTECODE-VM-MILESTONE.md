# EFF-2104 Bytecode/VM Milestone Report

## Outcome

This milestone implements the verifier-gated immutable/irrefutable
`ling.bytecode/1.3` Handler slice and its VM continuation ABI. It is executable
evidence for EFF-2104, not a Done claim. Accepted DEC-0262 now retains this
historical slice while assigning complete shared Cell/State lowering to 1.4;
its checked-source irrefutable-parameter gate is implemented separately.

## Normative clauses covered

- DEC-0261 clauses 1–4: checked-only entry, nearest Console dispatch, deep
  lexical restoration, zero/one resume, clause-outside-selected-handler, and
  body/clause result propagation.
- DEC-0261 clauses 5–7: private Once continuation handles, unmasked Console
  Capability preflight, Fault propagation, cancellation checkpoints, and
  explicit handler-depth/continuation-frame limits.
- DEC-0261 clauses 9–13: exact format 1.3 Handle records, older-reader
  rejection, 1.3 backward reads, verifier typing/effects/capabilities, VM frame
  restoration, stable cardinality Fault facts, UTF-8 source maps, and unchanged
  1.0–1.2 lowering behavior.

## Implementation evidence

- `ling-effects` publishes deterministic exact Effect rows for every checked
  expression consumed by Handler closure lowering.
- `ling-bytecode` adds format 1.3, opcode `0x1c`, deterministic lowering,
  writing, decoding, disassembly, canonical verified re-encoding, exact clause
  order/signature/capture/effect checks, and an unmasked entry Capability fact.
- `ling-vm` constructs bounded verified Handler closures, captures only VM
  frames, keeps continuation handles private and heap-accounted, reinstalls
  deep handlers, routes nested clause completion through suspended resume
  boundaries, enforces Once cardinality, and preserves committed Fault state.
- The fuzz target now exercises the newest 1.x reader/verifier while retaining
  the existing deterministic bounded oracle.

## Tests and gates

Focused fixtures cover direct zero/one resume, lexical captures, nested and
transitive handlers, a second operation inside a resumed body, higher-order
over-resume, masked Capability preflight, Fault/committed output, handler and
continuation limits, cancellation before restoration, exact Unicode/BOM/CRLF
spans, path-independent bytes, older-version compatibility, malformed tags,
reserved bytes, captures, order/duplicates, signatures, and vector bounds.
The table-driven interpreter/VM suite compares logical events, Unit results,
Fault category/operation/span/committed projections, and deterministic Program
IDs for the representable slice.

The milestone validation runs the locked offline workspace tests, workspace
Clippy with warnings denied, CI/governance/LSP/support/status/RC0/traceability/
fuzz gates, formatting, and diff checks. Exact command results belong to the
milestone commit evidence; this report does not claim future CI execution.

## Specification gaps and compatibility impact

`GAP-EFFECT-HANDLER-BYTECODE-001` recorded two contradictions exposed during
this milestone:

1. DEC-0261 requires repeated restoration to retain the same lexical Cell
   identities and keep `State<T>` unmasked, but its exact 1.3 record has
   ordinary value captures, only the Console Effect tag, and no verifier-typed
   Cell construction/load/store or serialized State row representation.
2. DEC-0260 accepts checked clause patterns, but DEC-0261 carries only operation
   signatures and has no representation or accepted failure rule for a
   refutable parameter that does not match an operation input.

Accepted DEC-0262 resolves both questions normatively. The checked resolver now
rejects every refutable or structurally redundant operation input with
`L-EFFECT-0005` before typed or Handler Core publication; 1.3 lowering retains
only its historical mutable-capture rejection. Published protocol inventory
remains at 1.2 until the complete 1.4 implementation and evidence pass.

No diagnostic code or schema field changes. `L-RUNTIME-0001` gains only the
accepted `handler_resume_cardinality` category/operation projection. Semantic
IDs, Audit shapes, source spellings, CLI/LSP/package behavior, Rust ownership,
and Unicode 17.0.0 remain unchanged. Bytecode 1.0–1.2 writer behavior is
preserved; their readers reject 1.3 as required.

## Intentionally deferred

Shared mutable Handler capture lowering and complete committed-mutation
differential evidence remain deferred; the isolated bytecode 1.4 Cell/State
model is tracked by `EFF-2104-BYTECODE-1.4-MODEL-MILESTONE.md`. Clock/Random
producers, Many-runtime production, user operations, Task/Actor transfer,
rollback/cleanup, continuation serialization, Native/Wasm, migrations, and
Stable claims also remain deferred. EFF-2104 stays In Progress.
