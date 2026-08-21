# DEC-0024: Trait obligation collection boundary

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-21  
> Decision date: 2026-08-21  
> Owner role: type-system-design  
> Related authority/gap: `RFC-0005`, `GAP-TRAIT-COHERENCE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

## Question

TRAIT-1303 needs a type-system interface for collecting the obligations
introduced by `requires` clauses. The interface must be useful to the later
coherence and solver stages without selecting an implementation, depending on
filesystem or hash-map order, or allowing unresolved obligations to enter the
executable Typed Core.

## Decision

1. TRAIT-1303 introduces an internal, deterministic obligation collector in
   `ling-types`. It consumes resolved HIR and emits normalized trait names,
   normalized type arguments, an owner, a source-order ordinal, and an origin
   chain rooted in the original UTF-8 source span.
2. Collection walks resolved modules, definitions, and expression-local
   bindings in their existing ordered vectors. It preserves constraint order
   and never uses filesystem order, hash-map iteration, or candidate
   selection as a semantic tie breaker.
3. The collector accepts only the RFC-0005 first-slice type forms needed for
   an obligation head and its arguments: qualified names, type variables,
   nested nominal applications, and parenthesized forms. Function/product
   syntax and malformed delimiters are rejected as collection errors rather
   than normalized heuristically.
4. The collector records provenance but does not resolve trait names, inspect
   impl candidates, check coherence/orphan rules, solve recursion, or create a
   dictionary witness. Those responsibilities remain with TRAIT-1304 through
   TRAIT-1306.
5. Until the solver and Checked Core witness stages are complete, the normal
   `ling-types::check` entry point rejects programs containing Trait items or
   collected obligations through the existing `UnsupportedTypeSyntax` path.
   No unresolved obligation may produce a successful executable Typed Core.
6. This is an internal implementation boundary. It adds no diagnostic code,
   diagnostic schema, Semantic ID rule, CLI/LSP protocol, ABI, or Unicode
   table, and it does not claim Trait support in the v0.0.1 support matrix.

## Conformance plan

- Collect multiple constraints in source order and verify normalized
  qualified/nested arguments, owner identity, source ordinal, and original
  byte spans.
- Collect constraints in nested local bindings and verify deterministic
  traversal across repeated collection runs.
- Reject malformed or unsupported obligation argument syntax without
  fabricating a normalized type.
- Verify that `check` cannot return Typed Core for a program with Trait items
  or unresolved obligations, while existing Seed programs remain unchanged.
- Run the collection and type-check tests in independent offline processes;
  compare the complete result, including origin data and error order.

## Compatibility impact

- Adds an internal `ling-types` implementation module only; no public protocol
  or serialized schema changes are made.
- Existing diagnostic allocation and bilingual rendering remain unchanged;
  the existing `UnsupportedTypeSyntax` diagnostic is used as the temporary
  non-executable boundary.
- Determinism continues to use ordered HIR vectors and stable normalized names;
  Unicode handling remains governed by Unicode 17.0.0 and existing HIR spans.

## Unresolved alternatives

- Solver obligation substitutions, cross-package candidate indexes, coherence
  and orphan diagnostics, recursion limits, dictionary layout, and runtime
  lowering are deferred to TRAIT-1304 through TRAIT-1307.
- A public obligation-inspection API and serialized obligation schema require
  a separate protocol decision after the solver contract is accepted.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
