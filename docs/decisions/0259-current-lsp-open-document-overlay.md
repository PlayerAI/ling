# DEC-0259: Current LSP open-document overlay / 当前 LSP 打开文档覆盖层

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：ide-protocol-design
> 相关 RFC/缺口：RFC-0023 | DEC-0019 | LSP-2103
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision accepts the implemented RFC-0023 full-text overlay as the
complete bounded LSP-2103 task. It does not accept incremental edits, compiler
transactions, host-path resolution, or Stable editor compatibility.

本决定接受已实现的 RFC-0023 全文覆盖层，作为完整且有界的 LSP-2103 任务；
它不接受增量编辑、编译器事务、宿主路径解析或 Stable 编辑器兼容性。

## Question

Does LSP-2103 require the incremental-edit, compiler-snapshot, diagnostic, and
Workspace Edit work assigned to later tasks, or is its stated open-document
overlay contract complete through the Accepted RFC-0023 implementation?

## Decision

1. LSP-2103 means exactly the RFC-0023 `ling.lsp.overlay/0.1` full-text
   document state: a restricted URI maps to one session-local VFS file whose
   record contains the last accepted client version, visible bytes, and
   open/closed state.
2. `didOpen` publishes exact editor UTF-8 bytes as an overlay over the disk
   layer. An already-open URI or a reopen version that is not strictly newer
   is rejected before VFS mutation.
3. `didChange` accepts exactly one full-text replacement with a strictly
   increasing non-negative version. Range and `rangeLength` fields are not
   LSP-2103 behavior and are rejected without mutation.
4. `didClose` removes only the overlay for workspace/dependency documents and
   reveals the latest disk layer; closing an untitled document removes its
   temporary VFS file. Duplicate or unknown close operations do not mutate
   other documents.
5. Dependency documents are readable overlays but never writable. URI forms,
   logical names, sizes, state, writability, and versions are validated before
   publication. `SourceId`, VFS revision, host path, and allocation or map
   order never appear as wire semantics.
6. The current request-form responses are conformance probes permitted by
   RFC-0023; ordinary notifications remain response-free. Overlay methods obey
   the RFC-0004 lifecycle gates and framed stdout ownership.
7. Incremental ranges belong to LSP-2104. Workspace reload, compiler request
   snapshots, stale analysis, diagnostics, cancellation, Workspace Edits, and
   Semantic Transactions belong to later tasks. Their absence does not make
   the narrower LSP-2103 overlay incomplete.
8. `ling.lsp.overlay/0.1` remains Experimental/current-writer-only. This
   decision changes no method, field, error, protocol byte, version, or
   stability level and makes no complete-editor or Stable 1.0 claim.

## Conformance plan

- Retain workspace, dependency, and untitled URI fixtures for open/change/
  close, overlay precedence, disk changes hidden while open, disk reveal,
  temporary removal, and deterministic document ordering.
- Retain rejection fixtures for malformed/unsupported URIs, duplicate opens,
  stale or duplicate versions, closed/unknown documents, read-only changes,
  ranged changes, invalid parameters, and oversized text; prove rejected
  operations leave VFS bytes, version, and revision unchanged.
- Retain request-versus-notification response rules, lifecycle gates, framed
  transport, exact IDs, bilingual errors, and repeated-run determinism.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, offline, and deterministic-diff gates.

## Compatibility impact

- **LSP:** accepts already implemented Experimental behavior; no command,
  method, capability, field, error, frame, or output byte changes.
- **Protocol:** adds parent-level authority evidence to `PROTO-LSP-OVERLAY`;
  its marker and stability remain unchanged.
- **Language/compiler:** no syntax, type, Effect, Capability, Checked Core,
  diagnostic, Semantic ID, source-span, runtime, bytecode, VM, or ABI change.
- **Schemas/data/migration:** none because serialized behavior is unchanged.
- **Filesystem/network:** none; URI mapping is host-independent and no host
  path, project discovery, disk read, or network operation is added.
- **Determinism/Unicode:** no new input or ordering source; exact UTF-8 bytes
  and Unicode 17.0.0 behavior remain unchanged.

## Unresolved alternatives

`file://` resolution, project-root discovery, generated-file policy,
incremental synchronization, compiler snapshot identity, stale results,
diagnostics publication, navigation, cancellation, Workspace Edits, Semantic
Transactions, and Stable editor compatibility remain separately governed.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
