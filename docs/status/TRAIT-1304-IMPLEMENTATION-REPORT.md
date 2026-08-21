# TRAIT-1304 Implementation Report

Status: **In Progress**

## Outcome

An internal coherence index now validates the restricted RFC-0005 declaration
and impl shape before solver work. It indexes normalized Trait IDs, nominal
receiver terms, package ownership, original spans, and ordered member names.
Duplicate members, missing/unexpected members, generic receivers, unknown
Traits/receivers, duplicate receivers, and applicable overlap are represented as
bounded internal errors.

The index records the resolved package graph identity when package-aware HIR is
available. It does not select a candidate, produce a dictionary, allocate a
user-facing diagnostic, or make Trait programs executable; `ling-types::check`
continues to reject the entire Trait boundary through the existing
`UnsupportedTypeSyntax` path.

## Normative traceability

- `docs/RFC-0005.md` §1.1–§1.3 requires finite ordered Trait members, one
  nominal receiver, complete impl members, and rejection of partial/structural
  matching.
- `docs/RFC-0005.md` §3.1–§3.5 requires package-aware ownership, orphan and
  overlap rejection, no generic impls in the first slice, and deterministic
  behavior independent of host paths or Rust map order.
- `docs/RFC-0005.md` §5.1–§5.3 allocates no current diagnostic code and keeps
  Trait syntax/semantics outside the v0.0.1 support matrix.
- `docs/decisions/0025-trait-coherence-index-boundary.md` accepts this internal
  index boundary and defers solving, dictionaries, and runtime behavior.

## Implemented files

- `crates/ling-types/src/coherence.rs`: ordered Trait/impl indexing, receiver
  canonicalization, package/lock identity capture, ownership and shape checks,
  deterministic duplicate/overlap evidence, and unit coverage.
- `crates/ling-types/src/constraints.rs`: exposes the shared first-slice type
  expression normalizer used by receiver indexing.
- `crates/ling-types/src/lib.rs`: invokes the coherence boundary while keeping
  all Trait failures on the existing non-executable diagnostic path.
- `docs/decisions/0025-trait-coherence-index-boundary.md` and generated
  governance/status records document the accepted internal contract.

## Verification

Executed offline with the locked dependency set:

- `cargo fmt --all`
- `cargo test -p ling-types --locked --offline`
- `cargo check -p ling-types --locked --offline`

The targeted suite passes valid nominal indexing, ordered member preservation,
duplicate/missing member and receiver rejection, generic receiver rejection,
and deterministic ownership/overlap matrix tests.

## Compatibility and determinism

- No diagnostic code, schema, Semantic ID, CLI/LSP or package protocol, ABI, or
  Unicode table changed. Internal errors have no public renderer.
- Existing Seed programs retain their previous type-checking path; Trait items
  remain outside v0.0.1 execution.
- Resolved module/package order, canonical receiver terms, and source spans
  define index and candidate order. The package graph identity includes the
  lock-derived content identity; filesystem paths and hash-map iteration are
  absent from the result.
- HIR-normalized Unicode names and original UTF-8 declaration spans are kept
  unchanged.

## Specification gaps and conflicts

`GAP-TRAIT-COHERENCE-001` remains open for solver, dictionary, runtime, and
cross-package conformance behavior. No conflict with Accepted RFC-0005,
`SEMANTICS.md`, or `LANGUAGE.md` was found. DEC-0025 resolves only the internal
coherence/index interface; it does not resolve the remaining semantic gap.

## Intentionally deferred

Unique candidate solving, recursion/cycle/depth limits, stable Trait diagnostic
allocation, explicit Checked Core dictionary witnesses, Semantic Graph fields,
interpreter/VM lowering, differential fixtures, IDE support, and performance
gates remain deferred to TRAIT-1305 through TRAIT-1309.
