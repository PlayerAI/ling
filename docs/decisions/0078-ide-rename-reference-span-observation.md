# DEC-0078: Internal rename reference-span observation / IDE 内部重命名引用范围观察

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0019` | `GAP-LSP-TRANSACTION-PROTOCOL-001` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process observation that pairs existing
resolver reference identities with the exact original UTF-8 identifier spans
already retained by HIR. It does not authorize rename target selection,
position conversion, edits, snapshot mutation, or an LSP response.

本决定只授权在进程内将现有 resolver 引用身份与 HIR 已保留的原始 UTF-8 标识符范围配对，
不授权重命名目标选择、位置转换、编辑、快照修改或 LSP 响应。

## Question

`IDE-2306` needs exact source spans when collecting the references affected by a
future identity-based rename. The existing forward and reverse reference
indexes intentionally omit source ranges. HIR already carries the precise
name, projection-field, and mutable-place-root spans, so a separate read-only
projection can supply this evidence without inventing an edit protocol.

## Decision

1. `ling-db` may expose `ResolvedReferenceSpanIndex`, an immutable index built
   from the validated `ResolvedProgram` modules and their HIR expressions.
2. For a `Name` reference, the observation uses the HIR `Name.span`; for a
   projection reference it uses the field name span; for an assignment place it
   uses the place root name span. Only reference identities present in the
   resolver map are published; orphan HIR IDs are omitted.
3. Each entry retains logical source name, resolver module/reference IDs, and
   the original `SourceId + Span`. It does not normalize, re-hash, or convert
   the span to an editor position.
4. Entries are sorted by logical source name, module ID, reference ID, and span
   bytes. Repeated equal resolver/HIR input therefore produces equal output
   independent of traversal, map, allocation, host-path, or locale behavior.
5. The observation has no relation taxonomy, target eligibility, alias policy,
   keyword/confusable decision, URI/version, snapshot, stale check, edit,
   cancellation, persistence, cache, JSON, or JSON-RPC state. The public
   `IDE-2306` rename task remains `BlockedSpec`.

## Conformance plan

- Verify exact name spans for Unicode, BOM, and CRLF source; include ordinary
  names, local bindings, projections, and mutable-place roots where accepted
  Seed syntax supplies them.
- Repeat construction and compare source-scoped lookup and full equality; omit
  HIR references absent from resolver identity and never fabricate a span.
- Keep target selection, reference/alias policy, Unicode name acceptance,
  temporary snapshots, type/effect/behavior checks, identity migration,
  Workspace Edit, position/version, stale, rollback, and protocol fixtures
  deferred.

## Compatibility impact

- Adds only internal `ling-db` observation values and a read-only accessor;
  `DEC-0002` source-span truth is reused without changing language semantics,
  diagnostics, schemas, Semantic IDs, CLI output, LSP wire behavior, runtime,
  bytecode, VM, ABI, or Unicode 17.0.0 tables.
- The index is evidence for future rename analysis, not a source edit or a
  public range contract. No protocol-inventory entry or Stable 1.0 rename
  claim is introduced.

## Unresolved alternatives

Rename target and relation policy, declaration/import-alias handling, generated
and dependency mutability, keyword/confusable/collision rules, visibility and
coherence, temporary snapshot application, DefinitionId migration, atomic
multi-file edits, URI/version and position encoding, cancellation, stale
publication, rollback, protocol negotiation, and Semantic Graph lifecycle
remain open under `IDE-2305`, `IDE-2306`, `DEC-0012`,
`GAP-UNICODE-ALIAS-SYNTAX-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
