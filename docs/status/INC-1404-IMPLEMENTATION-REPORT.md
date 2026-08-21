# INC-1404 Implementation Report: Resolve and Module Queries

## Outcome

INC-1404 is complete. The internal `ling-db` query boundary now lowers valid
AST results to HIR, builds a canonical module graph, and resolves one module
against the current module set. Query values are immutable `Arc`-backed results
and cache keys contain only deterministic compiler, source, workspace, module
topology, and imported-surface inputs.

## Normative traceability

- Accepted `DEC-0019` §§1–3 authorize immutable compiler query families,
  exact retained source bytes, canonical logical names, revision-aware inputs,
  and deterministic cache keys.
- Accepted `DEC-0019` §4 requires the initial scheduler and traversal to be
  canonical; module headers, nodes, edges, and imported surfaces are sorted
  and deduplicated before publication.
- Accepted `DEC-0019` §§6–8 keep persistence, cache migration, corruption
  protocols, parallel scheduling, and third-party query engines out of this
  internal slice.
- Existing `ling-hir` and `ling-resolve` contracts remain authoritative for
  HIR lowering, Unicode 17.0.0 name normalization, source spans, import
  binding, and registered resolver diagnostics. No new language rule is
  introduced here.

## Implemented boundary

- `CompilerDb::hir` consumes only the valid `ast` query and caches the
  repository-owned HIR result by the existing source/workspace query key.
- `CompilerDb::module_graph` snapshots visible VFS files in canonical order,
  derives normalized module names, import names, and top-level definition/type
  exports, and publishes deterministic nodes and directed import edges.
- `CompilerDb::resolve_module` invokes the existing resolver over canonical HIR
  inputs and returns the requested `ResolvedModule`. Resolver failures are
  retained as structured error lists rather than converted to host or debug
  text.
- A module's own source revision is part of its resolve key. Imported module
  public surfaces are also part of that key, so a private imported-body edit
  reuses the dependent result while an export change invalidates it. Missing
  imports continue through the existing resolver diagnostics.
- The query layer remains in-memory and repository-internal. It does not read
  ambient host paths, serialize cache values, add a public CLI/LSP field, or
  expose Rust allocation/order details as Ling behavior.

## Evidence

- `module_graph_is_canonical_and_retains_exports_and_edges` verifies canonical
  node ordering, normalized import/export surfaces, edge ordering, and graph
  hit reuse.
- `resolve_queries_reuse_private_bodies_and_invalidate_dependents_on_exports`
  verifies private-body reuse and dependent invalidation after a public export
  change.
- `cargo test -p ling-db --locked --offline`, `cargo clippy -p ling-db
  --all-targets --locked --offline -- -D warnings`, `cargo test --workspace
  --locked --offline`, `cargo fmt --all -- --check`, and `git diff --check`
  passed.

## Compatibility and deferred work

- No syntax or semantics, diagnostic allocation, schema, Semantic ID, source
  span, CLI/LSP field, public protocol, or Unicode table changed.
- Type/effect queries (INC-1405), semantic queries (INC-1406), clean versus
  incremental equivalence across the full query pipeline (INC-1407), persistent
  cache and migration work, parallel scheduling, compiler-facing cancellation,
  and LSP adapters remain deferred according to the execution plan.
- No third-party dependency was added; `ling-db` depends only on existing
  repository crates for HIR and resolution.

## Validation and next target

The implementation commit and machine-readable completion evidence are recorded
in `docs/status/implementation-status.toml`. INC-1405, the type/effect query
boundary, is the next executable target after this milestone.
