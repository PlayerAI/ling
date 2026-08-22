# EFF-2104-REJECTION-GATE implementation report

## Result

The bounded unresolved-handler execution rejection child is implemented under
Accepted DEC-0088. It keeps unchecked handler data out of checked snapshots and
execution without selecting runtime semantics.

## Implementation

- `crates/ling-cli/tests/handler_boundary.rs` compiles an unresolved handler
  fixture through the shared CLI compiler.
- The fixture asserts `L-EFFECT-0004`, bilingual diagnostic text, an original
  source span, and absence of a checked compilation result.

## Verification

- `cargo fmt --all -- --check`
- `cargo test -p ling-cli --test handler_boundary --locked --offline` — 1 test passed.
- `cargo clippy -p ling-cli --test handler_boundary --locked --offline -- -D warnings`

## Boundaries

This child is not handler evaluation, continuation storage, operation dispatch,
bytecode/VM lowering, Fault/cancellation behavior, differential execution,
public syntax stabilization, or an LSP/JSON-RPC response. Public `EFF-2104`
remains `BlockedSpec`.
