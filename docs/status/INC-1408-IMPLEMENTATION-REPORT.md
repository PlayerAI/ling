# INC-1408 Implementation Report: Deterministic Parallel Scheduling

## Outcome

INC-1408 is complete for its accepted internal scope. `ling-db` now executes
independent source-parse misses in bounded worker scopes while publishing
immutable cache results and query evidence in canonical order. Worker
assignment and completion timing cannot change parsed output, diagnostics, or
trace order.

## Normative traceability

- Accepted `DEC-0019` §4 establishes canonical dependency traversal and permits
  parallel scheduling only after independently reproducible scheduling and
  clean/incremental-equivalence evidence.
- Accepted `DEC-0021` fixes the bounded worker, canonical publication, error
  ordering, and no-public-protocol boundary implemented here.
- Existing `ling-source` and `ling-syntax` contracts remain authoritative for
  immutable UTF-8 snapshots, source spans, Unicode 17.0.0 behavior, and parse
  results. The scheduler does not reinterpret source semantics.

## Implemented boundary

- `CompilerDb::parse_all` gathers source snapshots and immutable parse misses in
  canonical logical-name order, schedules them through bounded scoped workers,
  and joins all workers before publishing any parse cache entry.
- `schedule_order` varies worker assignment using a deterministic seed for
  stress evidence. Results, cache insertion, and `QueryEvent` publication are
  always replayed in canonical source order, independent of worker completion
  order or host CPU count.
- Cache hits do not spawn workers. A worker panic cannot publish a partial parse
  batch; malformed UTF-8 remains rejected at the existing source boundary.
- Dependent HIR, resolve, type/effect, semantic, persistence, and corruption
  paths retain their existing deterministic boundaries and are not claimed by
  this slice.

## Evidence

- `parallel_parse_scheduling_is_deterministic_across_task_seeds` runs the same
  multi-file source set—including a Unicode logical name and malformed syntax—
  under five scheduling seeds and compares parsed structures and query traces.
- Existing canonical-order, immutable-cache, malformed-source, and INC-1407
  clean/incremental tests remain green with the parallel path enabled.
- `cargo test -p ling-db --all-targets --locked --offline` passed with 17 tests;
  `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` also passed.

## Compatibility and deferred work

- No source syntax or semantics, diagnostics, schemas, Semantic IDs, CLI/LSP
  fields, public protocol, persistence format, or Unicode table changed.
- The open `GAP-INCREMENTAL-CACHE-001` now retains only INC-1409 for persistent
  cache and corruption-recovery authority. Workspace-wide dependent query
  parallelism remains a separately measurable optimization rather than an
  implied public guarantee.

## Validation and next target

Governance and status records include Accepted DEC-0021 and the executable
evidence. INC-1409 persistent cache is the next execution-plan target, subject
to a separate accepted cache protocol and corruption-recovery decision.
