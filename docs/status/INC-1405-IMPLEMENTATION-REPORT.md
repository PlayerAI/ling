# INC-1405 Implementation Report: Type and Effect Queries

## Outcome

INC-1405 is complete. The internal `ling-db` boundary now runs the existing
`ling-types` and `ling-effects` checkers and publishes an immutable summary for
one resolved module. The query key separates the requested module's complete
body revision from imported module interface inputs, allowing safe reuse of
unchanged dependents without changing language semantics.

## Normative traceability

- Accepted `DEC-0019` §§1–3 authorize the internal `type_effect` query family,
  immutable values, exact source/project revisions, canonical logical names,
  and deterministic cache keys.
- Accepted `DEC-0019` §4 requires canonical dependency traversal; workspace
  resolution, interface collection, definition projection, and effect names
  use sorted/deduplicated inputs or the existing ordered maps.
- Accepted `DEC-0019` §§6–8 keep persistence, migration, corruption recovery,
  parallel scheduling, and third-party query engines out of this slice.
- Existing `ling-types` and `ling-effects` contracts remain authoritative for
  inference, source spans, bilingual diagnostics, effect rows, capabilities,
  and checked Typed Core inputs. No new type or effect rule was introduced.

## Implemented boundary

- `CompilerDb::type_effect` resolves the current workspace, runs the existing
  type checker followed by the effect checker, and projects the requested
  module's normalized definition types, canonical effect names, and declared
  capabilities into `TypeEffectModule`.
- Type and effect failures are retained as bounded `TypeError` or `EffectError`
  arrays in the query cache and returned through structured `QueryError`
  variants; no debug text is used as a diagnostic contract.
- `ModuleInterfaceKey` contains canonical module/import/requirement names,
  declaration parameter and annotation shapes, type declarations, logical
  source identity, and workspace revisions. Inferred public definitions and
  effect-bearing bodies additionally retain their source revision so a public
  contract change cannot leave dependents stale.
- The requested module keeps its full `QueryKey`, while imported interfaces are
  collected transitively. A private edit to an explicitly typed, effect-free
  imported body reuses the dependent module result; explicit or inferred public
  type changes invalidate dependents.
- Workspace resolution is cached separately by graph, entry, and all current
  HIR source keys. The implementation remains in-memory and repository-owned;
  it does not add a public CLI/LSP field, cache file, schema, or protocol.

## Evidence

- `type_effect_queries_project_types_and_reuse_private_imported_bodies` verifies
  module projection, function type display, private body reuse, and explicit
  public annotation invalidation.
- `inferred_public_type_changes_invalidate_importers` verifies that an inferred
  public type change invalidates its importer.
- `type_effect_queries_cache_structured_effect_failures` verifies bounded effect
  errors and deterministic error-query hit reuse.
- `cargo test -p ling-db --locked --offline`, `cargo clippy -p ling-db
  --all-targets --locked --offline -- -D warnings`, `cargo test --workspace
  --locked --offline`, `cargo fmt --all -- --check`, and `git diff --check`
  passed.

## Compatibility and deferred work

- No language syntax or semantics, diagnostic allocation, schema, Semantic ID,
  source span, CLI/LSP field, public protocol, persistence format, or Unicode
  table changed.
- Semantic queries (INC-1406), full clean versus incremental equivalence
  (INC-1407), deterministic parallel scheduling, persistent cache decisions,
  compiler-facing cancellation, and LSP adapters remain deferred.
- No third-party dependency was added; `ling-db` reuses the existing
  repository-owned `ling-types` and `ling-effects` crates.

## Validation and next target

The implementation commit and machine-readable completion evidence are recorded
in `docs/status/implementation-status.toml`. INC-1406 semantic queries is the
next executable target.
