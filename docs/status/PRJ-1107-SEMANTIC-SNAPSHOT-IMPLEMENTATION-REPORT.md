# PRJ-1107-SEMANTIC-SNAPSHOT implementation report

## Result

The bounded internal project semantic-snapshot child is implemented under
Accepted DEC-0083. `ling-db` now consumes a validated `LockedProject`, rebuilds
the existing package-aware checked pipeline from retained source bytes, and
returns the existing `ling.semantic/0.2` `ProjectProgramSnapshot`.

## Implementation

- `crates/ling-db/src/project_snapshot.rs` performs deterministic package/source
  traversal, path-free source naming, parse/AST/HIR lowering, package-aware
  resolution, type/effect checking, and semantic snapshot construction.
- `CompilerDb::project_semantic_snapshot` caches successful and failed results
  by `PackageGraphId`; immutable graph identity prevents host-path or map-order
  leakage into the query key.
- `ProjectSnapshotError` preserves source coordinates and existing compiler
  errors without allocating new diagnostic codes or public wire fields.
- `Cargo.toml` adds only the existing `ling-project` library dependency needed
  for the `LockedProject` input type.

## Verification

- `cargo fmt --all -- --check`
- `cargo check -p ling-db --offline` (used once to update the lockfile after the
  direct dependency declaration)
- `cargo test -p ling-db --all-targets --locked --offline` — 43 tests passed.
- The project fixture test confirms repeated `Arc` reuse, package graph ID,
  `ling.semantic/0.2`, package count, and absence of host/fixture path text.

## Boundaries

This is not a project compiler host, project selector, CLI command, execution
or test runner, build/artifact API, workspace reload service, or LSP/DAP/JSON
protocol. The parent `PRJ-1107` remains `BlockedSpec` for those contracts.
