# TASK-2201 Rejection Gate Implementation Report

**Status:** Done (bounded negative Seed boundary)
**Decision:** Accepted `DEC-0089`
**Implementation:** `crates/ling-cli/tests/task_boundary.rs`

## Delivered

- Compiled a Task-shaped top-level declaration through the shared in-memory
  CLI pipeline.
- Asserted the existing bilingual `L-SYNTAX-0010`/`UNEXPECTED_TOKEN`
  diagnostic, exact `task` source span, and JSON serialization.
- Asserted that compilation returns diagnostics instead of a checked snapshot.

## Verification

```text
cargo test -p ling-cli --test task_boundary --locked --offline
1 passed; 0 failed
```

## Compatibility and deferrals

No lexer keyword, grammar production, AST/HIR/Core node, diagnostic code,
Semantic ID, schema, CLI command, runtime, bytecode, VM, ABI, dependency, or
Unicode 17.0.0 data changed. Positive Task syntax and all lifecycle/runtime
semantics remain deferred under the `TASK-2201` parent.
