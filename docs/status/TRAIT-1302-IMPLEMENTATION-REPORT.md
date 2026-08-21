# TRAIT-1302 Implementation Report

Status: **Done**

## Outcome

The first accepted Trait representation slice is implemented from the lexer
through the parser, CST, AST, and unresolved HIR. The implementation preserves
the original token spans and keeps Trait declarations, `impl` declarations,
generic parameters, and `requires { ... }` constraints as explicit data. It
does not select implementations, solve obligations, or lower to Checked Core;
those responsibilities remain TRAIT-1303 through TRAIT-1307.

## Normative traceability

- `docs/RFC-0005.md` §1 establishes the accepted first-slice syntax and the
  representation boundary: nominal `trait` declarations, restricted `impl`
  declarations, generic parameter lists, and explicit constraint blocks.
- `docs/RFC-0005.md` §2.1 requires constraints to remain explicit until the
  later solver/lowering stages; this slice therefore stores them without
  interpreting or resolving them.
- `docs/SEMANTICS.md` §3.7 reserves `trait` and `impl`; the v0.0.1 Seed
  boundary remains unchanged because the new nodes are not evaluated.
- `docs/LANGUAGE.md` §6.4 remains descriptive only; RFC-0005 is the accepted
  authority for the implemented grammar.

## Implemented files

- `crates/ling-syntax/src/token.rs` and `lexer.rs`: reserve and lex `trait`
  and `impl`.
- `crates/ling-syntax/src/cst.rs` and `parser.rs`: add type-parameter and
  constraint nodes plus Trait/impl declaration and member parsing, including
  comma or layout-separated constraints.
- `crates/ling-ast/src/lib.rs`: lower the new CST nodes into explicit
  `TraitDeclaration`, `ImplDeclaration`, and generic/constraint fields while
  rejecting invalid CSTs as before.
- `crates/ling-hir/src/lib.rs`: preserve the declarations and constraints in
  unresolved HIR with deterministic source order and original spans.
- `crates/ling-project/src/discovery.rs` and `crates/ling-cli/src/session.rs`:
  keep project declaration classification and REPL HIR construction exhaustive
  without enabling Trait evaluation.
- `crates/ling-ast/tests/snapshots.rs`: keep the AST/CST snapshot projection
  exhaustive for the new item kinds.

## Verification

Executed offline with the locked dependency set:

- `cargo fmt --all`
- `cargo test -p ling-syntax -p ling-ast -p ling-hir --locked --offline`

The targeted suites pass, including parser span/member coverage, AST lowering
of generic constraints, HIR preservation of Trait/impl declarations, existing
syntax differential tests, Unicode identifier tests, and snapshot stability.

## Compatibility and determinism

- No diagnostic code, public protocol schema, Semantic ID rule, or Unicode
  table changed.
- Existing v0.0.1 Seed programs retain their previous parse and lowering
  behavior; `trait` and `impl` are already reserved keywords per the accepted
  language surface.
- Member and constraint order is source order, and no Rust hash-map or debug
  representation is exposed.
- UTF-8 byte spans are retained from the original source through CST, AST, and
  HIR.

## Intentionally deferred

Constraint collection and canonicalization, orphan/overlap checking, recursive
obligation solving, coherence diagnostics, dictionary witness construction,
Checked Core lowering, runtime behavior, IDE support, and performance gates
remain deferred to TRAIT-1303 through TRAIT-1309.
