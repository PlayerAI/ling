# DEC-0029: LSP position-encoding projection

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0002`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision fixes only the deterministic projection between Ling's
original-byte source model and negotiated editor positions. It does not define
an LSP server, document lifecycle, document versions, Workspace Edits,
Semantic Transactions, diagnostics publication, or any public editor command.

## Question

The source layer already preserves original UTF-8 byte offsets and a
BOM-free, LF-normalized lexical view, but an editor adapter still needs a
deterministic conversion for its negotiated position encoding. The conversion
boundary must be complete without deciding the separate LSP lifecycle,
snapshot, or edit protocol.

## Decision

1. The source projection supports the wire labels `utf-8`, `utf-16`, and
   `utf-32`. `utf-8` counts UTF-8 bytes, `utf-16` counts UTF-16 code units,
   and `utf-32` counts Unicode scalar values. The implementation advertises
   these encodings in that deterministic order.
2. Negotiation consumes the client's ordered list and selects its first
   recognized label. Unknown labels are ignored. An absent or empty list uses
   the LSP-compatible `utf-16` fallback. The selected encoding is explicit in
   the projection API and is never inferred from host locale.
3. A position is zero-based `(line, character)`. Lines and characters are
   measured in the BOM-free, LF-normalized lexical view. The original UTF-8
   byte offset remains authoritative and is recovered through `SourceMap`.
   A leading BOM therefore maps lexical line 0, character 0 to original byte
   offset 3, while CRLF is one lexical newline whose original span is retained
   by the source map.
4. Byte-to-position conversion accepts only an original offset that maps to a
   lexical character boundary. Offsets inside a UTF-8 scalar or inside a
   normalized CRLF sequence are rejected. UTF-8 positions count bytes;
   UTF-16 positions count surrogate-pair code units; UTF-32 positions count
   scalar values.
5. Position-to-byte conversion rejects unknown lines, positions beyond the
   lexical line content, UTF-8 positions inside a scalar, and UTF-16 positions
   inside a surrogate pair. It does not silently clamp. A valid position maps
   back to an original `ByteOffset` through `SourceMap`.
6. Conversion failures are typed library errors. They do not allocate a
   diagnostic code, change Semantic IDs, or alter compiler spans. Public LSP
   error responses, stale-version handling, and edit-field compatibility
   remain governed by a later accepted protocol decision.

## Conformance plan

- Round-trip every valid character boundary for UTF-8, UTF-16, and UTF-32.
- Cover a leading BOM, CRLF, Chinese identifiers, emoji, combining marks,
  empty lines, final lines without a newline, and positions at line ends.
- Reject offsets inside UTF-8 scalars, CRLF bytes, and UTF-16 surrogate pairs;
  reject unknown lines and overlong character values without clamping.
- Verify negotiation order, unknown-label filtering, UTF-16 fallback, and
  repeatable results across process runs and source-map rebuilds.

## Compatibility impact

- Adds an internal `ling-source` position projection API used by a future LSP
  adapter; the LSP server and public protocol inventory remain unchanged.
- Preserves DEC-0002's original UTF-8 byte spans, Unicode-scalar human
  diagnostics, normalized lexical view, and deterministic source mapping.
- Adds no source syntax, diagnostic code, JSON schema, Semantic ID, bytecode,
  runtime, CLI, Workspace Edit, or Semantic Transaction behavior.

## Unresolved alternatives

Document URI identity, snapshot/version preconditions, stale edits,
diagnostic/rename range publication, JSON-RPC lifecycle, and Stable versus
Preview editor fields remain open under `GAP-LSP-TRANSACTION-PROTOCOL-001` and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
