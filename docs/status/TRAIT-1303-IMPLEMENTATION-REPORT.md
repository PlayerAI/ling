# TRAIT-1303 Implementation Report

Status: **In Progress**

## Outcome

An internal first-slice obligation collector now consumes resolved HIR and
produces normalized RFC-0005 obligation records. It preserves owner identity,
ordered source provenance, and original UTF-8 byte spans. The collector does
not resolve a Trait name, inspect an impl candidate, or construct a dictionary.

The `ling-types::check` entry point invokes the collection boundary and rejects
Trait declarations, impl declarations, and unresolved obligations through the
existing `UnsupportedTypeSyntax` path. Consequently no unresolved obligation
can enter a successful executable Typed Core before TRAIT-1304 through
TRAIT-1306 are implemented.

## Normative traceability

- `docs/RFC-0005.md` §2.1 makes `requires { ... }` the first-slice source of
  user obligations and requires generic arguments to remain explicit.
- `docs/RFC-0005.md` §2.3 requires normalized obligation collection before impl
  selection, preserves the source-origin chain, and forbids filesystem or
  hash-map order as a semantic tie breaker.
- `docs/RFC-0005.md` §4.2 forbids unresolved obligations in executable Typed
  Core; the temporary check boundary enforces this requirement.
- `docs/decisions/0024-trait-obligation-collection-boundary.md` accepts the
  internal interface and explicitly defers coherence, solving, dictionaries,
  and runtime lowering.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` still exclude Trait execution from
  the v0.0.1 Seed support matrix; no semantic scope was expanded.

## Implemented files

- `crates/ling-types/src/constraints.rs`: deterministic collection for
  top-level definitions, impl members, and expression-local bindings;
  normalized qualified names, variables, nested nominal applications, and
  parenthesized arguments; bounded malformed-syntax errors; origin records;
  repeatability and boundary tests.
- `crates/ling-types/src/lib.rs`: invokes the internal collection boundary and
  prevents unresolved Trait programs from producing Typed Core.
- `docs/decisions/0024-trait-obligation-collection-boundary.md` and generated
  governance reports: record the accepted interface and lifecycle.

## Verification

Executed offline with the locked dependency set:

- `cargo fmt --all`
- `cargo test -p ling-types --locked --offline`
- `cargo check -p ling-types --locked --offline`

The targeted suite passes the positive nested/local collection, deterministic
repeatability, malformed first-slice syntax, and non-executable Typed Core
boundary tests.

## Compatibility and determinism

- No diagnostic allocation, public protocol schema, Semantic ID rule, CLI/LSP
  protocol, ABI, or generated Unicode table changed. Existing bilingual
  `UnsupportedTypeSyntax` rendering is reused without new codes or facts.
- Existing Seed programs without Trait items or `requires` clauses retain the
  existing type-checking path.
- Module, source-span, and HIR vector order determine collection order; final
  ordinals are normalized after a stable span sort. Hash-map and filesystem
  iteration are absent from the semantic result.
- Original `Span` values and HIR-normalized Unicode names are retained; Unicode
  version remains the repository-wide 17.0.0 contract.

## Specification gaps and conflicts

`GAP-TRAIT-COHERENCE-001` remains the governing accepted gap for the later
cross-package coherence, solving, dictionary, and runtime evidence. No conflict
with Accepted RFC-0005, `SEMANTICS.md`, or `LANGUAGE.md` was found. The lower
execution plan's former “G0/interface” blocker is resolved only for this
internal collection boundary by DEC-0024; it does not authorize later solver
behavior.

## Intentionally deferred

Cross-package impl indexing, duplicate/overlap/orphan checking, candidate
selection, recursive solving and cycle/depth diagnostics, explicit Checked Core
dictionary witnesses, interpreter/VM lowering, differential conformance, IDE
support, and performance/termination gates remain deferred to TRAIT-1304
through TRAIT-1309.
