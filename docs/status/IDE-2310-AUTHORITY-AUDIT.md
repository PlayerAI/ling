# IDE-2310 Authority Audit: Formatting

## Outcome

`IDE-2310` is correctly recorded as `BlockedSpec`. The execution plan proposes
calling `ling-fmt` for document formatting and enabling range formatting only
after semantic-boundary decisions and tests exist. The repository does have an
internal Author Source formatter governed by Accepted DEC-0023, but it has no
accepted LSP formatting request/response, document-version, edit, CLI, or range
formatting contract.

No LSP formatter adapter, `ling-fmt` command, range-formatting behavior,
Workspace Edit schema, protocol field, or placeholder editor surface was added.

## Normative traceability

- The execution package is non-normative; its `ling-fmt` and range-formatting
  wording does not authorize a public command or editor protocol.
- DEC-0023 is Accepted: Author formatting consumes authoritative compiler CST
  and spans, preserves identifier/literal/comment/documentation bytes and
  localized spelling, is idempotent for valid source, and publishes no partial
  output. It explicitly separates Author Source from Audit Source and leaves
  broader formatter policy open.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  formatting request ranges, URIs, versions, or edit application.
- DEC-0015 fixes canonical Audit Source separately from Author Source.
- `GAP-FORMATTER-AUTHOR-SOURCE-001` leaves broader normalization and
  presentation policy open; `GAP-FORMATTER-CLI-PROTOCOL-001` leaves command,
  stdin, check, exit, and report behavior open.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version preconditions and
  Stable versus Experimental edit fields open, while
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves protocol migration open.

## Current interface evidence

- `ling-format` implements the internal Author Source preservation boundary and
  tests CST/comment/invalid-source behavior, but it exposes no LSP request,
  negotiated position encoding, document version, or Workspace Edit adapter.
- No `ling-fmt` public CLI contract exists in the protocol inventory; the
  formatter CLI gap explicitly remains open.
- Range formatting has no accepted boundary semantics for partial CST regions,
  comment attachment, invalid/incomplete input, or overlapping ranges.
- No executable editor fixture covers full/range requests, UTF-8/UTF-16
  projection, BOM/CRLF, comments/docs/Unicode, invalid fallback, idempotence,
  stale versions, deterministic edits, or migration.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. document/range formatting request scope, URI/package ownership, source
   snapshot/version, position encoding, cancellation, limits, and stale-result
   behavior;
2. full-document and range boundaries, CST completeness/invalid-source fallback,
   comment/doc attachment, line-ending/BOM policy, Unicode spelling, final-LF
   policy, idempotence, semantic-equivalence checks, and deterministic edit
   generation;
3. Workspace Edit response schema, URI/path normalization, byte-to-position
   conversion, edit ordering/overlap/atomicity, diagnostics, protocol inventory,
   Stable versus Experimental fields, and migration;
4. `ling-fmt` CLI/library ownership, stdin/logical filename behavior, check and
   report semantics, and how editor formatting reuses the formatter without
   becoming a second parser; and
5. executable positive/negative full/range fixtures for valid, incomplete, and
   invalid source, comments/docs, Unicode/NFC, BOM/CRLF, stale versions,
   deterministic output, idempotence, semantic equivalence, and migration.

Until these decisions are Accepted, editor formatting could apply partial or
stale edits, change author-controlled bytes, or make range formatting a second
language/formatter authority.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0015, DEC-0023,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-format`.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2310` can begin after formatter CLI/editor, LSP transaction, range-boundary,
and protocol lifecycle decisions are Accepted. The future implementation must
reuse `ling-format`'s checked CST boundary, preserve source bytes and spans,
publish atomic versioned edits, and label experimental fields.
