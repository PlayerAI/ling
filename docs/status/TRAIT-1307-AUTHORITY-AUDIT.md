# TRAIT-1307 Authority and Implementation Report

## Outcome

TRAIT-1307 is now `In Progress`. The missing runtime boundary is authorized by
Accepted RFC-0021, which closes the implementation seam left by RFC-0005 §4
and DEC-0027. The current vertical slice is deliberately static and bounded:
the checker selects a concrete nominal implementation, attaches an immutable
witness/member mapping, and both runtime backends consume that mapping without
candidate search.

The task is not complete. IDE projection, generic/blanket implementations,
serialized dictionary formats, and the remaining negative/determinism fixture
matrix are intentionally deferred.

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
  and exposes the same immutable mapping through `CheckedProgram`.
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

## Verification performed

- `cargo test -p ling-types --locked --offline` — 36 tests passed.
- `cargo test -p ling-effects --locked --offline` — 9 tests passed.
- `cargo test -p ling-semantic --locked --offline` — 12 unit tests and 5
  project tests passed.
- `cargo test -p ling-vm --test execution --locked --offline` — 22 tests
  passed, including Trait interpreter/VM differential execution.
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

- unknown-member, ambiguous-impl, malformed-witness, and over-application
  differential fixtures beyond the current checked rejection coverage;
- cross-module package conformance and module-input-order reproducibility;
- v1.0/v1.1 aggregate limitations and any new bytecode serialization;
- generic receiver substitution, blanket impls, trait objects, associated
  types, default methods, specialization, and IDE/LSP projections.

TRAIT-1307 may be marked `Done` only after the deferred evidence is added and
the governance/status/support registries are regenerated and verified.
