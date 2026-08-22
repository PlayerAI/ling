# DEC-0086: Internal checked-token snapshot identity / 内部已检查 token 快照身份

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0019` | `DEC-0071` | `DEC-0084` | `DEC-0085` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only the in-process identity of the immutable VFS
source snapshot used to build the checked-token observation. It does not define
an LSP document version, semantic-token encoding, result ID, full/delta
transport, or a public protocol.

本决定只授权记录构建已检查 token 观察所使用的不可变 VFS 源快照的进程内身份。不定义
LSP 文档版本、semantic-token 编码、result ID、full/delta 传输或公共协议。

## Question

`LSP-2403` eventually needs proof that a future token result is tied to one
immutable source snapshot. `CompilerDb` already keys queries by the existing
`QueryKey`, and `FileSnapshot` already exposes the session-local `SourceId` and
`Revision`. Retaining those existing values on the checked-token observation
records the boundary without inventing a transport identity.

`LSP-2403` 最终需要证明未来 token 结果绑定到单一不可变源快照。`CompilerDb` 已经使用现有
`QueryKey` 为查询建立身份，`FileSnapshot` 也已经提供进程内 `SourceId` 与 `Revision`。
在已检查 token 观察上保留这些现有值，可以记录边界而不发明传输身份。

## Decision

1. `CheckedTokenSourceIndex` retains the existing `SourceId` and session-local
   `Revision` of the `FileSnapshot` used to construct it. The values are
   observations only and are not Semantic IDs or serialized document versions.
2. Repeated queries for one unchanged `QueryKey` reuse the immutable object.
   A source edit creates a new source revision and query key; the old
   observation remains valid and unchanged, with no partial replacement.
3. The source-order lexical entries and exact checked-definition facts remain
   governed by DEC-0084 and DEC-0085. No position conversion, token category,
   modifier, fallback origin, legend, result ID, delta edit, URI, negotiation,
   cancellation, or transport field is added.
4. The observation remains an in-process `ling-db` value. Public LSP-2403
   remains `BlockedSpec` until its full/delta and transaction authorities are
   Accepted.

1. `CheckedTokenSourceIndex` 保留构建它的 `FileSnapshot` 现有 `SourceId` 与进程内 `Revision`。
   这些值只是观察，不是 Semantic ID 或序列化的文档版本。
2. 同一未改变 `QueryKey` 的重复查询复用不可变对象。源文件编辑会产生新的源 revision 与
   query key；旧观察保持有效且不变，不进行部分替换。
3. 源顺序词法条目和精确已检查定义事实继续由 DEC-0084 与 DEC-0085 约束。不添加位置转换、
   token 类别、modifier、fallback origin、legend、result ID、delta edit、URI、协商、取消或
   传输字段。
4. 该观察仍是进程内 `ling-db` 值。公开 LSP-2403 仍为 `BlockedSpec`，直到其 full/delta 与
   transaction 权威被 Accepted。

## Conformance plan

- Assert the source identity and revision are retained exactly, repeated
  unchanged queries reuse the same immutable object, and a source edit yields
  a new revision and a distinct observation without mutating the old one.
- Preserve original UTF-8 spans, source order, BOM/CRLF and Unicode spelling;
  keep URI, document-version, UTF-16 position, legend, full/delta, result-ID,
  cancellation, and transport fixtures deferred.

## Compatibility impact

- Adds only existing `SourceId`/`Revision` accessors to an internal
  `ling-db` observation; language semantics, diagnostics, schemas, Semantic
  IDs, CLI/LSP behavior, runtime, bytecode, VM, ABI, dependencies, and
  Unicode 17.0.0 data remain unchanged.
- Query identity and invalidation continue to use the existing `QueryKey` and
  VFS revision rules; no cross-process or cross-workspace numbering promise is
  created.

## Unresolved alternatives

Document URI/version binding, snapshot lifetime across requests, semantic-token
taxonomy and fallback provenance, position encoding, result-ID generation and
retention, full/delta base validation, stale/cancellation/limit policy, client
negotiation, protocol inventory, and migration remain open under LSP-2401
through LSP-2404, LSP-2501/LSP-2502, and the registered LSP/semantic gaps.

文档 URI/版本绑定、跨请求快照生命周期、semantic-token taxonomy 与 fallback provenance、位置
编码、result-ID 生成与保留、full/delta base 校验、stale/取消/限制策略、客户端协商、协议注册
表和迁移仍由 LSP-2401 至 LSP-2404、LSP-2501/LSP-2502 及已登记 LSP/semantic 缺口决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
