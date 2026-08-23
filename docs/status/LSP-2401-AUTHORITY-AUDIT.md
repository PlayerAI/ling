# LSP-2401 Authority Audit: Semantic-Token Taxonomy

## Outcome

`LSP-2401` is authorized and complete under Accepted RFC-0046. The RFC defines
the Seed taxonomy, modifier semantics, checked-to-lexical evidence precedence,
conservative recovery, deterministic client-supported legend projection,
original-span/line-splitting rules, and explicit privacy and future-category
exclusions as `ling.semantic-token-taxonomy/0.1`.

This task is the taxonomy RFC/decision milestone. It intentionally does not
advertise `semanticTokensProvider` or implement a request: typed generation and
full/delta transport remain LSP-2402 and LSP-2403. The protocol inventory now
records that planned public surface accurately as `Future` and unimplemented.

## Normative traceability

- RFC-0046 is the sole Accepted taxonomy authority. It selects 18 standard LSP
  token types, six standard modifiers, optional custom `mutable`, exact role
  mapping, modifier exclusivity, evidence precedence, fallback, projection,
  span, freshness, privacy, and migration requirements.
- RFC-0004/RFC-0029 and DEC-0029 govern capability validation and negotiated
  positions. RFC-0023/RFC-0030/DEC-0071 govern immutable document/workspace
  snapshots. DEC-0002 keeps original UTF-8 spans authoritative.
- DEC-0012 prohibits Definition/Semantic identity leakage. DEC-0073/DEC-0075
  provide resolver-owned definitions/references. DEC-0084 through DEC-0087
  provide the lexical, checked-identity, revision, and fixture observations on
  which later generation may rely.
- The lower-authority plan's `effect`, `capability`, `resource`, `actor`,
  `node`, `kernel`, `semanticId`, `borrowed`, `unsafeBoundary`, and `generated`
  proposals are not copied into the taxonomy without exact implemented source
  roles and privacy/migration authority.

## Current interface evidence

- The exact canonical type/modifier orders and role mappings now have Accepted
  authority, including module/type/variant/Trait/record/type-parameter/
  parameter/value/field/constructor/function/member and lexical roles.
- Checked identity wins over structural evidence, which wins over the limited
  lexical fallback. Unresolved identifiers, errors, layout, punctuation, EOF,
  and synthetic zero-width tokens emit nothing.
- Client support is projected through a fixed fallback table; unsupported
  modifiers are omitted without changing analysis, and an empty selected type
  legend cannot advertise a provider.
- Multiline spans split into nonempty line-local segments; original spans,
  UTF-8/16/32 projection, non-overlap, snapshot freshness, and open-document
  version freshness are fixed before transport implementation.
- Inferred type/Effect/Capability facts, Semantic IDs, DefinitionIds, host
  paths, and future language concepts are excluded from token data.

## Acceptance boundary

RFC-0046 and its lifecycle record are the executable-design authority required
by this task. `PROTO-LSP-SEMANTIC-TOKENS` remains `Planned public`, `Future`, and
`implemented = false`; therefore the registry, support matrix, initialize
response, and existing diagnostic transcripts make no false provider claim.
LSP-2402 must implement typed classification and fixtures against this mapping;
LSP-2403 must separately accept and implement the wire transport.

## Evidence and compatibility

Evidence is `docs/RFC-0046.md`, its Accepted authority/lifecycle records, the
Future protocol/support records, DEC-0084 through DEC-0087, and
`docs/status/LSP-2401-IMPLEMENTATION-REPORT.md`.

Only governance and taxonomy authority changes. No provider, request, token
generator, diagnostic, schema, Semantic ID, public data, compiler language
semantics, interpreter, VM, bytecode, ABI, runtime, package, filesystem/network,
or Unicode 17.0.0 behavior changes.

## Intentionally deferred

Typed token generation, reference classification implementation, provider
advertisement, full/delta/result IDs, request/document scope, temporary and
dependency policy, caching, limits, cancellation, stale error codes, fixture
schema, Zed presentation, Stable lifecycle, and Semantic Transactions remain
LSP-2402 through LSP-2404 and LSP-2501/LSP-2502 work.
