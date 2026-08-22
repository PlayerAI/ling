# IDE-2302 Authority Audit: Hover

## Outcome

`IDE-2302` is correctly recorded as `BlockedSpec`. The execution plan asks for
hover over typed-core information such as inferred types, effects, and
capabilities. Accepted `DEC-0074` closes only the bounded internal
`IDE-2302-TYPED-INDEX` observation. The repository still has no accepted hover
request/response schema, display policy, position/version binding, or Trait
projection.

No hover handler, Markdown/plaintext renderer, type/effect display policy,
capability disclosure, or placeholder editor API was added. The child exposes
only immutable checked facts and resolver spans inside `ling-db`.

## Normative traceability

- `docs/SEMANTICS.md` defines types, effects, capabilities, Semantic Graph
  identity, spans, and localization principles, not an LSP hover schema or
  display grammar.
- DEC-0012 fixes Semantic IDs/canonical bytes, not hover text or source range
  selection. `PROTO-SEMANTIC-GRAPH-JSON` is an Experimental graph projection,
  not an editor hover contract.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves position/snapshot/version and
  editor fields open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves graph
  projection lifecycle open.
- RFC-0005 forbids Trait LSP claims without independent fixtures and allocates
  no Trait diagnostics; hover cannot invent Trait constraints or selected impls.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-types`, `ling-effects`, and `ling-semantic` compute internal checked
  type/effect/capability information. `ling-db::TypedDefinitionIndex` now
  joins those exact facts to user definitions, but remains an in-process
  observation with no hover presentation model.
- `ling-source` keeps byte spans and scalar columns; no negotiated editor
  position/version association or hover range is implemented.
- No code defines display of inferred versus declared types, effect rows,
  capabilities, documentation, aliases, unresolved/error states, or Trait
  witnesses. No hover fixture exists.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. hover request target/range, snapshot/version pinning, URI/package scope,
   cancellation, limits, and stale-result behavior;
2. display schemas for declared/inferred types, effects, capabilities,
   documentation, aliases, source/provenance, and error/unresolved states;
3. localization/markdown safety, stable versus Experimental fields, Semantic
   ID and Trait witness projection, and deterministic formatting;
4. interaction with diagnostics, position encoding, generated/dependency files,
   and protocol lifecycle/version migration; and
5. positive, negative, Unicode/CRLF/BOM, generic/effect/capability, Trait,
   stale-version, cross-package, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, a hover response could leak
unstable implementation details, expose unapproved Trait semantics, or bind
presentation text to a non-authoritative source span.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, `docs/decisions/0012-semantic-identity-and-canonical-bytes.md`,
`docs/decisions/0074-ide-typed-definition-observation.md`,
`docs/RFC-0005.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-types`, `crates/ling-effects`, `crates/ling-semantic`,
`crates/ling-source`, and `crates/ling-db/src/typed_definition_index.rs`.
The child adds only internal checked-observation values; no public protocol
behavior, diagnostic allocation, schema, Semantic ID, runtime, bytecode, VM,
or Unicode 17.0.0 claim is made.

## Accepted bounded child

`IDE-2302-TYPED-INDEX` is `Done` under `DEC-0074`. It preserves optional
missing facts rather than inventing placeholders, and its acceptance evidence
is recorded in `docs/status/IDE-2302-TYPED-INDEX-IMPLEMENTATION-REPORT.md`.

## Intentionally deferred

The parent `IDE-2302` can begin after the LSP position/snapshot and Semantic
Graph/hover projection decisions are Accepted. Any future implementation must
use checked data, preserve source-span/ID truth, keep formatting deterministic,
and label any experimental fields explicitly.
