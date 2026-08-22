# DEC-0084: Internal lexical token source index / 内部词法 token 来源索引

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0002` | `DEC-0019` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only a read-only internal inventory of the existing
lexer tokens, their original UTF-8 spans, and their exact source spelling. It
does not choose an LSP semantic-token taxonomy, legend, modifier set, range
encoding, transport, or editor response.

本决定只授权对现有 lexer token、原始 UTF-8 跨度及精确源文本的只读内部索引。不选择
LSP semantic token 的 taxonomy、legend、modifier、range 编码、传输或编辑器响应。

## Question

`LSP-2401` needs a lossless compiler-owned source for any future semantic-token
design, but the accepted authorities do not yet define semantic categories or
the LSP transport. The existing `ling-syntax` lexer already exposes a stable
token-kind name and original-byte span; recording those facts avoids inventing
editor semantics while making the next design step testable.

`LSP-2401` 需要一个无损、由编译器拥有的来源供未来 semantic-token 设计使用，但现有
权威尚未定义语义类别或 LSP 传输。`ling-syntax` 已经提供稳定的 token-kind 名称和
原始字节跨度；记录这些事实可以避免发明编辑器语义，同时让下一步设计可测试。

## Decision

1. `ling-db::CompilerDb` may expose an internal `token_source_index` query for
   one exact VFS source revision. Each entry retains the lexer `TokenKind`, the
   original UTF-8 `Span`, and the exact original source spelling.
2. Entries preserve lexer source order, including layout, trivia, error, and
   EOF tokens. The index exposes only whether lexical errors occurred; it does
   not reinterpret or repair an invalid stream.
3. The index is cached by the existing source query key and publishes no
   result when span-to-source projection fails. It uses no path, URI, document
   version, position encoding, semantic ID, or hash-map order as language
   behavior.
4. The index must not define semantic-token categories, modifiers, legends,
   client negotiation, full/delta encoding, cancellation, stale handling,
   limits, JSON-RPC, or an LSP response. Public `LSP-2401` remains
   `BlockedSpec`.

1. `ling-db::CompilerDb` 可以针对一个精确的 VFS source revision 提供内部
   `token_source_index` 查询。每一项保留 lexer `TokenKind`、原始 UTF-8 `Span` 以及
   精确的原始源文本。
2. 条目保持 lexer 源顺序，包括 layout、trivia、error 和 EOF token。索引只暴露是否
   存在词法错误，不重新解释或修复无效 token 流。
3. 索引按现有 source query key 缓存；跨度到源文本的投影失败时不发布结果。不使用路径、
   URI、文档版本、位置编码、Semantic ID 或 hash-map 顺序作为语言行为。
4. 索引不得定义 semantic-token 类别、modifier、legend、客户端协商、full/delta 编码、
   取消、stale 处理、限制、JSON-RPC 或 LSP 响应。公开 `LSP-2401` 仍为 `BlockedSpec`。

## Conformance plan

- Index ASCII, Chinese, BOM, CRLF, comments, layout, literal, delimiter,
  error, and EOF tokens and compare exact original bytes and source order.
- Repeat the same VFS revision and compare immutable equality and cache reuse;
  edit the source and verify the index changes with the source query key.
- Preserve lexical-error visibility without publishing a semantic token or
  silently repairing invalid source; keep positions, versions, URI, limits,
  cancellation, and transport fixtures deferred.

## Compatibility impact

- Adds only internal `ling-db` observation values, an accessor, and a typed
  projection error over the existing lexer.
- Language semantics, registered diagnostics, schemas, Semantic IDs, CLI/LSP
  behavior, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 data
  remain unchanged.
- No new token identity or semantic category is generated; all spans remain
  original UTF-8 byte spans.

## Unresolved alternatives

Semantic-token taxonomy and precedence, declaration/use classification,
modifiers, legends, position/version binding, document/project snapshots,
client negotiation, full/delta transport, stale/cancellation/resource policy,
protocol lifecycle, and migration remain open under `LSP-2401`–`LSP-2404` and
the registered LSP transaction and semantic lifecycle gaps.

semantic-token taxonomy 与优先级、声明/使用分类、modifier、legend、位置/版本绑定、文档/
项目快照、客户端协商、full/delta 传输、stale/取消/资源策略、协议生命周期和迁移仍由
`LSP-2401`–`LSP-2404` 及已登记的 LSP transaction/semantic lifecycle 缺口决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
