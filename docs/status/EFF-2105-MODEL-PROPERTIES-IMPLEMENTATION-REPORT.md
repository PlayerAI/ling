# EFF-2105-MODEL-PROPERTIES Implementation Report

**Status:** Done (bounded deterministic model-property child)  
**Decision:** Accepted `DEC-0067`  
**Implementation:** `crates/ling-effects/tests/model_properties.rs`

## Delivered

- Enumerated all permutations of a four-label, duplicate-bearing row corpus
  and proved canonical names, bytes, and open-tail identity are invariant.
- Enumerated insertion orders for bounded row constraints and proved solver
  substitutions serialize identically.
- Enumerated handler-clause orders and proved checked `HandlerCore` bytes,
  residual construction, and graph projection bytes are deterministic.
- Used distinct source spans in equivalent Core values to verify source evidence
  is excluded from canonical identity.

## Verification

The focused integration target passed offline:

```text
cargo test -p ling-effects --test model_properties --offline
3 passed; 0 failed
```

The test corpus is finite, named by the test functions, and bounded at four
labels, three row variables, and two handler clauses. It does not invoke the
evaluator, bytecode lowerer, VM, public protocol writers, or unresolved handler
HIR paths.

## Compatibility and deferrals

No language syntax, Seed behavior, diagnostic, schema, Semantic ID, CLI, LSP,
bytecode, VM, protocol, ABI, or Unicode 17.0.0 data changed. Full EFF-2105
property generation and interpreter/VM differential evidence remain deferred.
