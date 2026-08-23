# DEC-0258: Current LSP position-encoding boundary / 当前 LSP 位置编码边界

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：ide-protocol-design
> 相关 RFC/缺口：DEC-0002 | DEC-0029 | RFC-0004 | LSP-2102
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision accepts the implemented source projection and initialize-time
negotiation as the complete bounded LSP-2102 position-encoding boundary. It
does not accept document transactions, future editor methods, or Stable LSP
compatibility.

本决定接受已实现的源位置投影和初始化阶段协商，作为完整且有界的 LSP-2102
位置编码边界；它不接受文档事务、未来编辑器方法或 Stable LSP 兼容性。

## Question

Does LSP-2102 require the document-version, snapshot, stale-result, Workspace
Edit, and cancellation contracts assigned to later execution-plan tasks, or is
its stated negotiation-and-conversion contract complete when RFC-0004 and
DEC-0029 are composed and every implemented position-bearing handler uses the
shared source conversion API?

## Decision

1. LSP-2102 means exactly two composed boundaries: DEC-0029's strict
   original-byte/position projection in `ling-source`, and RFC-0004's
   initialize-time negotiation and selected per-server encoding state in
   `ling-lsp`.
2. Supported wire labels are `utf-8`, `utf-16`, and `utf-32`. The client list
   is processed in order, unknown labels are ignored, and an absent, empty, or
   unsupported-only list selects `utf-16`. Malformed metadata is rejected
   before the lifecycle state changes.
3. Original UTF-8 byte spans remain authoritative. Editor positions use the
   BOM-free, LF-normalized lexical view and are converted only through
   `ling-source` `SourceFile`/`SourceMap` APIs with an explicit negotiated
   `PositionEncoding`; no LSP handler may implement ad hoc byte, scalar, or
   UTF-16 counting.
4. The conversion rule applies to every position-bearing handler currently
   implemented and to each later handler when its own authority is accepted.
   At this boundary, initialize advertises the selected encoding, formatting
   projects its whole-document range through `SourceFile`, and internal
   diagnostic/edit adapters consume the same typed projection primitives.
5. Full-text synchronization contains no position range. Incremental change
   ranges, document versions and snapshots, stale-result behavior, diagnostic
   publication, Workspace Edits, cancellation, and future editor requests
   remain owned by LSP-2103 onward and their Accepted authorities. Their
   absence does not make the narrower LSP-2102 negotiation task incomplete.
6. `ling.lsp.lifecycle/0.1` remains Preview/current-writer-only. This decision
   changes no wire bytes, capability, command, method, error, protocol version,
   or stability level and makes no complete-editor or Stable 1.0 claim.
7. Existing source-map, negotiation, lifecycle, formatting, diagnostic, and
   position-edit tests are the executable parent evidence. A duplicate
   conversion layer or broader transaction implementation is not permitted by
   this parent task.

## Conformance plan

- Retain source-map round trips and strict failures for UTF-8, UTF-16, and
  UTF-32 across BOM, CRLF, Chinese text, emoji, combining marks, empty/final
  lines, invalid scalar boundaries, and UTF-16 surrogate interiors.
- Retain initialize fixtures for first-supported selection, unknown-label
  filtering, UTF-16 fallback, malformed metadata rejection before transition,
  and exact selected-capability output.
- Retain formatting, diagnostic projection, and position-edit tests proving
  implemented position-bearing paths consume the shared typed conversion API.
- Verify deterministic results without filesystem, environment, locale,
  allocation-order, hash-order, or network inputs.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, offline, and deterministic-diff gates.

## Compatibility impact

- **LSP:** accepts already implemented Preview behavior; no command, method,
  capability, field, error, frame, or output byte changes.
- **Protocol:** adds parent-level authority evidence to
  `PROTO-LSP-LIFECYCLE`; its marker and Preview stability remain unchanged.
- **Language/compiler:** no syntax, type, Effect, Capability, Checked Core,
  diagnostic, Semantic ID, source-span, runtime, bytecode, VM, or ABI change.
- **Schemas/data/migration:** none because no serialized representation or
  reader/writer behavior changes.
- **Filesystem/network:** none; negotiation and conversion are process-local.
- **Determinism/Unicode:** no new input or ordering source; Unicode remains
  17.0.0 and original UTF-8 bytes remain authoritative.

## Unresolved alternatives

Incremental synchronization, document/snapshot identity, stale results,
diagnostic publication, navigation, completion, rename, code actions, semantic
tokens, cancellation, Workspace Edits, Semantic Transactions, and Stable
editor compatibility remain separately governed. Any incompatible position
field or lifecycle change requires its own Accepted authority and migration
evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
