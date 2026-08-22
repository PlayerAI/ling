# TRAIT-1307 Authority and Implementation Report

## Outcome

TRAIT-1307 is now `Done` for the bounded v0.1 static Trait slice. The missing runtime boundary is authorized by
Accepted RFC-0021, which closes the implementation seam left by RFC-0005 §4
and DEC-0027. The current vertical slice is deliberately static and bounded:
the checker selects a concrete nominal implementation, attaches an immutable
witness/member mapping, and both runtime backends consume that mapping without
candidate search.

IDE projection, generic/blanket implementations, serialized dictionary formats,
and bytecode 1.0/1.1 aggregate coverage remain intentionally deferred scope;
they do not alter the completed static v1.2 execution boundary.

## Normative traceability

- RFC-0005 §1.5 and §2 authorize the restricted Trait declaration, impl,
  coherence, and concrete selection shape.
- RFC-0005 §4.1–§4.2 require immutable witness lowering and forbid unresolved
  obligations in executable Typed Core.
- DEC-0027 defines the internal dictionary witness data and canonical identity
  boundary.
- RFC-0021 §1–§8 accepts the static `Trait.member` call boundary, deterministic
  implementation-member identities, interpreter dispatch, existing direct-call
  bytecode lowering, and semantic identity inclusion.
- SEMANTICS.md §6.8 and LANGUAGE.md §6.4 remain the higher-authority v0.0.1
  support boundary; RFC-0021 does not claim a v0.0.1 Stable feature.

## Implemented evidence

- `ling-resolve` indexes deterministic Trait-member and implementation-member
  `DefinitionId` values and resolves local/imported qualified member names.
- `ling-types` selects concrete calls, rejects bare or unsatisfied members,
  attaches `DictionaryTable` and `TraitMemberCall` records to `TypedProgram`,
  exposes the same immutable mapping through `CheckedProgram`, canonicalizes
  imported nominal receiver identities, and rejects over-application.
- `ling-effects` tracks implementation-member effects and static member calls
  in capability propagation.
- `ling-semantic` includes canonical witness bytes and implementation-member
  bodies in Program identity inputs.
- `ling-eval` dispatches through the checked implementation identity and faults
  if an unchecked bare member reaches runtime.
- v1.2 bytecode lowering includes implementation members in the function table,
  lowers full and partial static member calls to existing `Call`/
  `CallClosure` instructions, and skips the non-runtime projection shape.
- `crates/ling-vm/tests/execution.rs` compares the checked interpreter with the
  independently verified v1.2 VM for a two-argument member and partial
  application.
- Cross-module Trait calls are checked with reordered module input and produce
  equal witness bytes, member ordinals, and implementation identities.

## Verification performed

- `cargo test -p ling-types --locked --offline` — 38 tests passed.
- `cargo test -p ling-effects --locked --offline` — 9 tests passed.
- `cargo test -p ling-semantic --locked --offline` — 12 unit tests and 5
  project tests passed.
- `cargo test -p ling-vm --test execution --locked --offline` — 22 tests
  passed, including Trait interpreter/VM differential execution.
- `cargo test --workspace --locked --offline --quiet` — complete workspace
  suite passed, including 92 governance tests.
- `cargo clippy --workspace --all-targets --all-features --locked --offline
  -- -D warnings` — passed.
- `cargo fmt --all -- --check`, `xtask governance check-all`, `xtask status
  verify`, and `xtask ci verify` — passed.
- `cargo fmt --all` completed successfully.

## Compatibility and determinism

- No bytecode wire revision, opcode, schema marker, diagnostic code, host
  capability, or Unicode table changed.
- Existing direct-call verifier rules remain in force; the selected
  implementation `DefinitionId` is resolved before lowering.
- Witness canonical bytes contain Trait/impl/member identity and omit source
  paths, spans, allocation details, and hash-map order.
- Original UTF-8 spans remain the diagnostic source of truth.

## Intentionally deferred

- package-level cross-package fixtures beyond the current same-workspace
  module-order coverage;
- v1.0/v1.1 aggregate limitations and any new bytecode serialization;
- generic receiver substitution, blanket impls, trait objects, associated
  types, default methods, specialization, and IDE/LSP projections.
