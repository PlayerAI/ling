# LSP-2102 implementation report

> Status: Done / 已完成
> Task: `LSP-2102`
> Authority: Accepted `DEC-0002`, `DEC-0029`, `RFC-0004`, and `DEC-0258`

## Scope

This parent milestone accepts the existing position-encoding implementation as
the complete bounded execution-plan task: initialize records the common
client/server encoding, and every implemented position-bearing path uses the
shared strict source conversion API. It adds no executable behavior.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2102: record the
  common encoding and require handlers to use the conversion API.
- `DEC-0002`: original UTF-8 byte spans remain authoritative.
- `DEC-0029` §§1–6: labels, negotiation, lexical position units, strict
  boundary validation, SourceMap ownership, and typed failures.
- `RFC-0004` §§3–5: lifecycle state, initialize metadata, selected capability,
  invalid-parameter behavior, and deterministic output.
- `DEC-0258` §§1–7: exact parent composition, implemented-handler ownership,
  and separation from downstream transaction tasks.

## Implementation and evidence

- `crates/ling-source/src/position.rs` owns `PositionEncoding`,
  `LspPosition`, negotiation, typed failures, and strict source projection.
- `crates/ling-lsp/src/lib.rs` stores the negotiated encoding, parses and
  validates initialize metadata, emits the selected capability, and routes
  implemented formatting range projection through `SourceFile`.
- `crates/ling-lsp/src/diagnostics.rs` and the internal position-edit boundary
  consume the same source-layer types; full-text synchronization has no range.
- Focused source, negotiation, lifecycle, overlay, formatting, diagnostic, and
  edit tests provide positive, negative, Unicode, normalization, and
  deterministic evidence.

## Compatibility and determinism

- No command, option, LSP method, capability, field, response, protocol marker,
  stability level, diagnostic code, schema, or migration changes.
- No syntax, Checked Core, Semantic ID, span, runtime, bytecode, VM, ABI,
  package, filesystem, or network behavior changes.
- Original UTF-8 spans remain authoritative; conversion inputs are only source
  bytes, normalized mapping, and explicit encoding. Unicode remains 17.0.0.

## Verification

Focused package tests and the full locked, offline workspace, CI, governance,
support, status, RC0, traceability, Clippy, formatting, and deterministic-diff
gates passed before acceptance commit
`15cdfe7963ecd272447ff9ac00d1b71df0a63800`. The task registry binds that
exact immutable commit.

## Intentionally deferred

Incremental changes, document/snapshot identity, stale results, diagnostics
publication, navigation, completion, rename, code actions, semantic tokens,
cancellation, Workspace Edits, Semantic Transactions, and Stable editor
compatibility remain owned by later tasks and Accepted authorities.
