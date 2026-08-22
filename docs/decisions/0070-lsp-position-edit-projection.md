# DEC-0070: LSP internal position-edit projection / LSP 内部位置编辑投影

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: ide-protocol-design  
> 相关 RFC/缺口：`DEC-0029` | `DEC-0069` | `RFC-0023` | `GAP-LSP-TRANSACTION-PROTOCOL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process projection from an explicit
position encoding to the existing original-byte edit primitive. It does not
define an LSP JSON schema, URI/version policy, or public transaction method.

本决定只授权将明确的位置编码投影到既有原始字节编辑原语的进程内边界，不定义
LSP JSON schema、URI/版本策略或公共事务方法。

## Question

`DEC-0029` already defines strict UTF-8/UTF-16/UTF-32 position projection, and
`DEC-0069` defines immutable original-byte edit application. The bounded
`LSP-2104-POSITION-EDITS` child needs a deterministic composition of these two
source-layer boundaries without turning it into a public `didChange` contract.

## Decision

1. `ling-source` exposes `LspPositionEdit`, containing a half-open lexical
   `(line, character)` range and replacement bytes. It carries no URI,
   document version, JSON, request ID, or VFS handle.
2. `SourceFile::apply_lsp_position_edit` accepts an explicit
   `PositionEncoding`, converts both positions through the authoritative
   `SourceMap` using `DEC-0029`'s no-clamping rules, and delegates to
   `DEC-0069`'s UTF-8 byte edit validation.
3. `SourceFile::apply_lsp_position_edits` applies a sequence in order against
   the snapshot produced by the preceding edit. Projection errors and byte
   validation errors are typed, and a failed sequence returns no new snapshot.
4. The source projection remains BOM-free and LF-normalized for positions,
   while successful output retains exact original BOM/CRLF bytes, source ID,
   logical name, and rebuilt maps. Unsupported lines, scalar/surrogate
   interiors, normalized CRLF interiors, invalid replacements, and reversed
   byte ranges are rejected without clamping or partial publication.
5. This boundary does not negotiate encodings, mutate `VirtualFileSystem`,
   validate client versions, serialize JSON-RPC, publish diagnostics, or
   define stale-result/transaction behavior. The public `LSP-2104` parent
   remains `BlockedSpec`.

## Conformance plan

- Apply equivalent edits under UTF-8, UTF-16, and UTF-32 and compare exact
  original bytes, lexical text, source identity, and source-map results.
- Cover Chinese text, emoji, leading BOM, CRLF, empty/final lines, scalar and
  surrogate boundaries, and no-clamping out-of-range positions.
- Apply an ordered position batch, compare it with full replacement, and prove
  a later invalid projection leaves the original source unchanged.
- Repeat every operation and compare immutable `SourceFile` values; keep URI,
  version, JSON-RPC, VFS, diagnostics, and transaction fixtures deferred.

## Compatibility impact

- Adds only an in-process `ling-source` position-edit value, typed error, and
  immutable composition. Language syntax, semantics, diagnostics, schemas,
  Semantic IDs, CLI, bytecode, VM, ABI, protocols, and Unicode 17.0.0 data are
  unchanged.
- No protocol inventory entry or Stable 1.0 editor claim is added.

## Unresolved alternatives

Negotiation and capability advertisement, URI/document identity, version and
stale-result checks, VFS publication, JSON-RPC `didChange`, Workspace Edits,
range transactions, and public compatibility remain open under the blocked
`LSP-2104` parent and `GAP-LSP-TRANSACTION-PROTOCOL-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

