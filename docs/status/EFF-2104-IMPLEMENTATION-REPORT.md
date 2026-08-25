# EFF-2104 Implementation Report: Interpreter and VM Handler Execution

## Outcome

EFF-2104 is complete under Accepted RFC-0006, DEC-0260, and DEC-0262. Commit
`1188b2472ff0a61ac3d96c4ae21bbe9b6bd7eaba` completes the checked-source to
verified-VM vertical slice for the currently executable `Console.write`
Handler producer and publishes `ling.bytecode/1.4` as the current Experimental
bytecode revision.

The implementation retains the checked interpreter as the semantic oracle,
keeps bytecode 1.3 as the historical immutable/irrefutable Handler slice, and
uses bytecode 1.4 Cells for mutable lexical bindings so accepted `State<T>`
rows remain present in verified declarations. EFF-2105 later extended the
initial Handler-crossing selection to all mutable lexical bindings after its
row oracle exposed the omission. No unchecked AST or unverified bytecode
reaches an execution path.

## Normative clauses covered

- RFC-0006 and DEC-0260: checked first-order Handler Core, exact operation
  resolution, total binding/wildcard inputs, residual Effect rows, and lexical
  resume bindings.
- DEC-0262 clauses 2–8: backward-reading bytecode 1.4, internal `Cell<T>`,
  typed Cell instructions, canonical unmasked `State<T>` rows, one lexical
  Cell identity, verifier-gated propagation, and unchanged 1.0–1.3 behavior.
- Retained DEC-0261 clauses 3–8 and 10–12: deep lexical dispatch, zero/one
  resume, nearest/nested propagation, continuation restoration, committed
  State/Fault behavior, Capability preflight, cancellation, resource limits,
  and interpreter/VM differential observation.
- DEC-0013 and RFC-0020: stable source-mapped Runtime Faults, committed flags,
  explicit host cancellation, and bounded VM execution.

## Implementation

- Lowering environments now distinguish direct SSA registers from private
  Cell handles. All binding reads, writes, captures, branch joins, match joins,
  and mutable propagation use that storage abstraction.
- The initial EFF-2104 Cell selection followed direct and transitive captures
  reachable from Handler bodies and clauses. EFF-2105 commit `3517ffcc`
  generalized that existing 1.4 representation to every mutable lexical
  binding so checked and verified State rows agree. The lexical owner emits
  exactly one `CellNew`; reads and writes use the same handle.
- Handler body/clause and enclosing function signatures retain exact
  `State<T>` rows for scalar, aggregate, and function-valued Cell payloads.
  State never creates a host Capability and is never removed by Handler
  masking.
- Function-valued Cell payloads are resolved through existing checked callable
  signatures so their Effect provenance is not reconstructed from Rust types
  or source spelling.
- The verifier's canonical type dependency order includes `State<T>` payload
  indexes in function types. This keeps aggregate State records backward
  referenced and makes the writer and independent verifier use the same
  deterministic topological order.
- The existing VM Cell store retains private monotonic identities, exact
  24-byte logical heap charging, commit-before-Unit `CellSet`, and shared
  identity under closure/frame/continuation cloning without `Rc` cycles.

## Executable evidence

- Lowering tests cover read-only and assignment-only captures, one lexical
  Cell allocation, no redundant read for a root assignment, scalar/record/
  function payloads, exact State rows, no State Capability, independent
  verification, canonical re-encoding, path independence, and unchanged 1.3
  rejection.
- VM tests compare interpreter and VM output for mutation before, after, and
  without resume; nested-function aliases; and a second deep operation during
  resume. They also cover mutation followed by Fault, cancellation before a
  pending CellSet, cancellation before continuation restoration, committed
  flags, source spans, heap limits, Handler/continuation limits, and Once
  cardinality.
- The bounded differential table contains 16 checked fixtures across bytecode
  1.0–1.4 and compares Unit completion, ordered Console events, Runtime Fault
  projections, source spans, and deterministic Program IDs.
- Existing 1.4 model/verifier evidence covers exact tags, golden bytes and
  disassembly, old-reader rejection, canonical re-encoding, forbidden Cell
  escape, wrong types, State order/duplicates/missing/excess, bounded malformed
  input, and deterministic arbitrary bytes.

Executed on 2026-08-24 before the implementation commit:

```text
cargo test --workspace --all-features --locked --offline
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo fmt --all
git diff --check
```

The following repository gates also passed after this report and the status
inventories were updated: governance, support, status, CI, v0.0.1
traceability, documentation, examples, tutorials, fuzz inventory, Fault
matrix, security matrix, RC0–RC3 inventories, and the v1 artifact inventory.

## Compatibility impact

- Diagnostics: no code, bilingual template, facts schema, or compatibility
  lock changed.
- Bytecode: 1.4 becomes the current Experimental library revision; readers
  accept exact 1.0–1.4 artifacts, older readers reject 1.4, and 1.0–1.3 writer
  behavior remains unchanged.
- Semantic data: no Semantic Graph, Audit, Program ID, source spelling, or
  public CLI artifact contract changed.
- Runtime: Cells remain private, non-printable, non-comparable, heap-accounted
  implementation values. No address, allocation order, Rust ownership, or
  debug representation becomes Ling semantics.
- Determinism and Unicode: canonical source order and typed indexes determine
  emitted bytes; original UTF-8 spans and Unicode 17.0.0 remain unchanged.

## Specification gaps and intentionally deferred work

DEC-0262 resolves `GAP-EFFECT-HANDLER-BYTECODE-001`; no unresolved semantic
conflict was encountered during implementation. The following remain outside
EFF-2104 and require their own accepted authority or dependency-ready task:

- executable Clock/Random producers and user-defined operations;
- Many-producing source operations beyond the registered current producer;
- general mutable closure capture when no Handler boundary selects a Cell;
- Task/Actor crossing, catchable Fault/cancellation, cleanup or rollback;
- continuation serialization, Native/Wasm lowering, migration tooling, CLI
  artifact/default-backend promises, and Stable compatibility.
