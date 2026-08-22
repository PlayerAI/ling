# DEC-0072: Internal diagnostic span projection / LSP 内部诊断范围投影

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: ide-protocol-design  
> 相关 RFC/缺口：`DEC-0002` | `DEC-0029` | `DEC-0034` | `GAP-LSP-TRANSACTION-PROTOCOL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an internal projection of an existing compiler
`DiagnosticSpan` from original UTF-8 byte offsets to an explicit source-layer
`LspPosition` range. It does not define an LSP diagnostic schema, field
mapping, publication lifecycle, or document/snapshot protocol.

本决定只授权将既有编译器 `DiagnosticSpan` 的原始 UTF-8 字节范围投影为明确的
源层 `LspPosition` 范围，不定义 LSP diagnostic schema、字段映射、发布生命周期
或文档/快照协议。

## Question

The blocked `LSP-2201` parent requires a future mapping from stable compiler
diagnostics to editor ranges. `DEC-0002` and `DEC-0029` already make original
byte spans and explicit position encodings authoritative. A source-only
projection can therefore be fixed without deciding how diagnostics are
published or serialized.

## Decision

1. `ling-lsp` may keep a `pub(crate)` `DiagnosticPositionRange` containing only
   start and end `LspPosition` values, plus a typed projection error.
2. Projection accepts a compiler `DiagnosticSpan`, a validated `SourceFile`,
   and an explicit `PositionEncoding`. The diagnostic logical name must equal
   the source logical name exactly; URI, path, and workspace identity are not
   inferred.
3. Original start/end offsets must be ordered, fit the source-layer `u32`
   byte-offset domain, and map through `SourceFile::lsp_position` without
   clamping. BOM removal, CRLF normalization, scalar boundaries, and UTF-8,
   UTF-16, and UTF-32 counting remain governed by `DEC-0002`/`DEC-0029`.
4. Failed identity, range, offset, or position validation returns a typed
   error and produces no range. The helper never mutates a source, diagnostic,
   VFS, query cache, or server state.
5. The range contains no code, severity, localized message, Facts, repairs,
   Semantic ID, version, snapshot, URI, JSON, or publication state. The public
   `LSP-2201` parent remains `BlockedSpec`.

## Conformance plan

- Project Unicode, Chinese, emoji, leading-BOM, CRLF, and final-newline spans
  under all three explicit position encodings and compare exact positions.
- Reject mismatched logical names, reversed spans, offsets outside `u32`,
  normalized/interior byte boundaries, and source-end overflow without
  clamping or partial output.
- Repeat projections and compare immutable ranges and errors; preserve the
  existing diagnostic JSON/CLI output and diagnostic ordering behavior.
- Keep severity/tags, related information, Facts/repairs, localization,
  document versions, stale results, publication, and JSON-RPC fixtures
  deferred to accepted parent-protocol decisions.

## Compatibility impact

- Adds only private `ling-lsp` projection values and an existing local crate
  dependency. Ling syntax, semantics, diagnostics, schemas, Semantic IDs,
  CLI output, LSP wire methods, runtime, bytecode, VM, ABI, and Unicode
  17.0.0 data remain unchanged.
- No diagnostic code allocation, protocol-inventory entry, Stable claim, or
  migration is introduced.

## Unresolved alternatives

Severity and tag mapping, related-information policy, localization, repair and
Fact schemas, URI/version association, snapshot freshness, publication and
clearance, cancellation, suppression, root-cause grouping, and Stable versus
Experimental lifecycle remain open under the blocked `LSP-2201`/`LSP-2204`
parents and registered protocol gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
