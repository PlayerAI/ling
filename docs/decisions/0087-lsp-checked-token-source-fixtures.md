# DEC-0087: Internal checked-token source fixture corpus / 内部已检查 token 源 fixture 集

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0002` | `DEC-0019` | `DEC-0071` | `DEC-0084` | `DEC-0085` | `DEC-0086` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes a small executable fixture corpus for the internal
checked-token source observation. The fixtures freeze byte/span/source-order
and VFS-revision evidence only; they do not define semantic-token categories,
positions, fallback presentation, or an LSP response.

本决定只授权内部已检查 token 源观察的最小可执行 fixture 集。fixture 仅冻结字节/span/源顺序
与 VFS revision 证据，不定义 semantic-token 类别、位置、fallback 展示或 LSP 响应。

## Question

The existing lexical and checked-identity observations need regression evidence
for Unicode identifiers, emoji literals, a leading BOM, CRLF boundaries, exact
original bytes, source ordering, and revision invalidation. The public LSP-2404
fixture contract is still unspecified, so these tests must remain compiler-owned
and must not contain expected semantic-token output.

现有词法与已检查身份观察需要覆盖 Unicode 标识符、emoji literal、前导 BOM、CRLF 边界、原始
字节、源顺序和 revision invalidation 的回归证据。公开 LSP-2404 fixture 合约仍未定义，因此
这些测试必须归编译器所有，不能包含 semantic-token 期望输出。

## Decision

1. `ling-db` may include internal fixtures that call
   `CompilerDb::checked_token_source_index` and assert existing source spans,
   exact source spelling, source order, checked-definition facts, `SourceId`,
   and session-local `Revision`.
2. Fixtures must include a leading BOM, CRLF, Chinese identifiers, and an emoji
   literal, and must compare token spans against the original bytes. A separate
   edit fixture must prove immutable cache reuse and new-revision invalidation.
3. Fixtures must not assert or serialize token legends, categories, modifiers,
   positions, document versions, fallback origins, result IDs, full/delta
   responses, negotiation, cancellation, limits, or JSON-RPC fields.
4. Public LSP-2404 remains `BlockedSpec`; this corpus is evidence for the
   internal source boundary only and may not be used to claim semantic-token
   protocol support.

1. `ling-db` 可以包含调用 `CompilerDb::checked_token_source_index` 的内部 fixture，断言现有
   source span、精确源码拼写、源顺序、已检查定义事实、`SourceId` 和进程内 `Revision`。
2. fixture 必须包含前导 BOM、CRLF、中文标识符与 emoji literal，并将 token span 与原始字节
   比较。另一个编辑 fixture 必须证明不可变缓存复用与新 revision invalidation。
3. fixture 不得断言或序列化 token legend、类别、modifier、位置、文档版本、fallback origin、
   result ID、full/delta 响应、协商、取消、限制或 JSON-RPC 字段。
4. 公开 LSP-2404 仍为 `BlockedSpec`；本 corpus 仅是内部源边界证据，不能用于宣称
   semantic-token 协议支持。

## Conformance plan

- Run the Unicode/BOM/CRLF fixture and compare exact original-byte slices and
  monotonic source spans.
- Run the revision fixture twice without edits, then after an edit; compare
  pointer reuse, old-object immutability, and strictly newer VFS revision.
- Keep semantic-token taxonomy, positions, versions, fallback, full/delta,
  result IDs, transport, cancellation, and migration fixtures deferred.

## Compatibility impact

- Adds only internal tests; language semantics, diagnostics, schemas, Semantic
  IDs, CLI/LSP behavior, runtime, bytecode, VM, ABI, dependencies, and Unicode
  17.0.0 data remain unchanged.
- The fixtures use original UTF-8 spans and existing VFS revision rules and do
  not create a serialized fixture schema or public protocol inventory entry.

## Unresolved alternatives

Versioned semantic-token fixture schema, legend/modifier mapping, typed versus
parsed fallback, position encoding, document versions, full/delta equivalence,
result-ID/base handling, stale/cancellation/limits, protocol inventory, and
migration remain open under LSP-2401 through LSP-2404, LSP-2501/LSP-2502, and
the registered LSP/semantic gaps.

semantic-token fixture schema、legend/modifier 映射、typed 与 parsed fallback、位置编码、文档
版本、full/delta 等价性、result-ID/base 处理、stale/取消/限制、协议注册表和迁移仍由
LSP-2401 至 LSP-2404、LSP-2501/LSP-2502 及已登记 LSP/semantic 缺口决定。

## Supersession

- Supersedes: `None`
- Superseded by: `None`
