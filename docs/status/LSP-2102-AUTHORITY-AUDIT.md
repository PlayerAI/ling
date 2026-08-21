# LSP-2102 Authority Audit: Position-encoding negotiation

## Outcome

`LSP-2102` is correctly recorded as `BlockedSpec`. The execution plan asks the
server to negotiate a shared position encoding and route every handler through
a conversion API. Accepted Ling authority fixes UTF-8 byte spans and Unicode
scalar diagnostic columns, but does not define the LSP negotiation, conversion
schema, or error behavior.

No UTF-16/UTF-8/UTF-32 negotiation, LSP position type, conversion API, or
placeholder editor adapter was added. Compiler spans and diagnostic positions
remain unchanged.

## Normative traceability

- Accepted DEC-0002 makes JSON diagnostic byte offsets the protocol truth and
  human columns Unicode scalar counts; it explicitly requires a future LSP
  UTF-16 projection to be derived from SourceMap and labeled with its encoding,
  without changing Span identity.
- `docs/SEMANTICS.md` requires original UTF-8 spans, deterministic source
  mapping, and stable diagnostics; it does not define an LSP position encoding
  negotiation or conversion error protocol.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves LSP positions, snapshots, and
  Workspace Edit fields open, while `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`
  leaves the public semantic protocol lifecycle open.
- No LSP protocol is currently inventoried; `PROTO-SEMANTIC-GRAPH-JSON` is an
  Experimental semantic projection, not an editor position contract.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-source` and `ling-db` preserve original byte spans, normalized line
  starts, and scalar human columns; they expose no LSP UTF-16 position model.
- The compiler has no negotiated client/server encoding state, no URI/document
  version association, and no conversion failure type for surrogate/code-point
  boundaries.
- There is no LSP handler or fixture covering Chinese text, emoji, combining
  marks, CRLF, BOM, or positions inside UTF-8 scalars/UTF-16 surrogate pairs.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. supported encodings, negotiation order/fallback, protocol version, and
   server/client capability fields;
2. byte-span ↔ line/character conversion rules for UTF-8, UTF-16, and any
   additional encoding, including invalid boundary rejection and overflow;
3. treatment of CRLF, BOM, Unicode scalar/combining sequences, supplementary
   characters, document versions, and stale snapshots;
4. handler-wide conversion API ownership, error mapping, diagnostics/Workspace
   Edit projection, and Stable versus Experimental field lifecycle; and
5. positive, negative, cross-encoding, emoji/Chinese/combining/CRLF/BOM,
   malformed-boundary, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, adding a conversion helper or
negotiation field could leak an incorrect position unit into diagnostics,
rename, code actions, or edits and violate DEC-0002's byte-span boundary.

## Evidence and compatibility

This audit was checked against `docs/decisions/0002-source-position-units.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source`, and `crates/ling-db`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2102` can begin after the LSP lifecycle and position-encoding decision are
Accepted. The implementation must derive editor positions from the approved
SourceMap, keep compiler byte spans authoritative, and prove conversion
behavior with the complete Unicode/line-ending fixture matrix.
