# IDE-2303 Authority Audit: Definition Navigation

## Outcome

`IDE-2303` is authorized by Accepted RFC-0038. It composes the accepted
resolver-reference child, exact source spans, checked nominal types, request
snapshot, URI, and position-projection boundaries into public Preview
`ling.lsp.navigation/0.1`. The RFC fixes definition/declaration/type-definition
semantics, tracked workspace and read-only dependency locations, source-less
null behavior, exact fields, limits, failures, and migration rules without
creating virtual documents or publishing compiler identities.

## Normative traceability

- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` define language identity and
  source semantics, not an editor navigation protocol.
- DEC-0002 makes `SourceId + Span` over original UTF-8 bytes authoritative and
  requires any future LSP UTF-16 position to be an explicit SourceMap
  projection. It does not define request positions, URIs, versions, or result
  lifetimes.
- DEC-0012 fixes typed Semantic IDs and canonical bytes. The registered
  `PROTO-SEMANTIC-GRAPH-JSON` projection is Experimental and does not define
  source-origin locations or navigation responses.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`
  remain broader lifecycle gaps. RFC-0038 authorizes only this synchronous,
  immutable, Preview navigation projection and leaves transactions and Stable
  graph lifecycle unresolved.
- DEC-0019 covers internal VFS revisions and invalidation, not an LSP wire
  protocol or dependency/virtual-document presentation policy.
- RFC-0005 forbids Trait LSP claims without independent fixtures; navigation
  cannot invent Trait implementation or witness locations.

## Accepted implementation boundary

- `ling-resolve` and the checked compiler pipeline retain internal references,
  definition identities, source spans, and nominal types.
- `ling-db::NavigationIndex` joins exact reference spans to user/binding
  definition locations and optional direct checked nominal type locations.
- `ling-semantic` emits deterministic graph definitions and references with
  Semantic IDs and module/origin metadata, but no editor ranges, URI mapping,
  source provenance, declaration/type-definition distinction, or virtual
  document contract.
- `ling-source` and the accepted request snapshot provide original-byte,
  negotiated UTF-8/16/32, document-version, and freshness boundaries.
- The LSP document registry maps canonical dependency logical names back to
  exact client-registered read-only `ling://dependency/` URIs without exposing
  filesystem paths.
- `ling-lsp::navigation` owns only RFC-0038 validation, method selection,
  URI/range projection, fixed failures, and freshness; executable fixtures
  cover the authorized target and exclusion boundaries.

## Accepted authority constraints

RFC-0038 defines:

1. request target, position encoding, URI/package scope, snapshot/version pin,
   cancellation, limits, empty-result and stale-result behavior;
2. mapping from resolved references to DefinitionId and SourceOrigin, including
   declaration versus type-definition semantics, aliases, constructors,
   prelude/builtin/primitive targets, generated documents, and dependency
   read-only policy;
3. source and virtual-document location schemas, URI normalization, source-map
   conversion, Unicode/CRLF/BOM boundaries, and deterministic ordering;
4. interaction with Semantic Graph identity, diagnostics, project revisions,
   protocol inventory, field stability, localization, and migration; and
5. executable positive, negative, cross-package, generated/primitive,
   Unicode/CRLF/BOM, stale-version, deterministic, and migration fixtures.

The implementation is constrained to those clauses. Generated/primitive
virtual documents, aliases as separate hops, multiple targets, `LocationLink`,
implementation witnesses, and nested composite-type searches remain excluded.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, `docs/RFC-0038.md`, DEC-0002, DEC-0012,
`docs/decisions/0075-ide-resolved-reference-index.md`,
`0019-incremental-query-boundary`, RFC-0005,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-resolve`, `ling-semantic`, `ling-source`, `ling-project`, and
`ling-db/src/reference_index.rs` crates.

The public addition is limited to Preview `ling.lsp.navigation/0.1`. No compiler
language, interpreter, VM, bytecode, diagnostic, schema, Semantic ID, runtime,
package format, dependency, or Unicode 17.0.0 behavior changes.

## Accepted bounded child

`IDE-2303-REFERENCE-INDEX` is `Done` under `DEC-0075`. It preserves absent
definition metadata rather than inventing locations, and its acceptance
evidence is recorded in
`docs/status/IDE-2303-REFERENCE-INDEX-IMPLEMENTATION-REPORT.md`.

## Intentionally deferred

Only RFC-0038's unique resolver-reference locations and direct nominal checked
type locations are implemented. Generated/primitive virtual documents, alias
hops, multiple targets, `LocationLink`, declaration-source selection, nested
composite types, implementation witnesses, dynamic registration, progress,
partial results, asynchronous cancellation, caching promises, Workspace Edits,
Semantic Transactions, and Stable lifecycle remain future work.
