# DEC-0085: Internal checked-token identity observation / 内部已检查 token 身份观察

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0002` | `DEC-0012` | `DEC-0019` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only an internal, read-only join between the existing
lexical token source index and exact checked definition facts. It does not turn
those facts into semantic-token categories, modifiers, positions, or an LSP
response.

本决定只授权将现有词法 token 来源索引与精确的已检查定义事实进行内部只读关联。不把
这些事实转换为 semantic-token 类别、modifier、位置或 LSP 响应。

## Question

`LSP-2402` eventually needs a checked source for mapping tokens to compiler
identities, but no accepted authority defines presentation categories,
precedence, redaction, or transport. The repository already exposes immutable
lexical and checked-definition observations; an exact span join records the
available facts without inventing a semantic-token mapping.

`LSP-2402` 最终需要一个将 token 关联到编译器身份的已检查来源，但现有权威尚未定义
展示类别、优先级、脱敏或传输。仓库已经提供不可变的词法与已检查定义观察；精确的跨度
关联可以记录现有事实，而不发明 semantic-token 映射。

## Decision

1. `ling-db::CompilerDb` may expose an internal
   `checked_token_source_index` query. It joins a lexical token to a checked
   definition only when source name and original UTF-8 span are exactly equal.
2. A joined entry may retain the existing definition ID, canonical type text,
   effect names, and capability names already exposed by
   `TypedDefinitionIndex`. Tokens without an exact definition span retain no
   fabricated checked facts.
3. Entries preserve lexical source order and the existing source/query key;
   successful immutable results may be cached. No new identity, hash, span,
   or category is generated.
4. The query must not classify references or non-definition tokens, choose
   semantic-token precedence, expose a legend/modifier, project positions or
   versions, redact policy, negotiate clients, encode full/delta data, or
   publish LSP/JSON-RPC output. Public `LSP-2402` remains `BlockedSpec`.

1. `ling-db::CompilerDb` 可以提供内部 `checked_token_source_index` 查询。只有 source name
   与原始 UTF-8 span 完全相等时，才将词法 token 与已检查定义关联。
2. 关联条目可以保留 `TypedDefinitionIndex` 已提供的定义 ID、规范类型文本、effect 名称
   和 capability 名称。没有精确定义跨度的 token 不生成任何虚构事实。
3. 条目保持词法源顺序与现有 source/query key；成功的不可变结果可以缓存。不生成新的
   identity、hash、span 或类别。
4. 查询不得分类 reference 或非定义 token，不选择 semantic-token 优先级，不暴露
   legend/modifier，不投影位置/版本，不定义脱敏策略，不协商客户端，不编码 full/delta
   数据，也不发布 LSP/JSON-RPC 输出。公开 `LSP-2402` 仍为 `BlockedSpec`。

## Conformance plan

- Join Unicode/BOM/CRLF definition spans and compare exact source order,
  existing Definition IDs, type text, effects, capabilities, and empty
  optional facts.
- Repeat the same checked source and compare immutable equality and cache
  reuse; edit a definition and verify the query key invalidates the old join.
- Verify identifiers and literals without exact definition spans carry no
  fabricated facts; keep references, precedence, modifiers, positions,
  versions, negotiation, cancellation, and transport fixtures deferred.

## Compatibility impact

- Adds only internal `ling-db` observation values and an accessor over existing
  lexer and checked-definition indexes.
- Language semantics, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
  runtime, bytecode, VM, ABI, dependency versions, and Unicode 17.0.0 data
  remain unchanged.
- Existing type/effect/capability facts are copied without new identity or
  presentation policy; source spans remain original UTF-8 bytes.

## Unresolved alternatives

Checked HIR/Core source precedence, reference identity mapping, semantic-token
categories/modifiers/legends, capability redaction, position/version binding,
snapshot lifecycle, client negotiation, full/delta transport, stale and
cancellation policy, protocol lifecycle, and migration remain open under
`LSP-2402`–`LSP-2404` and the registered LSP/semantic gaps.

已检查 HIR/Core 来源优先级、reference identity 关联、semantic-token 类别/modifier/legend、
capability 脱敏、位置/版本绑定、快照生命周期、客户端协商、full/delta 传输、stale 与取消
策略、协议生命周期和迁移仍由 `LSP-2402`–`LSP-2404` 及已登记缺口决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
