# IDE-2304-REVERSE-INDEX implementation report

## Outcome

The bounded internal child `IDE-2304-REVERSE-INDEX` is implemented. It groups
the existing resolver reference inventory by definition or binding target in
`ling-db::ResolvedReferenceReverseIndex`.

The public `IDE-2304` references task remains `BlockedSpec`: no relation
taxonomy, source range, incremental cache, URI/version, or public references
response was added.

## Normative clauses covered

- `DEC-0076` §§Decision 1–5 authorizes only immutable target grouping,
  existing resolver identities, deterministic ordering, and the in-process
  boundary.
- `DEC-0002` preserves original UTF-8 identity facts; no source position or
  SourceMap projection is performed.
- `DEC-0019` remains an internal query/VFS boundary; no persistent cache or
  invalidation protocol is introduced.

No Draft or lower-authority execution-plan text is used as semantic authority.

## Implementation

- `crates/ling-db/src/reference_index.rs` defines target keys, source records,
  grouped reverse entries, and the immutable reverse index.
- `CompilerDb::resolved_reference_reverse_index` builds the reverse view from
  the validated resolver workspace and does not publish it after source or
  resolution failure.
- Grouping preserves DefinitionId or binding module/local identity and source
  module/logical source/reference IDs. It does not infer relation categories or
  source locations.
- BTree grouping and explicit source ordering make repeated output stable;
  host paths, allocation addresses, map iteration order, and locale do not
  participate.

## Verification

Executed successfully:

- `cargo fmt --all`
- `cargo test -p ling-db --lib --locked --offline` (29 passed)
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`

Focused tests cover deterministic repeated grouping, forward-to-reverse target
identity, source-local reference IDs, and invalid UTF-8 failure.

## Compatibility and determinism

- No language semantics, diagnostics, schemas, Semantic IDs, CLI behavior,
  protocol inventory, runtime, bytecode, VM, ABI, or Unicode 17.0.0 data
  changed.
- Relation taxonomy, source reference spans, incremental/dependency
  invalidation, persistence/corruption, request positions, URI/version/
  snapshot binding, cancellation, stale publication, and JSON-RPC lifecycle
  remain deferred.

## Intentionally deferred

`IDE-2304` still requires Accepted relation/index and references-protocol
authority before editor-facing work. This child is only a resolver-derived
reverse observation and must not be presented as references or rename support
or Stable 1.0 IDE functionality.
