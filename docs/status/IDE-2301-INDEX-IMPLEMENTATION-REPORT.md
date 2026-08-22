# IDE-2301-INDEX Implementation Report: Resolved Definition Index

## Status

`Done` for the bounded internal child authorized by Accepted DEC-0073. The
parent `IDE-2301` task remains `BlockedSpec` for the public LSP document-symbol
contract.

## Normative clauses covered

- DEC-0073 §1–§2: `ResolvedDefinitionIndex` owns user definition records and
  copies existing resolver identity, source spelling, mutability, logical name,
  and original UTF-8 spans.
- DEC-0073 §3–§4: resolver member tables determine the bounded classification;
  output order is canonical and independent of host paths, allocation order,
  or map iteration.
- DEC-0073 §5: the accessor is an in-process compiler query and does not expose
  LSP ranges, hierarchy, URI/version, publication, or stale-result state.

## Implementation

- `crates/ling-db/src/definition_index.rs` defines the immutable index and its
  read-only lookup methods.
- `CompilerDb::resolved_definition_index` builds the index only after the
  existing module graph and resolver queries succeed.
- Trait and implementation member classification reuses the resolver's
  existing `trait_members` and `impl_members` tables; no identity is recomputed.
- Invalid source input returns the existing `QueryError` and no index value.

## Verification

Focused evidence:

```text
cargo fmt --all -- --check
cargo test -p ling-db --lib --locked --offline
cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings
```

The tests cover exact Unicode/BOM/CRLF byte spans, source order, ordinary
values, types, variant constructors, repeated equality, source lookup, and
failure without partial publication.

## Compatibility and determinism

The index retains original source bytes and session-local `SourceId` values
only as existing span data. It does not become a Semantic ID, cache key, LSP
position, URI, document version, or wire representation. Sorting uses explicit
logical/source/span/classification/name/identity fields.

## Deferred work

The public IDE-2301 symbol handler, LSP `DocumentSymbol` schema, hierarchy and
selection ranges, URI/version/snapshot policy, package/generated-file policy,
position conversion, cancellation, documentation rendering, rename/reference
identity, and protocol fixtures remain deferred to the blocked parent.

