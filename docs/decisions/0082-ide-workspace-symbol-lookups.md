# DEC-0082: Internal workspace-symbol source lookups / IDE 内部工作区符号来源查找

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0002` | `DEC-0012` | `DEC-0019` | `DEC-0073` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only exact, read-only module-name and source-name
lookups over the existing resolver-backed `ResolvedDefinitionIndex`. It does
not authorize a workspace-symbol request, search grammar, ranking, result
limit, cancellation, position projection, or protocol response.

本决定只授权在现有 resolver 支持的 `ResolvedDefinitionIndex` 上进行精确、只读的
模块名和来源名查找。不授权工作区符号请求、搜索语法、排序、结果限制、取消、位置
投影或协议响应。

## Question

`IDE-2311` needs a deterministic source of module-context symbols before any
public workspace-symbol query can be designed. The accepted definition index
already preserves user-definition identity, module/name facts, and original
UTF-8 spans. Exact lookups can expose those facts without inventing search or
editor policy.

`IDE-2311` 在设计公开工作区符号查询之前需要确定性的模块上下文符号来源。已接受的
定义索引已经保留用户定义身份、模块/名称事实和原始 UTF-8 跨度；可以公开精确查找，
而不发明搜索或编辑器策略。

## Decision

1. `ResolvedDefinitionIndex` may provide `module_symbols` and `name_symbols`
   exact lookups returning references to its existing immutable entries.
2. Results preserve the index's source/span/kind/name/identity order. A
   missing module or name returns an empty result; no result is synthesized.
3. The lookup domain remains the existing user-definition inventory. Builtins,
   Prelude entries, dependency/generated policy, package-root selection, and
   editor symbol taxonomy are not inferred or widened.
4. The lookups do not implement prefix/fuzzy matching, case-folding, ranking,
   deduplication policy, truncation metadata, result limits, cancellation,
   stale/revision binding, URI or position conversion, JSON serialization, or
   an LSP response. The public `IDE-2311` target remains `BlockedSpec`.

## Conformance plan

- Verify exact module and source-name lookup, missing-key emptiness, existing
  source/span/identity facts, Unicode/BOM/CRLF span preservation, and stable
  repeated construction.
- Verify result order equals the existing deterministic definition inventory
  order and that lookups return references without mutation or new identities.
- Keep prefix/fuzzy search, package/dependency/generated/builtin scope,
  taxonomy/containers, positions, versions, limits, truncation, cancellation,
  stale behavior, persistence, and protocol fixtures deferred.

## Compatibility impact

- Adds only internal read-only lookup methods on an existing compiler index.
  Language semantics, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
  runtime, bytecode, VM, ABI, and Unicode 17.0.0 tables remain unchanged.
- No new symbol identity is generated; all returned entries retain the
  resolver's existing `DefinitionId` and original source span.

## Unresolved alternatives

Workspace scope and package selection, symbol taxonomy/container hierarchy,
exact/prefix/fuzzy matching, visibility and duplicate policy, dependency and
generated sources, positions and versions, incremental invalidation,
result/resource limits, truncation, cancellation, stale behavior, protocol
lifecycle, and migration remain open under `IDE-2311` and the registered
incremental-cache, LSP, and Semantic Graph lifecycle gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
