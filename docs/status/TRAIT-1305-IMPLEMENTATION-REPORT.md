# TRAIT-1305 Implementation Report

Status: **In Progress**

## Outcome

`crates/ling-types/src/solver.rs` now contains the crate-private RFC-0005
first-slice solver boundary. It consumes the resolved program, the
TRAIT-1304 coherence index, ordered obligations, and an internal nested-
requirement map. The solver selects only an exact concrete nominal receiver,
keeps Trait/impl/member identity in immutable evidence, and never uses
candidate order to resolve ambiguity.

Zero candidates, variable receivers, unknown Traits, malformed receiver arity,
and multiple candidates are represented by bounded internal error kinds.
Nested requirements are traversed in vector order; active-key cycles are
rejected and depth 64 is a hard resource boundary.

## Normative traceability

- `docs/RFC-0005.md` §2.4 requires exactly one legal candidate and distinct
  zero-versus-multiple-candidate failure behavior.
- `docs/RFC-0005.md` §2.5 requires recursive termination and a bounded limit
  of 64 nested obligations.
- `docs/RFC-0005.md` §3.4 excludes generic/blanket impl inference and forbids
  candidate ordering from changing semantic selection.
- `docs/RFC-0005.md` §5.1–§5.3 keeps diagnostics unallocated and Trait support
  outside the v0.0.1 Seed matrix.
- `docs/decisions/0026-trait-solver-v0-boundary.md` accepts the internal
  receiver mapping, immutable selection evidence, and termination boundary.

## Implemented files

- `crates/ling-types/src/solver.rs`: deterministic concrete candidate
  selection, bounded solver errors, recursive requirement traversal, cycle and
  depth checks, and targeted tests.
- `crates/ling-types/src/coherence.rs`: shared normalized Trait-name and
  canonical receiver helpers for solver lookup.
- `crates/ling-types/src/lib.rs`: registers the crate-private module while the
  existing `check` boundary continues to reject Trait programs.
- `docs/decisions/0026-trait-solver-v0-boundary.md` and generated governance,
  gap, backlog, and status records document the accepted internal contract.

## Verification

Executed offline with the locked dependency set:

- `cargo test -p ling-types --locked --offline`
- `cargo test -p ling-types solver::tests::rejects_active_cycles_and_the_bounded_depth_limit --locked --offline`
- `cargo fmt --all -- --check`
- `git diff --check`
- governance and status gates are rerun before the completion milestone.

The targeted solver suite covers unique selection, unsatisfied concrete and
variable receivers, ambiguity, invalid receiver arity, active cycles, and the
64-level depth boundary. Full workspace verification remains a completion
gate.

## Compatibility and determinism

- No diagnostic code, schema, Semantic ID, CLI/LSP or package protocol, ABI,
  bytecode, runtime, or Unicode table changed. Solver errors have no public
  renderer.
- `ling-types::check` still rejects Trait items and unresolved obligations
  through the existing `UnsupportedTypeSyntax` path; no unresolved obligation
  reaches executable Typed Core.
- Candidate and error ordering use normalized HIR names, ordered vectors,
  stable IDs, source spans, and explicit sorting. Host paths, allocation
  addresses, filesystem enumeration, and hash-map order are absent.

## Specification gaps and conflicts

`GAP-TRAIT-COHERENCE-001` remains open for dictionary witnesses, Checked Core
projection, cross-package conformance, and runtime lowering. No conflict with
Accepted RFC-0005, `SEMANTICS.md`, or `LANGUAGE.md` was found. DEC-0026 closes
only the internal solver interface.

## Intentionally deferred

Type-variable substitution from inference, generic or blanket impls,
specialization, public bilingual Trait diagnostics, dictionary lowering,
Semantic Graph fields, interpreter/VM integration, and differential fixtures
remain deferred to TRAIT-1306 through TRAIT-1309.
