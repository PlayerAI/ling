# IDE-2304 Authority Audit: References

## Outcome

`IDE-2304` is correctly recorded as `BlockedSpec`. The execution plan asks for a
definition/reference index, relation categories (`read`, `write`, `call`,
`type`, and `implementation`), and incrementally updated results for later
semantic tokens and rename. The repository has internal forward reference data,
but no accepted reverse-index model, relation taxonomy, source-location
projection, snapshot contract, or public references response.

No reference handler, reverse index, relation schema, incremental cache API,
protocol field, or placeholder editor surface was added.

## Normative traceability

- The execution package is non-normative; its relation list and incremental
  wording do not authorize a public protocol.
- DEC-0002 makes original UTF-8 `SourceId + Span` the position truth and
  requires an explicit SourceMap projection for future LSP UTF-16 positions.
  It does not define reference query positions, document versions, or result
  ordering.
- DEC-0012 fixes DefinitionId/BodyId identity and canonical bytes. The
  registered `PROTO-SEMANTIC-GRAPH-JSON` projection is Experimental and does
  not define a reverse reference index or editor response.
- DEC-0019 fixes internal query/VFS revision boundaries, not persistent index
  serialization, dependency invalidation, or an LSP wire contract.
- `GAP-INCREMENTAL-CACHE-001` leaves dependent-query serialization,
  invalidation, migration, and corruption recovery open; the related
  `GAP-SEMANTIC-HASH-LIFECYCLE-001` leaves identity upgrade and dependency
  propagation open.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave snapshot/version preconditions,
  field stability, and protocol migration open. RFC-0005 forbids unverified
  Trait relation or LSP claims.

## Current interface evidence

- `ling-resolve` stores deterministic forward references to definitions or
  bindings, but does not expose an editor-facing reverse index or the planned
  read/write/call/type/implementation taxonomy with stable semantics.
- `ling-semantic` emits Experimental graph references with source/target
  identity and kind metadata, but no editor ranges, URI/source mapping,
  reverse-index lifecycle, or public references request/response.
- `ling-source` provides session-local revisions and byte/source mapping, while
  `ling-project` provides discovery and lock data; neither defines dependency
  invalidation or stale-result behavior for a references index.
- No reference handler or executable positive/negative relation, cross-package,
  incremental-edit, Unicode/CRLF/BOM, stale-version, deterministic-order, or
  migration fixture exists.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. target resolution and binding/definition semantics, including whether
   unresolved, builtin, prelude, generated, primitive, and dependency targets
   are included;
2. the complete relation taxonomy, overlap/precedence rules, source and target
   identities, declaration/reference ranges, and deterministic ordering;
3. reverse-index ownership, revision keys, dependency propagation, invalidation,
   persistence/corruption policy, resource limits, and stale-snapshot behavior;
4. LSP request/response, URI and position encoding, read-only/generated
   document policy, cancellation, empty results, protocol inventory, field
   stability, Semantic ID projection, and migration; and
5. executable positive, negative, cross-package, relation-overlap,
   incremental-edit, dependency, Unicode/CRLF/BOM, stale-version,
   deterministic, and migration fixtures.

Until these contracts are Accepted, a reverse index could classify references
  inconsistently, return stale locations after a revision, or freeze
  Experimental Semantic Graph fields as editor compatibility promises.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, `0019-incremental-query-boundary`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-resolve`, `ling-semantic`, `ling-source`, and `ling-project`
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2304` can begin after relation/index, incremental invalidation, LSP
transaction, and Semantic Graph lifecycle decisions are Accepted. The future
implementation must derive results from checked references, preserve source
span and identity truth, update indexes deterministically, and label
experimental fields explicitly.
