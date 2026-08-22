# IDE-2308 Authority Audit: Completion Resolve

## Outcome

`IDE-2308` is correctly recorded as `BlockedSpec`. The execution plan asks for
lazy completion-item resolution of documentation, full signatures,
Effect/Capability information, and Unicode/formatter-safe insertion text. The
repository has internal type/effect/capability results, but no accepted resolve
handle, presentation schema, documentation source, insertion/edit, or snapshot
contract.

No completion-resolve handler, documentation renderer, signature formatter,
capability disclosure, insertion-text API, protocol field, or placeholder editor
surface was added.

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

## Required authority before implementation

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

Until these decisions are Accepted, resolve could expose unstable type or
capability details, render unapproved documentation, or insert text at a span
that does not match the current source snapshot.

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

`IDE-2308` can begin after completion-item lifecycle, presentation,
LSP-position/version, formatter, and Semantic Graph protocol decisions are
Accepted. The future implementation must consume checked data, preserve source
span/identity truth, render effects/capabilities deterministically and safely,
and label experimental fields.
