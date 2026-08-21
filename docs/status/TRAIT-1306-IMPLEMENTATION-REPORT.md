# TRAIT-1306 implementation report / TRAIT-1306 实现报告

Status: **Done**

TRAIT-1306 adds the internal Checked Core dictionary-witness boundary in
`crates/ling-types/src/checked_core.rs`. It consumes immutable solver selection
evidence and produces an immutable, canonically ordered table. This report
records the accepted scope; it does not claim public Trait execution.

## Normative basis

- `docs/RFC-0005.md` §4.1–§4.3 requires explicit immutable dictionary
  witnesses in Checked Core, checked semantic identity, and deterministic
  projection behavior.
- `docs/decisions/0026-trait-solver-v0-boundary.md` supplies the immutable
  selection records and keeps the solver crate-private.
- `docs/decisions/0027-trait-checked-core-dictionary-witness.md` accepts the
  lowering shape, validation boundary, canonical-byte exclusions, and the
  continued v0.0.1 non-executable boundary.

## Implemented slice

- `DictionaryWitness` preserves `TraitId`, `ImplId`, receiver type, ordered
  member ordinal/name definitions, obligation order, and original
  `ObligationOrigin`.
- Lowering validates selected identity against the immutable coherence index;
  it never searches for a replacement candidate.
- Duplicate ordinals, unknown implementation IDs, Trait/receiver mismatches,
  and member-order/set mismatches produce stable internal evidence with the
  original source span.
- Canonical bytes use length-prefixed UTF-8 semantic fields and omit source
  names/spans, host paths, map order, allocation addresses, and hash seeds.
- The module is crate-private and is not attached to `TypedProgram`, the
  Semantic Graph, interpreter, VM, CLI, LSP, bytecode, or public schemas.

## Evidence

- `cargo test -p ling-types --locked --offline`: 34 tests passed, including
  identity/member-order, mismatch rejection, duplicate/unknown selection, and
  source-origin canonical-byte invariance cases.
- `cargo test --workspace --locked --offline`: passed for the full workspace,
  including all compiler, interpreter, VM, conformance, Unicode, and xtask
  suites (one existing explicitly ignored fixture remained ignored).
- `cargo xtask governance check-all`: passed with 61 documents, 28 gaps, 36
  lifecycle records, 21 protocols, and 82 diagnostic codes.
- `cargo xtask status verify`: passed with 65 tasks (62 Done before this
  completion record, now 63 Done) and 7 features with stabilization blockers.
- `cargo fmt --all -- --check`: passed.
- `git diff --check`: passed; only the repository's existing CRLF conversion
  warnings were reported.

## Compatibility and deferrals

No diagnostic code, JSON schema, Semantic ID, CLI/LSP protocol, ABI, bytecode,
runtime behavior, or Unicode 17.0.0 data changed. `ling-types::check` still
rejects Trait-bearing programs through `UnsupportedTypeSyntax`. Public checked
projection and interpreter/VM dictionary passing remain TRAIT-1307 work.
