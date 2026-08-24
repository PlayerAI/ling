# EFF-2104 Bytecode 1.4 Cell/State Model Milestone

## Outcome

This milestone implements DEC-0262 phases 2 and 3: the isolated
`ling.bytecode/1.4` Cell/State wire model, versioned codec and disassembly, and
independent verification boundary. It also installs the non-cyclic private VM
Cell primitive needed by later shared mutable Handler lowering. EFF-2104
remains In Progress; this report does not publish 1.4 as the current protocol.

## Normative clauses covered

- DEC-0262 clauses 2–5: backward-reading revision 1.4, `Cell<T>` tag `0x14`,
  `CellNew`/`CellGet`/`CellSet` opcodes `0x1d`–`0x1f`, and canonical typed
  `State<T>` Effect records.
- DEC-0262 clauses 3 and 7: Cell containment, exact instruction typing, SSA and
  source-map coverage, State propagation, unmasked State rows, and unchanged
  Console Capability closure.
- DEC-0262 clauses 6 and 8 at the runtime primitive boundary: private Cell
  identity, shared cloning behavior, heap accounting, commit-before-Unit
  mutation, and unchanged 1.0–1.3 bytes/readers.

## Implementation evidence

- `ling-bytecode` exposes dedicated 1.4 lower/writer/reader/disassembly APIs
  while every older entry point remains pinned to its exact revision.
- The model adds only typed `TypeIndex` and `RegisterIndex` operands; no host
  address, Rust ownership, allocation order, or VM identity is serialized.
- Version-aware Effect records retain the exact one-byte Console encoding for
  1.0–1.3 and encode State as tag `2` plus its source-value type in 1.4.
- The verifier rejects forward/nested Cells, aggregate/function/source escape,
  invalid capture positions, malformed rows, wrong instruction types,
  missing/excess State, incomplete source maps, and unsupported old readers
  before verified publication.
- `ling-vm` represents a Cell value only as a private integer ID into an
  Engine-owned store. The store owns the Ling value and a 24-byte logical heap
  charge, so closures containing their own Cell ID do not form Rust reference
  cycles. CellSet checks cancellation, replaces the value, marks mutation
  committed, and only then publishes Unit.

## Tests and compatibility impact

The frozen 599-byte Cell/State artifact and exact disassembly cover all new
tags and canonical re-encoding. Focused tests cover 1.4 reading 1.0–1.4,
1.0–1.3 readers rejecting 1.4, Cell capture prefixes, full closure-call State
propagation, reserved bytes, unknown tags, truncated instructions, forward and
nested type references, forbidden escape, State order/duplicates/missing/
excess, instruction typing, source maps, and exact/one-under VM heap limits.

No diagnostic code, schema field, Semantic ID, source syntax, CLI/LSP/package
contract, Program ID, or Unicode behavior changes. The published current
Experimental bytecode revision remains 1.2 until the complete DEC-0262
vertical slice and repository gates pass.

## Intentionally deferred

Binding-storage refactoring, detection and one-time boxing of mutable bindings
crossing Handler boundaries, complete Handler Cell/State lowering, observable
alias/mutation and continuation restoration fixtures, committed-mutation Fault
and cancellation cases, the full differential/resource matrix, Clock/Random
producers, Task/Actor transfer, Native/Wasm, migration, and Stable claims remain
deferred.
