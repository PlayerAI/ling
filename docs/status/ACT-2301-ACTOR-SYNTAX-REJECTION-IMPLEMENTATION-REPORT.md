# ACT-2301 Actor Syntax Rejection Implementation Report

**Status:** Done (bounded negative Seed boundary)
**Decision:** Accepted `DEC-0090`
**Implementation:** `crates/ling-cli/tests/actor_boundary.rs`

## Delivered

- Compiled an Actor-shaped top-level declaration through the shared in-memory
  CLI pipeline.
- Asserted the existing bilingual `L-SYNTAX-0010`/`UNEXPECTED_TOKEN`
  diagnostic, exact `actor` source span, and JSON serialization.
- Asserted that compilation returns diagnostics instead of a checked snapshot.

## Verification

```text
cargo test -p ling-cli --test actor_boundary --locked --offline
1 passed; 0 failed
```

## Compatibility and deferrals

No lexer keyword, grammar production, AST/HIR/Core node, diagnostic code,
Semantic ID, schema, CLI command, runtime, bytecode, VM, ABI, dependency, or
Unicode 17.0.0 data changed. Positive Actor syntax and all identity,
state-isolation, mailbox, supervision, and runtime semantics remain deferred
under the `ACT-2301` parent.
