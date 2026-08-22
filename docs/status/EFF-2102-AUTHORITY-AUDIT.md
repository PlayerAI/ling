# EFF-2102 Authority Audit: Effect Inference and Constraints

## Outcome

`EFF-2102` is now `Done`. RFC-0006 is Accepted for the experimental v0.2 Effect
model, and Accepted DEC-0062 closes the solver-specific authority gap. The
bounded in-process solver is implemented and committed under those documents,
separate from Seed syntax, Checked-Core lowering, runtime execution, and public
protocols.

The implementation report and repository-wide gates provide the acceptance
evidence; later tasks consume the solver only through its explicit API.

## Normative traceability

- The G2 execution package remains non-normative planning material; its task
  split is implemented only within the Accepted authority below.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and DEC-0010 continue to govern the
  v0.0.1 Seed closed Effect/State/Capability boundary. EFF-2102 must not alter
  that checker.
- Accepted RFC-0006 defines canonical `EffectId`, parameterized labels,
  open/closed rows, row variables, operation contracts, residual rows,
  Capability separation, and the explicit prohibition on implicit State
  masking. See RFC-0006 clauses 1–10.
- Accepted DEC-0062 defines the EFF-2102 constraint grammar, provenance and
  canonical work order, substitution/unification, deterministic fresh tails,
  occurs check, minimal conflicts, value-restricted generalization and
  instantiation, handler subtraction, State/Capability boundaries, and the
  `L-EFFECT-0001`/`L-EFFECT-0002` diagnostic allocation.
- `GAP-EFFECT-STATE-MASKING-001` and `GAP-EFFECT-HANDLER-001` are resolved by
  RFC-0006 for this bounded core model; runtime masking and handler execution
  remain outside its scope.

## Current implementation evidence

- `ling-effects` already contains the canonical RFC-0006 row, label, operation,
  handler, and projection values. The existing Seed `EffectRow` remains a
  closed `BTreeSet` fixed-point checker.
- The v0.2 module now provides the EFF-2102 constraint collection, substitution
  normalization, row unification, occurs-check boundary,
  generalization/instantiation, tracked conflict facts, and bilingual solver
  diagnostics in a separate solver module.
- No source-level row syntax, handler Typed Core node, runtime/VM handler
  execution, Task/Actor behavior, or public schema/protocol is added by this
  authority.

## Implementation acceptance boundary

The implementation must provide, with focused tests:

1. `Equal` and `Requires` constraints carrying stable provenance and original
   UTF-8 byte spans when supplied.
2. Canonical sorting/deduplication independent of insertion order, a
   deterministic substitution map, row-tail normalization, distinct-tail
   unification, and an occurs check.
3. Explicit value-restriction generalization and caller-seeded instantiation,
   with sorted quantified variables and no host allocator identity.
4. Handler subtraction that preserves open tails, never implicitly masks
   `State<T>`, and never changes Capability facts.
5. Minimal deterministic conflict sets and bilingual `L-EFFECT-0001` and
   `L-EFFECT-0002` diagnostics in human and JSON representations.
6. Positive, negative, property, randomized-order, Unicode/CRLF/BOM-span, and
   clean/incremental differential fixtures before EFF-2103 consumes results.

## Compatibility and determinism

This task may add only an Experimental v0.2 in-process API and the two
registered diagnostic entries. Seed source acceptance, existing diagnostics,
Semantic IDs, schemas, CLI, LSP, bytecode, VM, protocols, ABI, and Unicode
17.0.0 data remain unchanged. Canonical rows and substitutions must exclude
paths, host state, allocation identity, hash-map order, scheduling, and debug
formatting. Original UTF-8 byte spans remain evidence fields only.

## Intentionally deferred

Source syntax and Checked-Core lowering remain EFF-2103. Handler execution,
continuations, Task/Actor lifecycle, Replay, Remote, Native, GPU, FFI, and
Stable 1.0 compatibility require their own accepted authorities. This audit
does not claim any of those features.
