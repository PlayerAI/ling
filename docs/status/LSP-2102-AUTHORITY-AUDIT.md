# LSP-2102 Authority Audit: Position-encoding negotiation

## Outcome

`LSP-2102` is implementation-ready and complete under Accepted `DEC-0258`,
which composes the already accepted and implemented `DEC-0029` source
projection with `RFC-0004` initialize-time negotiation. Its only plan
dependency, `LSP-2101`, is Done.

The earlier audit incorrectly treated the complete future editor transaction
surface as part of this size-S task. The execution plan requires only recording
the common client/server encoding and routing position-bearing handlers through
one conversion API. Document versions, snapshots, stale results, incremental
edits, diagnostics publication, Workspace Edits, and cancellation are assigned
to later tasks and remain independently governed.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte offsets authoritative and
  requires editor projection to derive from `SourceMap` without changing span
  identity.
- Accepted `DEC-0029` fixes the `utf-8`, `utf-16`, and `utf-32` labels,
  first-supported selection, UTF-16 fallback, zero-based lexical positions,
  strict boundary failures, and source-layer conversion ownership.
- Accepted `RFC-0004` defines initialize parsing, selected-encoding process
  state, capability output, invalid-parameter behavior, and Preview lifecycle.
- Accepted `DEC-0258` composes those bounded authorities as the complete
  parent contract while preserving later transaction work as separate tasks.
- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` states only that the
  common encoding is recorded and all handlers use the conversion API.

## Current implementation evidence

- `ling-source` owns `PositionEncoding`, `LspPosition`, typed
  `PositionError`, deterministic negotiation, and strict `SourceFile`/
  `SourceMap` byte-position round trips.
- `ling-lsp` parses `capabilities.general.positionEncodings`, stores the
  selected encoding, returns `capabilities.positionEncoding`, and rejects
  malformed metadata before changing lifecycle state.
- The currently implemented position-bearing formatting path derives its end
  position through `SourceFile::lsp_position`; internal diagnostic and edit
  adapters also consume the shared typed projection boundary. Full-text
  synchronization contains no position range.
- Source and LSP tests cover client order, unknown labels, fallback, malformed
  metadata, Chinese text, emoji, combining marks, BOM, CRLF, empty/final lines,
  invalid UTF-8 boundaries, and UTF-16 surrogate interiors.

## Plan/repository drift resolved

The former `BlockedSpec` text conflated LSP-2102 with the open
`GAP-LSP-TRANSACTION-PROTOCOL-001` surface. DEC-0258 resolves that planning
drift without closing the gap: the gap still governs the downstream public
transaction fields and failure semantics, but it does not override the
execution plan's bounded negotiation task.

No second negotiation implementation, alternate position representation, or
ad hoc handler conversion is permitted. Later position-bearing handlers must
obtain their own Accepted method/transaction authority and reuse this boundary.

## Compatibility and determinism

This closure changes no executable behavior, protocol bytes or version,
diagnostic allocation, schema, Semantic ID, source span, runtime, bytecode, VM,
ABI, filesystem/network behavior, or Unicode 17.0.0 data. Negotiation depends
only on ordered client labels; conversion depends only on immutable source
bytes, normalized lexical mapping, and the explicit selected encoding.

## Intentionally deferred

Incremental changes, document/snapshot identity, stale-result handling,
diagnostic publication, navigation, completion, rename, code actions, semantic
tokens, cancellation, Workspace Edits, Semantic Transactions, and Stable
editor compatibility remain assigned to later execution-plan tasks.
