# LSP-2102 Authority Audit: Position-encoding negotiation

## Outcome

`LSP-2102` remains `BlockedSpec` for the complete editor position surface, but
the dependency-complete negotiation and source-projection slices are now
implemented under Accepted `RFC-0004` and `DEC-0029`:

- `LSP-2102-SOURCE-MAP` owns strict byte/position projection in `ling-source`.
- `LSP-2102-NEGOTIATION` owns the Preview initialize capability and selected
  `utf-8`/`utf-16`/`utf-32` encoding state in `ling-lsp`.

The parent remains blocked for document-version/snapshot preconditions,
handler-wide conversion for diagnostics and edits, stale-result behavior,
and Stable versus Experimental transaction compatibility. Compiler spans and
diagnostic positions remain unchanged.

## Normative traceability

- Accepted DEC-0002 makes JSON diagnostic byte offsets the protocol truth and
  human columns Unicode scalar counts; it explicitly requires a future LSP
  UTF-16 projection to be derived from SourceMap and labeled with its encoding,
  without changing Span identity.
- Accepted `RFC-0004` defines the Preview `ling lsp --stdio` initialize result
  and lifecycle boundary; it does not authorize document edits or diagnostics.
- Accepted `DEC-0029` fixes the supported wire labels, first-supported
  negotiation with UTF-16 fallback, deterministic advertisement order, and
  strict SourceMap boundary conversion.
- `docs/SEMANTICS.md` requires original UTF-8 spans, deterministic source
  mapping, and stable diagnostics; it does not define an LSP position encoding
  negotiation or conversion error protocol.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves LSP positions, snapshots, and
  Workspace Edit fields open, while `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`
  leaves the public semantic protocol lifecycle open.
- `PROTO-LSP-LIFECYCLE` inventories the current Preview lifecycle and records
  `RFC-0004`/`DEC-0029` as its authority; the overlay remains a separate
  Experimental protocol.

## Current interface evidence

The current repository now contains the bounded negotiation boundary while
confirming the remaining parent gap:

- `ling-source` exposes `PositionEncoding`, `LspPosition`, strict
  `PositionError`, negotiation, and SourceMap round trips without changing
  original byte spans.
- `ling-lsp` parses `capabilities.general.positionEncodings`, selects the first
  supported label with UTF-16 fallback, and returns the selected
  `capabilities.positionEncoding` during initialize.
- LSP unit/integration and source-map fixtures cover unknown labels, fallback,
  malformed metadata, Chinese text, emoji, combining marks, CRLF, BOM, and
  invalid UTF-8/UTF-16 boundaries.
- No URI/document-version association, stale-result policy, diagnostics
  projection, or Workspace Edit conversion is implied by these slices.

## Required authority for the remaining parent task

An implementation-ready decision or RFC must still define, at minimum:

1. document versions, snapshot identity, stale-result handling, and atomic
   preconditions for every position-bearing request;
2. handler-wide conversion ownership and error mapping for diagnostics,
   Workspace Edits, rename, completion, and code actions;
3. invalidation/cancellation behavior and Stable versus Experimental field
   lifecycle; and
4. positive, negative, cross-encoding, malformed-boundary, stale-version,
   deterministic, and migration fixtures for the complete editor surface.

## Evidence and compatibility

This audit was checked against `docs/RFC-0004.md`,
`docs/decisions/0002-source-position-units.md`,
`docs/decisions/0029-lsp-position-encoding-projection.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source`, and `crates/ling-db`.
The bounded slices add no diagnostic allocation, schema, Semantic ID,
source-span, runtime, bytecode, VM, or Unicode 17.0.0 behavior.

## Intentionally deferred

The negotiation and source-map slices are complete. The parent `LSP-2102`
remains deferred until the remaining transaction authority and fixtures exist;
future work must continue deriving editor positions from the approved
SourceMap, keep compiler byte spans authoritative, and avoid exposing
document/snapshot state as Ling semantics.
