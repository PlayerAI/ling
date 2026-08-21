# IDE-2303 Authority Audit: Definition Navigation

## Outcome

`IDE-2303` is correctly recorded as `BlockedSpec`. The execution plan assumes a
`ResolvedRef -> DefinitionId -> SourceOrigin` pipeline and requires navigation
to user, dependency, generated, primitive, and type definitions. The repository
has internal resolution, Semantic Graph identity, source spans, and project
discovery, but it has no accepted definition-navigation request/response,
source-origin, URI, or snapshot contract.

No definition handler, location adapter, virtual-document policy, dependency
read-only API, protocol field, or placeholder editor surface was added.

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
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version preconditions and
  Stable versus Experimental editor fields open. `GAP-SEMANTIC-PROTOCOL-
  LIFECYCLE-001` leaves graph projection stability, compatibility, and
  migration open.
- DEC-0019 covers internal VFS revisions and invalidation, not an LSP wire
  protocol or dependency/virtual-document presentation policy.
- RFC-0005 forbids Trait LSP claims without independent fixtures; navigation
  cannot invent Trait implementation or witness locations.

## Current interface evidence

- `ling-resolve` and the checked compiler pipeline retain internal references,
  definition identities, and source spans, but expose no public navigation
  request or response model.
- `ling-semantic` emits deterministic graph definitions and references with
  Semantic IDs and module/origin metadata, but no editor ranges, URI mapping,
  source provenance, declaration/type-definition distinction, or virtual
  document contract.
- `ling-source` preserves original byte spans and scalar columns; no negotiated
  editor encoding, document version, or stale-result behavior is implemented.
- `ling-project` discovers package/module paths and lock data, but no accepted
  rule permits exposing dependency locations as read-only editor targets.
- No navigation handler or positive/negative cross-package, generated,
  primitive, Unicode, CRLF/BOM, stale-version, or deterministic-order fixture
  exists.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

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

Until those contracts are Accepted, a navigation result could expose an
unstable path or Semantic Graph field, mis-map a UTF-8 span, leak dependency
implementation details, or claim unsupported Trait semantics.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, `0019-incremental-query-boundary`, RFC-0005,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-resolve`, `ling-semantic`, `ling-source`, and `ling-project`
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2303` can begin after LSP transaction/position, Semantic Graph lifecycle,
and source-origin/navigation decisions are Accepted. The implementation must
consume checked resolution, preserve byte-span and Semantic ID truth, expose
dependency/generated targets only under explicit policy, and label any
experimental fields.
