# IDE-2301 Authority Audit: Document symbols

## Outcome

`IDE-2301` remains correctly recorded as `BlockedSpec` for its public editor
surface. Accepted DEC-0073 closes only the bounded `IDE-2301-INDEX` child: an
in-process deterministic inventory of resolver-owned user definitions and
original spans. The decision does not fix an LSP document-symbol schema, node
kinds, hierarchy, ranges, URI/version association, or field lifecycle.

No document-symbol handler, symbol-kind mapping, hierarchy projection,
location conversion, or placeholder editor API was added.

## Normative traceability

- Accepted DEC-0012 defines path-free Semantic IDs and canonical bytes, not an
  LSP symbol request/response or presentation-range projection.
- Accepted DEC-0073 authorizes only the internal resolved-definition index;
  existing resolver IDs and spans are copied without creating presentation
  ranges or a new identity rule.
- `docs/LANGUAGE.md` and `docs/SEMANTICS.md` describe Semantic Graph nodes,
  stable identity, and source relationships, but do not freeze LSP symbol
  kinds, nesting, selection, or document lifecycle fields.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave URI/snapshot/version and public
  Semantic Graph protocol lifecycle open.
- No LSP protocol inventory entry authorizes document symbols; the Experimental
  Semantic Graph JSON protocol is not an LSP response schema.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-semantic` emits canonical graph fragments/JSON and internal IDs, while
  `ling-db` now exposes the bounded resolver-owned definition inventory for
  compiler queries; neither exposes LSP symbol kinds, hierarchical
  `DocumentSymbol` nodes, or URI locations.
- `ling-source` preserves byte spans, but no accepted position adapter maps
  them to editor ranges or document versions.
- No fixture covers symbol nesting, duplicate display names, Unicode names,
  generated/virtual files, stale snapshots, or deterministic ordering.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. symbol kinds, tags, names/detail/display policy, hierarchy/flat fallback,
   selection ranges versus full ranges, and source-span identity;
2. URI/document version and package/workspace scope, generated/dependency file
   policy, position encoding, and stale-result behavior;
3. Semantic ID and rename/reference identity mapping, localization, field
   stability, limits, cancellation, and deterministic ordering;
4. LSP lifecycle/capability negotiation, protocol inventory/version migration,
   and interaction with diagnostics/semantic snapshots; and
5. positive, negative, nested, Unicode/CRLF/BOM, cross-module/package,
   generated-file, stale-version, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, an LSP symbol projection could
leak unstable graph details, map an incorrect range, or freeze identity rules
that conflict with DEC-0012.

## Evidence and compatibility

This audit was checked against `docs/decisions/0012-semantic-identity-and-canonical-bytes.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-semantic`, `crates/ling-db`, and `crates/ling-source`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

The bounded `IDE-2301-INDEX` child is complete under DEC-0073. The public
`IDE-2301` implementation can begin only after LSP lifecycle/position and
Semantic Graph projection decisions are Accepted. It must derive locations
from approved source maps, preserve Semantic IDs, and keep compiler graph
identity separate from presentation details.
