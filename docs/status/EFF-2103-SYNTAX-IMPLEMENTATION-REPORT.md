# EFF-2103-SYNTAX Implementation Report

## Result

Implemented the DEC-0064 parser/CST-only handler projection in `ling-syntax`.
The child deliberately stops at the checked-only boundary: `ling-ast` rejects
the new CST kind until an Accepted AST/HIR and checking decision exists.

## Changes

- Added deterministic `HandleExpression` and `HandlerClause` CST node kinds.
- Added contextual handler-shape parsing with operation qualified names,
  pattern parameters, optional `resume` marker, indented clause bodies, and
  nested handler expressions.
- Added cursor/diagnostic rollback for failed contextual shape probes so
  ordinary Seed identifiers named `handle`, `operation`, or `resume` remain
  valid outside the recognized shape.
- Added parser positive/negative coverage and an AST rejection test.

## Evidence

- `cargo test -p ling-syntax --all-targets --offline` — 26 unit/integration
  tests passed.
- `cargo test -p ling-ast --all-targets --offline` — 8 unit/integration tests
  passed.

## Compatibility and deferred work

No Seed diagnostic, Semantic ID, schema, CLI, LSP, bytecode, VM, protocol,
runtime, or Unicode behavior changed. Full AST/HIR lowering and checked Effect
semantics remain EFF-2103 parent scope and are intentionally not claimed here.

