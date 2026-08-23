# EFF-2104 DEC-0262 Implementation Plan

## Status and entry gate

This is a non-normative engineering plan for Draft DEC-0262. None of the
behavior below may be implemented until the decision advances through
Proposed to Accepted and the lifecycle/authority reports are regenerated.
EFF-2104 remains In Progress and `GAP-EFFECT-HANDLER-BYTECODE-001` remains Open.

The implementation starts only from a clean commit that passes governance and
status verification. Each phase must keep bytecode 1.0–1.3 bytes and behavior
unchanged and must not publish partial 1.4 execution authority.

## Phase 1: close the checked-source pattern boundary

1. Add one total `is_handler_input_irrefutable` predicate over checked HIR.
   For the current registered scalar operation inputs, only `Binding` and
   `Wildcard` are accepted. Do not reuse general match exhaustiveness or infer
   validity from lowering behavior.
2. Run the predicate while constructing checked Handler evidence, before
   `HandlerCore` or `ProgramSnapshot` publication. Report existing
   `L-EFFECT-0005` with reason `refutable_parameter`, canonical operation,
   original pattern span, and bilingual messages.
3. Remove the interpreter's reachable "checked operation input did not match"
   branch only after checked-publication tests prove it unreachable; retain an
   internal invariant assertion for corrupted in-memory state.
4. Add positive binding/wildcard fixtures and negative literal, tuple, record,
   and constructor fixtures. Assert no checked partial state, stable facts,
   BOM/CRLF/Unicode byte spans, deterministic error order, and unchanged
   handler-free diagnostics.

Primary files:

- `crates/ling-effects/src/lib.rs`
- `crates/ling-effects/src/handler_core.rs`
- `crates/ling-effects/tests/checked_handlers.rs`
- `crates/ling-eval/src/machine.rs`
- `docs/ERROR-CODES.md` only if the existing schema cannot express the reason;
  no new code is otherwise permitted

## Phase 2: define the isolated bytecode 1.4 data model

1. Add `FORMAT_VERSION_1_4` and `BYTECODE_PROTOCOL_1_4`; keep every older
   writer pinned to its exact version and make older readers reject 1.4.
2. Add `ValueType::Cell(TypeIndex)` with tag `0x14`, `Effect::State(TypeIndex)`
   with tag `2`, and instructions `CellNew=0x1d`, `CellGet=0x1e`, and
   `CellSet=0x1f`. Use typed index wrappers; never serialize a host address or
   VM Cell identity.
3. Make Effect encoding version-aware. In 1.0–1.3, retain the exact one-byte
   Console record. In 1.4, encode Console as tag `1` without payload and State
   as tag `2` plus one earlier source-value `TypeIndex`. Sort State entries by
   their canonical type index after tag order and reject duplicates.
4. Add `LoweredProgramV1_4`, dedicated lower/encode/decode/disassemble APIs,
   and 1.4 canonical verified re-encoding. Do not change `lower_v1_3` or use a
   feature flag to alter an older artifact.

Primary files:

- `crates/ling-bytecode/src/{format,model,encode,decode,disassemble,lib}.rs`
- `crates/ling-bytecode/src/lower/v1_4.rs`
- `crates/ling-bytecode/tests/{model,decode_verify,lowering}.rs`

## Phase 3: independently verify Cell and State invariants

The verifier must establish all conditions before constructing
`VerifiedProgramV1`:

- `Cell<T>` refers backward to a valid non-Cell source-value type and appears
  only in closure capture-prefix parameters and Cell instruction registers;
- no Cell appears in source parameters, results, entry signatures, constants,
  aggregate fields/cases, comparisons, intrinsics, Console operations, or
  source-facing projections;
- `CellNew` maps `T -> Cell<T>`, `CellGet` maps `Cell<T> -> T`, and `CellSet`
  maps `(Cell<T>, T) -> Unit`, with exact SSA dominance and source-map coverage;
- every function containing a reachable Cell instruction declares the exact
  `State<T>` row; direct, closure, Handler body, and clause call propagation is
  a deterministic fixed point;
- Handler residual effects subtract only handled labels, never State; unmasked
  Console Capability reachability remains separate;
- malformed type/effect tags, forward indexes, order, duplicates, missing or
  excess State rows, forbidden Cell escape, instruction arity/type, and hard
  limits fail atomically with existing `L-BYTECODE-*` reasons where applicable.

The verifier should compute and retain entry Capability facts as it does for
1.3; it must not expose VM allocation details in the verified model.

## Phase 4: refactor lowering around explicit binding storage

Perform this as a mechanical internal refactor before emitting any Cell
instruction:

1. Replace lowering environments' raw `BindingKey -> RegisterIndex` values
   with `BindingStorage::Direct(RegisterIndex)` or
   `BindingStorage::Cell { handle, value_type }`.
2. Centralize reads in `read_binding`: Direct returns its register; Cell emits
   `CellGet`. Centralize writes in `write_binding`: Direct preserves existing
   SSA environment replacement; Cell emits `CellSet` and retains the handle.
   All reference, assignment, capture, branch, match, Boolean short-circuit,
   sequence, and mutable-propagation paths must use these helpers.
3. Precompute the exact mutable `BindingKey` set crossing any Handler body or
   clause. At its lexical declaration, emit one `CellNew` and store Cell
   storage. Never box an immutable or non-crossing mutable binding.
4. A branch join never merges Cell contents or creates a new identity: both
   arms retain the same handle. Existing block parameters continue to merge
   Direct SSA values. Reject inconsistent Direct/Cell storage as an internal
   checked-lowering invariant.
5. Handler body and clause capture signatures use `Cell<T>` for shared mutable
   bindings while retaining ordinary capture operands. Nested closures inside
   those functions capture the same handle. Every read/write emits the exact
   `State<T>` row from checked expression/definition evidence.
6. Complete Handler source lowering selects 1.4 uniformly. The 1.3 lowerer
   retains its explicit experimental rejection and deterministic bytes.

Primary implementation concentration:

- `crates/ling-bytecode/src/lower/v1_1.rs` for the shared canonical planner and
  FunctionLowerer; isolate 1.4 switches behind an explicit lowering mode
- `crates/ling-bytecode/src/lower/v1_4.rs` for the public 1.4 boundary
- `crates/ling-effects/src/lib.rs` expression Effect rows as the only semantic
  source; never infer State from emitted instructions alone

## Phase 5: add a non-cyclic private VM Cell store

Do not represent a VM Cell as `Rc<RefCell<Value>>`: a Cell containing a closure
that captures the Cell can create an uncollectable Rust reference cycle.
Instead:

1. Add a monotonic private `CellId` and an Engine-owned bounded map from CellId
   to `{ value, heap_charge }`. `Value::Cell` contains only the private ID; it
   cannot be constructed by bytecode except through verified `CellNew`.
2. `CellNew` charges the heap before allocating the map entry. `CellGet` clones
   the stored Ling value. `CellSet` checks cancellation and limits before
   replacing the value, then marks mutation committed before publishing Unit.
3. Closure and continuation frame cloning copies CellId, so every alias and
   restoration observes the same map entry. No snapshot copies Cell contents.
4. The Engine owns and drops all Cell entries at execution end, preventing Rc
   cycles and host-lifetime leakage. Cell IDs, map order, allocation bytes, and
   Rust debug text never enter diagnostics or differential projections.
5. Impossible missing-ID/type states route to `L-INTERNAL-0001`; ordinary
   heap/step/frame/handler/continuation/cancellation failures retain existing
   source-mapped `L-RUNTIME-0001` contracts and committed flags.

Primary files:

- `crates/ling-vm/src/{value,execute,fault}.rs`
- `crates/ling-vm/tests/{execution,differential}.rs`

## Phase 6: evidence matrix

Required vertical fixtures:

| Area | Required evidence |
| --- | --- |
| Pattern gate | binding/wildcard success; literal/tuple/record/constructor rejection; no checked publication; bilingual facts/spans |
| Cell identity | body mutation visible in clause after resume; clause mutation visible to later outer code; two aliases; nested function capture |
| Deep control | zero/one resume; second operation during resume; nested inner/outer handlers; higher-order Once over-resume |
| State rows | exact `State<Int>` and aggregate State rows; no masking; direct/closure/Handler fixed-point propagation; no Capability requirement |
| Fault/control | mutation then Fault; host commit then Fault; cancellation before dispatch/restoration/CellSet; no rollback |
| Resources | exact/one-over heap, Cell count, frame, handler depth, continuation frame, decoder vectors, and step limits |
| Wire | exact tags/bytes/disassembly; 1.0–1.3 byte identity; 1.3 rejects 1.4; 1.4 reads 1.0–1.4; canonical re-encode |
| Malformed | Cell escape, type cycles/forward refs, wrong instruction types, State order/duplicate/missing/excess, reserved fields, truncation, fuzz determinism |
| Differential | result, logical Console events, resume count, committed mutations, Fault facts/spans, ProgramId, repeated/path-independent bytes |

## Phase 7: completion and integration order

Deliver the smallest complete commits in this dependency order:

1. Accepted authority/lifecycle/gap transition and negative checked-pattern
   evidence.
2. Format/model/codec/disassembly plus malformed verifier evidence.
3. Binding-storage refactor with unchanged 1.0–1.3 golden evidence.
4. Cell/State lowering and VM execution with differential/resource/cancellation
   evidence.
5. Protocol inventory, support/status/traceability reports, complete EFF-2104
   implementation report, full repository gates, completion commit, and push.

Only the fifth commit may mark EFF-2104 Done, resolve
`GAP-EFFECT-HANDLER-BYTECODE-001`, or publish 1.4 as the current Experimental
revision. EFF-2105 becomes dependency-ready only after that commit is recorded.
