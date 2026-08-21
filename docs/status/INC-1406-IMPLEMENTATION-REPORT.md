# INC-1406 Implementation Report: Semantic Queries

## Outcome

INC-1406 is complete. `ling-db` now exposes a cached canonical
`ProgramSnapshot` query and a module-local semantic definition/reference
fragment. The snapshot delegates identity computation and canonical JSON
writing to the existing `ling-semantic` implementation; the fragment cache
reuses equal module identity content without hiding full-workspace program-ID
changes.

## Normative traceability

- Accepted `DEC-0019` §§1–3 authorize the internal semantic/checked query
  boundary, immutable values, exact source/project revisions, and deterministic
  dependency keys.
- Accepted `DEC-0019` §4 requires canonical dependency traversal; workspace
  source keys, semantic graph ordering, and fragment fields are sorted or
  delegated to the existing canonical writer.
- Existing `ling-semantic` contracts remain authoritative for `ProgramSnapshot`,
  definition/body/node/reference identity, Semantic IDs, Unicode metadata, and
  `ling.semantic/0.1` canonical JSON. The query layer does not recalculate or
  reinterpret those identities.
- Accepted `DEC-0019` §§6–8 keep persistence, migration, corruption recovery,
  parallel scheduling, and third-party query engines out of this slice.

## Implemented boundary

- `CompilerDb::semantic_snapshot` runs the checked workspace query and invokes
  `ling_semantic::build`, caching the immutable `ProgramSnapshot` by graph,
  entry, and all current HIR source keys. The canonical writer's JSON is
  available through `ProgramSnapshot::json` and round-trips through the existing
  semantic reader.
- `CompilerDb::semantic_fragment` projects one module's canonical imports,
  requirements, definition IDs/body IDs/types/effects, node IDs, and reference
  identities. Its identity cache key excludes the full program ID, so an
  imported module body change can change the workspace program ID while
  preserving an unaffected dependent fragment.
- Presentation-only edits retain the canonical program and definition/body
  identities covered by the semantic crate; changed semantic node structure or
  import aliases produce a distinct fragment key instead of being silently
  merged.
- Type/effect failures remain structured through the existing query errors;
  semantic serialization failures are bounded to a message string and no host
  path, allocation address, or debug representation enters a key or result.

## Evidence

- `semantic_queries_publish_canonical_snapshots_and_identity_fragments` verifies
  canonical JSON round-trip, repeated snapshot hits, comment/presentation
  program/body identity behavior, and fragment identity behavior.
- `semantic_fragments_reuse_dependents_when_an_imported_body_changes` verifies
  full program-ID invalidation alongside reuse of the unaffected Main fragment.
- `cargo test -p ling-db --locked --offline`, `cargo clippy -p ling-db
  --all-targets --locked --offline -- -D warnings`, `cargo test --workspace
  --locked --offline`, `cargo fmt --all -- --check`, and `git diff --check`
  passed.

## Compatibility and deferred work

- No language syntax or semantics, diagnostic allocation, schema, Semantic ID
  algorithm, CLI/LSP field, public protocol, persistence format, or Unicode
  table changed.
- Deterministic parallel scheduling, persistent cache decisions,
  compiler-facing cancellation, and LSP adapters remain deferred. INC-1407
  clean versus incremental equivalence is now complete in its test-only scope.
- No third-party dependency was added; `ling-db` reuses the existing
  repository-owned `ling-semantic` crate and its canonical writer.

## Validation and next target

The implementation commit and machine-readable completion evidence are recorded
in `docs/status/implementation-status.toml`. INC-1408 deterministic parallel
scheduling is the next execution-plan item, subject to its accepted design
authority.
