# DEC-0069: LSP internal UTF-8 edit primitive / LSP 内部 UTF-8 编辑原语

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: ide-protocol-design  
> 相关 RFC/缺口：`RFC-0023` | `DEC-0002` | `DEC-0019` | `DEC-0029` | `GAP-LSP-TRANSACTION-PROTOCOL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process source-layer primitive for the
bounded `LSP-2104-UTF8-EDITS` child. It does not accept the public LSP
incremental-change schema, position negotiation, document versions, or a
transaction protocol.

本决定只授权 `LSP-2104-UTF8-EDITS` 子任务所需的进程内源码层原语，不接受公共
LSP 增量变更 schema、位置协商、文档版本或事务协议。

## Question

The accepted overlay keeps immutable full-text snapshots, while the execution
plan also requires evidence that an ordered byte edit can be applied without
splitting Unicode or normalized CRLF boundaries. The public LSP range and
transaction contract remains open, so this boundary must be source-only and
must not become an accidental wire API.

## Decision

1. `ling-source` exposes `Utf8Edit`, containing a half-open original UTF-8
   byte range `[start, end)` and replacement bytes. It carries no URI,
   document version, negotiated position, JSON, or transport field.
2. `SourceFile::apply_utf8_edit` applies one edit to an immutable source and
   `SourceFile::apply_utf8_edits` applies a supplied sequence in order. Each
   range is checked against the snapshot produced by the preceding edit; a
   failed sequence returns an error and publishes no replacement snapshot.
3. An edit MUST have `start <= end`, both offsets within the source, and both
   offsets at UTF-8 scalar boundaries. An offset in the interior of a CRLF
   pair is also rejected because it is not a boundary in the normalized
   lexical view. Leading BOM boundaries remain representable in the original
   byte view.
4. Replacement bytes are revalidated through `SourceFile::from_bytes`.
   Invalid UTF-8, misplaced BOMs, and a result larger than the accepted `u32`
   source-span unit fail before a new `SourceFile` is returned. Successful
   edits preserve `SourceId` and logical name and rebuild the source map and
   lexical view from the exact resulting bytes.
5. The primitive does not mutate `VirtualFileSystem`, check client versions,
   convert `LspPosition`, serialize JSON-RPC, publish diagnostics, or define
   stale-result and transaction behavior. The parent `LSP-2104` task remains
   `BlockedSpec` for those public contracts.

## Conformance plan

- Apply partial and full replacements over Chinese, emoji, combining text,
  leading BOM, CRLF, and final-newline inputs and compare exact original bytes,
  lexical text, source identity, and rebuilt maps.
- Apply an ordered multi-edit sequence and compare it with the equivalent full
  replacement result.
- Reject reversed and out-of-bounds ranges, offsets inside Unicode scalars or
  CRLF pairs, invalid UTF-8/replacement BOMs, and failed later edits; verify
  the original source remains unchanged after every failure.
- Repeat the same edit sequence and compare `SourceFile` values to establish
  deterministic results independent of allocation order or host paths.
- Keep public LSP, JSON, version, VFS publication, diagnostics, and
  transaction fixtures deferred to the parent authority gate.

## Compatibility impact

- Adds only an in-process `ling-source` value, typed error, and immutable
  source transformation. Ling syntax, semantics, diagnostics, schemas,
  Semantic IDs, CLI behavior, bytecode, VM, ABI, protocols, and Unicode
  17.0.0 data are unchanged.
- The primitive retains original UTF-8 byte spans and does not add a protocol
  inventory record or Stable 1.0 editor claim.

## Unresolved alternatives

Negotiated UTF-8/UTF-16/UTF-32 positions, URI and document-version checks,
VFS publication, stale compiler results, JSON-RPC `didChange`, Workspace Edits,
range transactions, and public compatibility remain open under
`GAP-LSP-TRANSACTION-PROTOCOL-001` and the blocked `LSP-2104` parent.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

