# VM-1207 Implementation Report

## Outcome

VM-1207 is implemented for the v0.0.1 Seed mutable-place boundary. The v1.2
lowerer now consumes checked assignment nodes and represents mutable roots as
SSA environment values. Field writes rebuild independent record values with
the existing `GetField` and `UpdateRecord` instructions; `if`, checked
`match`, and short-circuit boolean joins carry mutable roots through typed CFG
block parameters. No new bytecode opcode, wire field, protocol revision, or
runtime cell was introduced.

## Authority and normative coverage

- Accepted `DEC-0009` authorizes only current-function mutable local roots and
  mutable record fields rooted at those locals; parameters, imports, globals,
  immutable fields, temporaries, and mutable captures remain rejected.
- Accepted `DEC-0010` requires `State<T>` in the checked semantic/effect
  layers. The local State effect is represented by SSA updates in this v1.2
  bytecode slice and therefore is not emitted as a host capability tag.
- Accepted `RFC-0014` and `RFC-0016` authorize the v1.1/v1.2 model, immutable
  aggregate values, `UpdateRecord`, and CFG block arguments.
- Accepted `RFC-0017` freezes the lowering boundary, evaluation order,
  independent record-copy behavior, control-flow joins, rejection boundary,
  source-span preservation, and deterministic encoding requirements.

No unresolved semantic question was resolved through implementation behavior.
Borrow inference, `&mut`, Resource values, mutable captures, parameters,
actors, native ownership, aggregate equality/serialization, lists, and
recursive wire types remain outside this slice.

## Implementation

- `crates/ling-bytecode/src/lower/v1_1.rs`
  - accepts mutable locals only in aggregate/v1.2 mode;
  - lowers root assignment by replacing the current SSA register and returning
    `Unit`;
  - lowers nested field assignment by reading the path and rebuilding records
    from the innermost field outward;
  - propagates mutable bindings through sequences and ordinary operand
    evaluation in source order;
  - adds typed mutable-root parameters and arguments for `if`, `match`, and
    boolean short-circuit joins;
  - keeps v1.0/v1.1 rejection behavior and does not perform borrow inference.
- `crates/ling-bytecode/tests/lowering.rs` verifies v1.2 UpdateRecord emission,
  CFG parameters, and byte round-trip verification.
- `crates/ling-vm/tests/execution.rs` compares the verified VM with the checked
  interpreter for mutable field assignment and branch-dependent reads.
- `docs/RFC-0017.md` and the generated governance reports register the accepted
  authority and its lifecycle.

## Verification evidence

- `cargo test -p ling-bytecode --locked --offline`: passed (21 tests).
- `cargo test -p ling-vm --locked --offline`: passed (18 execution tests).
- `cargo clippy -p ling-bytecode -p ling-vm --all-targets --locked --offline`:
  passed without warnings.
- `cargo fmt --all` and `git diff --check`: passed.
- The existing verifier, source-map, deterministic-byte, malformed-input, and
  v1.1 compatibility tests remain green.

## Compatibility and determinism

- Diagnostic allocation, JSON schemas, Semantic IDs, canonical semantic bytes,
  CLI names, ABI/FFI layout, and Unicode 17.0.0 tables are unchanged.
- Existing v1.2 readers and verifiers remain authoritative; the implementation
  uses only existing instructions and block-argument validation.
- Source maps retain original UTF-8 byte spans. Binding keys, mutable-root
  parameter order, type tables, and emitted updates use deterministic ordering
  independent of Rust allocation or hash-map iteration.

## Deferred work

VM-1208 (Effect/Capability/Fault integration), VM-1209 (broader interpreter/VM
differential coverage), and VM-1210 (fuzz/resource-limit expansion) remain
separate execution-plan targets. Mutable closure captures, function-parameter
mutation, and borrow/resource semantics require a separate accepted authority.
