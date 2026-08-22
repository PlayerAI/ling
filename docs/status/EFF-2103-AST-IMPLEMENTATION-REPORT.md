# EFF-2103-AST Implementation Report

## Result

Implemented the DEC-0065 unresolved handler AST projection in `ling-ast` and
the explicit HIR rejection boundary in `ling-hir`.

## Changes

- Added span-preserving `HandlerClause` and `ExpressionKind::Handle` AST data.
- Lowered operation qualified names, parameter patterns, optional contextual
  `resume`, and clause bodies without resolving or reordering them.
- Added `LowerErrorKind::UnsupportedHandler` so HIR cannot publish unchecked
  handler data.
- Added AST positive coverage and HIR negative coverage.

## Evidence

- `cargo test -p ling-ast --all-targets --offline` — 8 unit/integration tests
  passed.
- `cargo test -p ling-hir --all-targets --offline` — 5 unit tests passed.

## Compatibility and deferred work

Non-handler Seed source and existing runtime paths are unchanged. This child
adds no diagnostic, schema, Semantic ID, CLI, LSP, bytecode, VM, or protocol.
Checked HIR/Typed Core lowering and execution remain intentionally deferred.

