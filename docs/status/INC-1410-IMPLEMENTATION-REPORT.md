# INC-1410 Implementation Report: Incremental Performance Baseline

## Outcome

INC-1410 is complete as a measurement-only engineering slice. The repository
now provides `cargo xtask performance baseline`, an opt-in harness that records
three timing samples for cold/warm checked queries, single-character and
signature edits, a workspace-input (cross-package boundary) edit, and cold,
warm, and single-edit runs over a 10,000-file synthetic workspace.

The output is the versioned internal evidence artifact
[`INC-1410-PERFORMANCE-BASELINE.json`](INC-1410-PERFORMANCE-BASELINE.json).
Fixture construction is excluded from timed regions and reported explicitly;
the harness makes no absolute latency or hardware claim.

## Normative and implementation boundary

- `DEC-0019` and `DEC-0021` remain the authority for query identity,
  invalidation, and deterministic publication. The harness consumes those
  existing `ling-db` APIs and does not change query semantics.
- The checked scenarios use `CompilerDb::semantic_snapshot`; the synthetic
  scenario uses the existing canonical `parse_all` boundary. No unchecked AST,
  HIR, Typed Core, host path, allocation identity, or map order is reported.
- The cross-package case changes the `PackageManifest` workspace input. This
  is the current file-mode compiler's explicit workspace/package revision
  boundary; a package-graph benchmark will be added only when that public
  project query surface is implemented.

## Recorded observation

The checked-query samples in this run were approximately 1.38–3.73 ms cold,
18.6–23.3 μs warm, 1.25–1.39 ms for the single-character edit, 1.28–1.31 ms
for the signature edit, and 1.40–1.45 ms for the workspace-input edit. The
10,000-file parse samples were approximately 639–654 ms cold, 38–42 ms warm,
and 40–43 ms after one edited file. These are one local observation, not a
release threshold; future gates compare like-for-like trends from the JSON
schema.

## Evidence and validation

- `tools/xtask/src/performance.rs` defines the versioned JSON shape, bounded
  fixture sizes, deterministic logical names, and observable hit/miss counts.
- `cargo xtask performance baseline` completed successfully and generated the
  checked-in JSON artifact with three samples for all eight scenarios.
- `cargo clippy -p xtask --all-targets --locked --offline -- -D warnings`,
  `cargo fmt --all -- --check`, and the existing workspace test suite passed.

## Compatibility and deferred work

- No source syntax, Typed Core semantics, diagnostics, schemas, Semantic IDs,
  CLI/LSP fields, public protocol, cache format, or Unicode data changed.
- Timing values are evidence only; no absolute performance promise, scheduler
  policy, package-graph API, or benchmark dependency was introduced.
- Regression thresholds, repeated-host sampling, memory/IO profiles, and
  package-graph scale tests remain follow-up work under the quality/release
  gates rather than being inferred from this baseline.

## Next target

The next execution-plan row is `FMT-1501` (formatter preservation authority),
subject to its required Accepted decision and evidence boundary.
