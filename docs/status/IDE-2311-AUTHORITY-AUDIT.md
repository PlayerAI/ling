# IDE-2311 Authority Audit: Workspace Symbols

## Outcome

`IDE-2311` is correctly recorded as `BlockedSpec`. The execution plan asks for
an incremental workspace symbol index, package/module context, maximum-result
limits, and query cancellation. The repository has project/package graph data,
Semantic Graph definitions, and bounded internal cache/revision components, but
no accepted workspace query, symbol projection, index invalidation, position,
limit, or cancellation contract.

No workspace-symbol handler, reverse/incremental index, query API, result-limit
or cancellation protocol, protocol field, or placeholder editor surface was
added.

## Normative traceability

- The execution package is non-normative; its index and query wording does not
  authorize a public workspace-symbol protocol.
- Accepted RFC-0002 and package decisions define library project/package graph
  identity and lock inputs, not an editor workspace selection, search, or symbol
  presentation contract.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  workspace query positions, URIs, versions, or limits.
- DEC-0012 fixes Semantic IDs and canonical bytes. The registered Semantic Graph
  projection is Experimental and does not define workspace symbol kinds,
  containers, source locations, or result truncation.
- DEC-0019 and DEC-0022 authorize internal VFS/revision and disposable cache
  boundaries only; `GAP-INCREMENTAL-CACHE-001` and
  `GAP-SEMANTIC-HASH-LIFECYCLE-001` leave broader dependent-query invalidation,
  persistence, migration, and identity propagation open.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave position/version, limits,
  cancellation, field stability, and protocol migration open. RFC-0005/DEC-0027
  provide no public Trait symbol projection.

## Current interface evidence

- `ling-semantic` emits deterministic definitions/modules/packages and
  Experimental references, but no workspace symbol kind/container/location
  schema or query service.
- `ling-project` provides deterministic package discovery and lock graph data;
  it does not define editor workspace-root selection, dependency/generated
  visibility, or read-only symbol policy.
- `ling-source` VFS revisions and the completed disposable cache slice are
  internal; no incremental symbol-index ownership, dependent invalidation,
  cancellation, or resource accounting is exposed.
- No workspace-symbol fixture covers exact/prefix matching, package/module
  context, duplicate/overload ordering, dependency/generated/builtin symbols,
  Unicode/CRLF/BOM locations, stale revisions, limits, cancellation, or
  deterministic truncation.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. workspace/project scope, root and package selection, dependency/generated/
   builtin inclusion and read-only policy, URI/path normalization, and module
   context;
2. symbol taxonomy, Semantic ID/provenance, declaration/container hierarchy,
   source ranges, position encoding, exact/prefix/fuzzy matching, case and NFC
   rules, duplicate/overload behavior, and deterministic ordering;
3. index ownership and revision keys, incremental invalidation across source,
   package, and dependency changes, cache/persistence/corruption policy,
   memory/result limits, truncation metadata, cancellation, and stale-query
   behavior;
4. request/response schema, snapshot/version binding, protocol inventory,
   Stable versus Experimental fields, diagnostics, and migration; and
5. executable positive/negative fixtures for roots/packages/modules,
   cross-package/generated/builtin symbols, duplicate/overload ordering,
   Unicode/CRLF/BOM, edits/revisions, limits, cancellation, deterministic
   truncation, and migration.

Until these decisions are Accepted, a workspace index could expose the wrong
package scope, return stale or nondeterministic locations, or freeze
Experimental graph fields and implementation-defined search behavior.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, RFC-0002, DEC-0002, DEC-0012, DEC-0019, DEC-0022,
RFC-0005, `docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-semantic`, `ling-project`, and `ling-source` crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2311` can begin after workspace scope/search, incremental index,
LSP-position/version/cancellation, and Semantic Graph lifecycle decisions are
Accepted. The future implementation must use checked graph data, preserve
source-span and identity truth, invalidate deterministically, enforce limits,
and label experimental fields.
