# DEC-0081: Internal structured diagnostic repair index / IDE 内部结构化诊断修复索引

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> Related RFC/gaps: `DEC-0001` | `DEC-0002` | `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes an immutable, deterministic internal index that copies
the existing diagnostic code, severity, primary source span, and structured
`Repair` payloads. It does not authorize a code action, `FixPlan`, edit,
action identifier, applicability policy, or protocol response.

本决定只授权建立不可变、确定性的内部索引，复制现有诊断的错误码、严重级别、
主源码跨度和结构化 `Repair` 载荷。不授权代码动作、`FixPlan`、编辑、动作标识符、
适用性策略或协议响应。

## Question

`IDE-2309` needs a structured source for future code-action analysis, but the
public action and edit lifecycle remains unspecified. Existing diagnostics
already carry registered codes, source spans, and structured repair facts. Those
facts can be indexed without parsing localized message text or creating an
unaccepted public action model.

`IDE-2309` 需要为未来代码动作分析提供结构化来源，但公开动作和编辑生命周期仍未
确定。现有诊断已经携带已注册错误码、源码跨度和结构化修复事实；可以在不解析本地化
消息文本、不创建未接受的公开动作模型的前提下索引这些事实。

## Decision

1. `ling-diagnostics` may expose `DiagnosticRepairIndex`, built from an
   immutable slice of existing `Diagnostic` values.
2. Each observation copies the diagnostic code, severity, optional primary
   span, repair ordinal, repair kind, `changes_semantics` flag, and structured
   fact map. Localized `message_zh` and `message_en` are never parsed.
3. The index provides read-only whole-entry, diagnostic-code, and repair-kind
   lookups. Empty diagnostics and diagnostics without repairs publish an empty
   index.
4. Construction sorts observations by registered code, severity, source span,
   repair kind, semantic-change flag, canonical fact-map JSON, and original
   repair ordinal. Repeated construction from equal input is equal and does
   not depend on host paths, allocation addresses, or map iteration order.
5. This decision does not define action IDs, titles, applicability, preferred
   or suppressed state, capabilities, `FixPlan`, edits, ranges, positions,
   snapshot/version binding, stale or rollback behavior, cancellation, limits,
   formatter or mutation policy, protocol schemas, or transaction semantics.
   The public `IDE-2309` target remains `BlockedSpec`.

## Conformance plan

- Verify code/severity/span and every structured repair field are retained,
  message-text changes do not affect the index, code/kind lookups are
  read-only, and empty/no-repair input stays empty.
- Verify repeated construction is equal and ordering remains stable for
  different input order, Unicode source names, BOM/CRLF spans, and BTreeMap
  fact order.
- Keep action IDs/applicability/preferred state, `FixPlan`, edit overlap and
  atomicity, positions, versions, stale/rollback, cancellation, limits,
  formatter/mutation safety, and protocol fixtures deferred to the blocked
  public `IDE-2309` contract.

## Compatibility impact

- Adds only internal diagnostic observation values and read-only getters.
  Language semantics, registered diagnostic codes and wire schema,
  diagnostics rendering, Semantic IDs, CLI/LSP behavior, runtime, bytecode,
  VM, ABI, and Unicode 17.0.0 tables remain unchanged.
- The index copies existing structured facts; it does not derive or publish a
  new public protocol identity.

## Unresolved alternatives

Code-action kind/ID/applicability/capability policy, `FixPlan` and edit
overlap/atomicity/version/stale/rollback/position behavior, cancellation and
resource limits, formatter and mutation safety, protocol lifecycle, and
migration remain open under `IDE-2309` and the registered LSP, formatter,
Unicode-alias, localization, and semantic lifecycle gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
