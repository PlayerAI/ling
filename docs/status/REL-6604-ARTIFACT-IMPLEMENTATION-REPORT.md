# REL-6604-ARTIFACT Implementation Report

## Result

`cargo xtask performance verify` now validates both the twelve-row performance
coverage matrix and the checked-in INC-1410 JSON artifact. The current artifact
contains eight structurally valid scenarios with three samples each and the
expected deterministic query-work observations.

The verifier does not compare nanoseconds, execute the timing harness, or
freeze a threshold. Parent `REL-6604` remains `BlockedSpec`.

## Implementation

- `tools/xtask/src/performance.rs` exposes crate-internal constants for the
  existing sample count, synthetic fixture size, and scenario names; the
  harness consumes the same names.
- `tools/xtask/src/performance_matrix.rs` strictly deserializes the historical
  artifact, denies unknown fields, validates metadata and scenario order,
  checks array cardinality and non-zero durations, and verifies trace,
  hit/miss, and completed-work invariants.
- A negative test mutates the schema and cold-query miss counts and asserts
  fail-closed errors.
- `tools/xtask/src/main.rs` reports the structurally verified scenario count.

## Acceptance evidence

- Internal schema: `ling.performance-baseline/1`.
- Eight scenarios, three samples per numeric field, 10,000 synthetic files,
  fixture setup excluded.
- Cold/warm/edit query and synthetic parse hit/miss observations match the
  recorded INC-1410 work boundary.
- Timing values remain opaque positive observations with no threshold or
  comparison rule.
- A fresh opt-in `cargo xtask performance baseline` run reproduced the same
  schema, scenario order, sample cardinality, and deterministic work counts;
  its host-specific timing samples were observed but deliberately not written
  over the historical INC-1410 artifact.
- Focused and full offline repository gates are required before completion is
  recorded.

## Compatibility and deferrals

No historical sample is rewritten and no public schema or performance promise
is created. Ling behavior, diagnostics, Semantic IDs, packages, dependencies,
CLI/editor protocols, runtime, and Unicode 17.0.0 remain unchanged. New
measurements, statistics, thresholds, host tiers, memory/IO, missing runtime or
editor surfaces, and G6 release evidence remain deferred.
