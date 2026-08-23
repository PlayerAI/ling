# IDE-2308 Authority Audit: Completion Resolve

## Outcome

Historical audit outcome: `IDE-2308` was correctly recorded as `BlockedSpec`
when this audit was written. Accepted RFC-0043 now supplies the bounded resolve
handle, snapshot, checked presentation, documentation-absence, exact insertion
preservation, failure, limit, and migration contract required below. The
implementation evidence is recorded separately in
`docs/status/IDE-2308-IMPLEMENTATION-REPORT.md`.

The bounded child `IDE-2308-METADATA` only joins existing checked type, Effect
Row, and module Capability facts to resolver-backed definitions and bindings.
It does not render, disclose, redact, or insert any completion item.

## Normative traceability

- The execution package is non-normative; its lazy-loading list does not
  authorize a public completion-item protocol.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  resolve handles, item lifetimes, document versions, or insertion edits.
- DEC-0012 fixes Semantic IDs and canonical bytes. The registered Semantic Graph
  projection is Experimental and does not define completion display or
  provenance fields.
- DEC-0010 fixes Seed Effect/Capability checking and requires the Evaluator to
  consume checked requirements; it does not define localized IDE presentation
  or disclosure policy.
- DEC-0023 fixes the preservation-oriented Author Source formatter boundary,
  while `GAP-FORMATTER-AUTHOR-SOURCE-001` leaves broader rewrite/presentation
  policy open. Completion insertion cannot invent a formatter contract.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave position/version, field
  stability, stale results, and protocol migration open. Alias/localization
  gaps remain relevant to Unicode insertion and display. RFC-0005/DEC-0027
  provide no public Trait completion projection.

## Current interface evidence

- `ling-types` and `ling-effects` compute checked signatures, effects, and
  capabilities internally, but expose no completion-resolve item identity,
  display model, or capability-redaction policy.
- `ling-db::resolved_completion_metadata_index` now preserves optional checked
  type displays, canonical effects, and module capabilities alongside existing
  resolver identities and original spans. It has no documentation, signature
  presentation, redaction, insertion, position, version, or publication policy.
- `ling-semantic` can project Experimental graph type/effect/capability data,
  but that graph is not a documentation/signature protocol and has no editor
  resolve handle or source provenance contract.
- `ling-source` preserves byte spans and revisions; no Unicode-safe insertion
  edit, formatter integration, negotiated position encoding, or stale-item
  behavior is implemented.
- No completion item store or fixture covers lazy documentation, generic/full
  signatures, effects/capabilities, localized/Unicode names, formatter-safe
  insertion, stale versions, deterministic output, or capability redaction.

## Authority requirements resolved by RFC-0043

An Accepted decision or RFC must define, at minimum:

1. completion-item identity/handle, snapshot and document-version binding,
   cancellation, limits, stale/expired resolve behavior, and deterministic
   missing-item errors;
2. documentation source, markdown/plaintext safety, localization, full
   signature grammar, generic/parameter/return display, Effect Row and
   Capability rendering, profile/stability labels, and redaction rules;
3. insert text and text-edit schema, selection/range semantics, Unicode
   17.0.0/XID/NFC/confusable handling, SourceMap/UTF-16 conversion, and
   formatter/comment preservation;
4. Semantic ID/source provenance fields, dependency/generated/builtin policy,
   protocol inventory, Stable versus Experimental lifecycle, and migration; and
5. executable positive/negative fixtures for lazy handles, missing/stale items,
   generic/effect/capability signatures, docs/localization, Unicode/CRLF/BOM,
   formatter-safe insertion, deterministic rendering, resource limits, and
   migration.

RFC-0043 resolves these requirements only for its negotiated Preview. General
Semantic Transactions, Author Source documentation/localization, formatter
rewrites, generated/dependency/builtin presentation, persistent handles,
asynchronous cancellation, and Stable lifecycle remain outside its scope.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0010, DEC-0012, DEC-0023, RFC-0005,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-types`, `ling-effects`, `ling-semantic`, and `ling-source`
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

RFC-0043 intentionally defers Author Source documentation/localization,
formatter-generated insertion, generated/dependency/builtin presentation,
persistent handles, asynchronous cancellation, AI assistance, and Stable
lifecycle. Later work must not broaden its bounded checked-fact and unchanged-
edit contract without new Accepted authority.
