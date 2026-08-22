# EFF-2103-HIR Implementation Report

## Result

Implemented the DEC-0066 unresolved handler HIR projection and resolver gate.
The compiler preserves handler data for a future checked contract but never
publishes it as a resolved or executable program.

## Changes

- Added HIR `HandlerClause` and `ExpressionKind::Handle` values with original
  spans and deterministic source order.
- Lowered AST operation names, parameter patterns, optional resume names, and
  bodies without creating operation/resume references or checked semantics.
- Added `ResolveErrorKind::UnsupportedHandler` and registered bilingual
  diagnostic `L-EFFECT-0004`.
- Added explicit downstream no-interpretation/rejection branches in type,
  Effect, semantic, evaluator, bytecode, database, and CLI helper paths.
- Added focused HIR and resolver coverage for positive projection and negative
  publication behavior.

## Evidence

- `cargo test -p ling-hir -p ling-resolve --all-targets --offline` — all
  focused tests passed (5 HIR, 15 resolver/project tests).
- `cargo check --workspace --all-targets --offline` — passed after the
  downstream exhaustiveness audit.
- Governance, status, formatting, full workspace tests, and clippy gates are
  run in the milestone verification step.

## Compatibility and deferred work

Non-handler Seed behavior remains unchanged. The new `L-EFFECT-0004` is only
emitted when an experimental unresolved handler reaches resolution. No checked
handler operation, runtime continuation, bytecode opcode, VM behavior, schema,
Semantic ID, CLI command, LSP protocol, or migration is claimed.

