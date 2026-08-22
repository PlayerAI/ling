# EFF-2103-AST Implementation Report

## Result

Implemented the DEC-0065 unresolved handler AST projection in `ling-ast`.
The follow-on DEC-0066 HIR projection now preserves this data until the
resolver rejection gate.

## Changes

- Added span-preserving `HandlerClause` and `ExpressionKind::Handle` AST data.
- Lowered operation qualified names, parameter patterns, optional contextual
  `resume`, and clause bodies without resolving or reordering them.
- The HIR boundary remains unresolved and non-executable; resolution rejects
  it with `L-EFFECT-0004` under DEC-0066.
- Added AST positive coverage and HIR negative coverage.

## Evidence

- `cargo test -p ling-ast --all-targets --offline` — 8 unit/integration tests
  passed.
- `cargo test -p ling-hir --all-targets --offline` — 5 unit tests passed.

## Compatibility and deferred work

Non-handler Seed source and existing runtime paths are unchanged. This child
adds no diagnostic, schema, Semantic ID, CLI, LSP, bytecode, VM, or protocol.
Checked HIR/Typed Core lowering and execution remain intentionally deferred.
