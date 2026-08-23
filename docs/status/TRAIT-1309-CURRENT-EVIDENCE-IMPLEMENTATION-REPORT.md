# TRAIT-1309-CURRENT-EVIDENCE implementation report

## Result

The bounded internal Trait solver performance/termination evidence gate is
implemented under Accepted DEC-0252. `cargo xtask trait-performance verify`
composes the existing cycle, depth-64, and source-evidence-independence facts
while preserving five unresolved production surfaces. Parent `TRAIT-1309`
remains `BlockedSpec`.

Implementation commit: `d4c24920d0dc719a517da2e804b4824dcd792633`.

## Normative clauses covered

- Accepted RFC-0005 §2.5 and DEC-0026 fix active-cycle rejection and the exact
  64-level nested-obligation limit.
- Accepted DEC-0068 authorizes source-evidence-independent internal selection
  evidence without timing or cancellation claims.
- Accepted DEC-0252 authorizes this evidence composition and keeps production
  performance/editor behavior blocked.

## Implementation and tests

- `docs/testing/TRAIT-PERFORMANCE-STATUS.md` records three Internal and five
  `BlockedSpec` surfaces.
- `tools/xtask/src/trait_performance_status.rs` validates the exact matrix,
  seven evidence files, and two task states.
- Focused tests reject state drift, parent promotion, missing child status,
  and missing solver evidence.
- The verifier is wired into xtask usage and always-on governance CI.

## Compatibility, determinism, and Unicode

The gate is deterministic, read-only, path-independent, and offline. It adds
no language behavior, public diagnostic, schema, Semantic ID, protocol,
dependency, CLI/LSP/DAP/runtime behavior, bytecode, VM, ABI, timing promise,
network behavior, or Unicode 17.0.0 change. It runs no benchmark or solver.

## Intentionally deferred

Production obligation/query integration, deterministic work budgets,
benchmark corpora and thresholds, environment/variance policy, cancellation,
document-version/stale-result behavior, and public evidence formats remain
blocked by the parent and registered gaps.
