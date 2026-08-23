# IDE-2302 Authority Audit: Hover

## Outcome

`IDE-2302` is authorized by Accepted RFC-0037. The RFC composes the previously
accepted typed-definition, reference-span, checked-token, Trait-call,
request-snapshot, and source-position observations into the bounded public
`ling.lsp.hover/0.1` Preview protocol. It fixes the request and discovery
shapes, exact target/range rules, plaintext/Markdown rendering, checked
type/Effect/Capability/Trait projection, limits, failures, and migration
boundary without publishing compiler identities.

## Normative traceability

- `docs/SEMANTICS.md` defines types, effects, capabilities, Semantic Graph
  identity, spans, and localization principles, not an LSP hover schema or
  display grammar.
- DEC-0012 fixes Semantic IDs/canonical bytes, not hover text or source range
  selection. `PROTO-SEMANTIC-GRAPH-JSON` is an Experimental graph projection,
  not an editor hover contract.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` remain broader lifecycle gaps;
  RFC-0037 explicitly authorizes only this bounded Preview projection and
  leaves transactions and Stable graph lifecycle unresolved.
- RFC-0005 forbids Trait LSP claims without independent fixtures and allocates
  no Trait diagnostics; hover cannot invent Trait constraints or selected impls.

## Accepted implementation boundary

The repository now provides the required boundary:

- `ling-db::CheckedHoverIndex` joins exact resolver-filtered source spans to
  complete checked type, Effect, Capability, and concrete Trait-selection
  observations while keeping resolver identities internal.
- `ling-source` and the accepted LSP snapshot boundary provide exact original
  byte and negotiated UTF-8/16/32 position projection.
- `ling-lsp::hover` owns only the RFC-0037 presentation and wire boundary;
  executable fixtures prove the negotiated formats, failures, isolation,
  Unicode ranges, determinism, and bounds.

## Accepted authority constraints

RFC-0037 defines:

1. hover request target/range, snapshot/version pinning, URI/package scope,
   cancellation, limits, and stale-result behavior;
2. display schemas for checked types, effects, capabilities, explicit omission
   of documentation/source provenance, and error/unresolved states;
3. localization/markdown safety, stable versus Experimental fields, Semantic
   ID and Trait witness projection, and deterministic formatting;
4. interaction with diagnostics, position encoding, generated/dependency files,
   and protocol lifecycle/version migration; and
5. positive, negative, Unicode/CRLF/BOM, generic/effect/capability, Trait,
   stale-version, cross-package, deterministic, and migration fixtures.

The implementation is constrained to those clauses. Documentation, profile
facts, resource/borrow observations, arbitrary expressions, and public
compiler identities remain excluded because no higher-priority authority
permits them.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, `docs/decisions/0012-semantic-identity-and-canonical-bytes.md`,
`docs/RFC-0037.md`, `docs/decisions/0074-ide-typed-definition-observation.md`,
`docs/RFC-0005.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-types`, `crates/ling-effects`, `crates/ling-semantic`,
`crates/ling-source`, and `crates/ling-db/src/typed_definition_index.rs`.
The public addition is limited to Preview `ling.lsp.hover/0.1`. It changes no
diagnostic allocation, Semantic ID/schema, language semantics, runtime,
bytecode, VM, ABI, dependencies, or Unicode 17.0.0 behavior.

## Accepted bounded child

`IDE-2302-TYPED-INDEX` is `Done` under `DEC-0074`. It preserves optional
missing facts rather than inventing placeholders, and its acceptance evidence
is recorded in `docs/status/IDE-2302-TYPED-INDEX-IMPLEMENTATION-REPORT.md`.

## Intentionally deferred

Only the RFC-0037 bounded target set and checked facts are implemented.
Documentation, arbitrary expression hover, imports/modules, profile and
resource/borrow facts, unresolved recovery, dynamic registration, progress,
partial results, asynchronous cancellation, caching promises, Workspace Edits,
Semantic Transactions, and Stable lifecycle remain future work.
