# IDE-2307 Authority Audit: Completion v0

## Outcome

`IDE-2307` is correctly recorded as `BlockedSpec`. The execution plan names six
completion contexts (expression start, member access, type position,
pattern/variant, import/module, and keyword) and requires ordering by scope
proximity, type fit, explicit imports, and stable dictionary order, with no AI
completion. The repository has no accepted completion request/response,
context classification, candidate visibility, insertion-text, or snapshot
contract.

No completion handler, candidate ranking, insertion-text API, protocol field,
capability disclosure, or placeholder editor surface was added.

The bounded child `IDE-2307-SOURCE-INDEX` only copies resolver-backed
definitions, bindings, and resolved import aliases into an internal read-only
source inventory. It does not classify a completion context, filter or rank
candidates, or apply an edit.

## Normative traceability

- The execution package is non-normative; its context list and ordering rule do
  not authorize a public editor protocol.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  completion request positions, trigger/version semantics, or text edits.
- DEC-0012 fixes Semantic IDs and canonical bytes, but the registered Semantic
  Graph projection is Experimental and does not define completion candidates,
  display fields, or insert-text identity.
- DEC-0019 fixes internal VFS/query revision boundaries, not completion request
  snapshots, cancellation, resource limits, or cache compatibility.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave editor snapshot/version,
  position, field stability, and protocol migration open. The protocol
  inventory contains no completion contract.
- `GAP-UNICODE-ALIAS-SYNTAX-001` and
  `GAP-AUTHOR-SOURCE-LOCALIZATION-001` leave alias identity, localized source
  spelling, insertion, and migration rules open. RFC-0005/DEC-0027 provide no
  public Trait completion or witness projection.

## Current interface evidence

- `ling-resolve` exposes internal scopes, definitions, references, and module
  resolution; it has no completion context or candidate model.
- `ling-db::resolved_completion_source_index` now records resolver-backed user
  definitions, local bindings, and resolved import aliases with exact original
  UTF-8 spans and existing identities. It has no visibility, ranking,
  insertion-text, position, version, or publication policy.
- `ling-types` and `ling-effects` compute checked information internally, but
  no public candidate-to-type/effect/capability presentation policy exists.
- `ling-source` preserves byte spans and revisions, but no negotiated editor
  position, trigger, cancellation, stale-result, or insertion-text mapping is
  implemented.
- No completion handler or fixture covers shadowing, inaccessible members,
  type-fit ranking, aliases, Unicode/NFC/confusables, CRLF/BOM, imports,
  variants, deterministic ties, stale snapshots, resource limits, or the
  explicitly forbidden AI path.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. request target, document/snapshot/version binding, position encoding,
   trigger/incomplete behavior, cancellation, limits, and stale-result policy;
2. the six context grammars and candidate sources, including lexical scope,
   shadowing, visibility, members, types, patterns/variants, imports/modules,
   keywords, aliases, prelude/builtins, generated/dependency symbols, and any
   Trait boundary;
3. candidate identity and deterministic ranking: scope distance, type/effect/
   capability fit, explicit import priority, stable dictionary tie-breaks,
   duplicate suppression, and ambiguity behavior;
4. completion item schema, label/detail/documentation stability, insert text
   and text-edit ranges, Unicode/formatter interaction, localization, Semantic
   ID/source provenance, protocol inventory, and migration; and
5. executable positive and negative fixtures for every context, shadowing and
   visibility, type/effect/capability fit, imports/variants/aliases,
   Unicode/CRLF/BOM, deterministic ranking, stale versions, limits, migration,
   and no-AI behavior.

Until these contracts are Accepted, completion could expose unstable internal
types or capabilities, insert text at a non-authoritative span, leak private or
dependency symbols, or freeze an undocumented ranking order.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, DEC-0019, RFC-0005,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-resolve`, `ling-types`, `ling-effects`, and `ling-source`
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2307` can begin after completion-context/ranking, LSP snapshot/position,
Semantic Graph lifecycle, and Unicode/localization decisions are Accepted. The
future implementation must use checked data, preserve source-span and identity
truth, rank deterministically without AI, disclose capabilities explicitly, and
label experimental fields.
