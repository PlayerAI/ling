# LSP-2403 Authority Audit: Full and Delta Tokens

## Outcome

`LSP-2403` is correctly recorded as `BlockedSpec`. The execution plan suggests
full token responses first and delta responses after performance evidence, with
tokens sorted by position, non-overlapping, and generated against one document
version. The repository has no accepted full/delta wire schema, token legend,
result-ID lifecycle, base-result validation, or stale-version behavior.

No full-token writer, delta encoder, result-ID store, protocol field, diagnostic
allocation, or placeholder LSP surface was added.

## Normative traceability

- The execution package is non-normative; its full/delta ordering and performance
  sequencing do not authorize a public transport protocol.
- LSP-2401 token taxonomy and LSP-2402 typed generation are both
  `BlockedSpec`, so a transport cannot freeze their categories or source-origin
  semantics.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  token integer encoding, result IDs, delta edits, or version preconditions.
- DEC-0012 fixes Semantic IDs/canonical bytes; the Semantic Graph projections
  are Experimental and are not a token-result cache or delta base.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version and editor fields
  open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves protocol stability,
  reader/writer compatibility, stale rejection, and migration open. LSP-2501
  and LSP-2502 separately remain unaccepted snapshot/cancellation contracts.

## Current interface evidence

- No crate writes a semantic-token legend, full response, delta response,
  `resultId`, `previousResultId`, or delta edit list.
- `ling-semantic` produces deterministic graph data, but its Experimental JSON
  identity and ordering are not a token transport or cache compatibility rule.
- `ling-source` has byte/source maps and revisions, but no same-version token
  requirement, stale-result rejection, cancellation publication rule, or
  non-overlap validator.
- No fixture proves full/delta equivalence, base-result mismatch recovery,
  changed token ranges, document version transitions, limits, cancellation,
  Unicode/CRLF/BOM positions, or deterministic encoding.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. full response schema, token legend, integer/relative encoding, result ID
   generation and retention, document URI/version/snapshot binding, and
   cancellation/limit behavior;
2. delta request base-result semantics, edit operation/index rules, mismatch or
   expired-base fallback to full, result-ID migration, memory/eviction policy,
   and failure responses;
3. token ordering/non-overlap/duplicate rules, UTF-8/UTF-16 SourceMap
   projection, taxonomy/modifier stability, source-origin and Semantic ID
   provenance, and generated/dependency/error fallback policy;
4. protocol inventory, Stable versus Experimental fields, client capability
   negotiation, diagnostics, and migration; and
5. executable fixtures for full output, deltas with insertion/deletion/reorder,
   base mismatch, stale versions, cancellation, limits, full/delta semantic
   equivalence, Chinese/emoji/CRLF/BOM positions, deterministic bytes, and
   migration.

Until these decisions are Accepted, delta state could be applied to the wrong
document, produce overlapping/out-of-order tokens, or turn an internal cache
identifier into an external compatibility promise.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-semantic`, `ling-source`, and schema/fixture directories.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2403` can begin after LSP-2401 taxonomy, LSP-2402 generation,
LSP-2501/LSP-2502 snapshot/cancellation, and Semantic Graph lifecycle decisions
are Accepted. The future implementation must publish only same-version,
non-overlapping deterministic tokens, recover to full on invalid bases, and
label experimental fields.
