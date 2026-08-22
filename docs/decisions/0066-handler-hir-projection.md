# DEC-0066: Handler unresolved HIR projection / Handler 未解析 HIR 投影

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: effect-system-design
> 相关 RFC/缺口：`RFC-0006`, `DEC-0063`, `DEC-0064`, `DEC-0065`, `GAP-EFFECT-HANDLER-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes the next bounded EFF-2103 child: a data-only HIR
projection for the unresolved handler AST. It extends DEC-0065 only at the
AST-to-HIR boundary; it does not define handler semantics or publish checked
HIR.

本决定授权 EFF-2103 的下一有界子任务：将未解析 Handler AST 投影为纯数据 HIR。
它只扩展 DEC-0065 的 AST 到 HIR 边界，不定义 Handler 语义，也不发布已检查 HIR。

## Question

DEC-0065 requires AST lowering to stop before HIR because operation lookup,
binding identity, resume typing, and effect checking are unresolved. How can
the compiler preserve the handler shape through the HIR boundary without
allowing an unchecked handler to enter resolution, typing, effects, or
execution?

DEC-0065 要求在 HIR 前停止 AST 降级，因为 operation 查找、绑定身份、resume 类型和
Effect 检查尚未确定。如何在不让未检查 Handler 进入解析、类型、Effect 或执行阶段的
前提下保留其 HIR 形状？

## Decision

1. **HIR data shape.** Add `hir::ExpressionKind::Handle` containing a lowered
   body and source-order `hir::HandlerClause` values. Each clause retains its
   span, lowered `QualifiedName` operation, lowered parameter `Pattern`s,
   optional resume `Name`, and lowered body. Existing expression, pattern,
   binding, and source spans remain the only identity domains.

2. **Unresolved lowering.** HIR lowering MUST preserve clause order and
   original spans while allocating ordinary local HIR IDs. It MUST NOT resolve
   the operation, create a reference for the operation or resume marker, infer
   a resume cardinality, establish handler scope, or construct a checked
   Effect row.

3. **Resolution gate.** The resolver MUST reject every unresolved HIR handler
   with the structured `UnsupportedHandler` error and registered bilingual
   diagnostic `L-EFFECT-0004`. It MUST publish no handler references or checked
   program when this error is present.

4. **Downstream boundary.** Type inference, Effect inference, Semantic Graph,
   evaluator, bytecode lowering, and VM execution MUST not interpret this
   unresolved variant. Direct entry points must reject it with their existing
   structured invalid-input boundary or remain unreachable from a successful
   resolver result. No handler runtime behavior is authorized.

5. **Compatibility.** Handler spellings remain contextual and ordinary Seed
   identifiers are unchanged. No operation namespace, signature, return or
   recovery clause, nested propagation, State/Fault/cancellation interaction,
   Task/Actor crossing, public protocol, schema, or migration is defined.

## Conformance plan

- Lower one and multiple clauses with nested bodies, Unicode names, patterns,
  optional resume names, BOM, and CRLF; compare exact HIR data, source order,
  IDs, and original spans.
- Verify operation and resume names have no resolver references and that the
  resolver returns `L-EFFECT-0004` without publishing a `ResolvedProgram`.
- Exercise direct downstream boundary calls with unresolved HIR and verify
  they reject or are not reachable; ordinary Seed programs remain unchanged.
- Repeat lowering in fresh processes and compare deterministic HIR output
  without host paths, allocation order, map order, or debug formatting.

## Compatibility impact

- Adds an Experimental internal unresolved HIR variant and one registered
  bilingual compiler diagnostic for the explicit resolution gate.
- Non-handler Seed source, existing diagnostics, Semantic IDs, schemas, CLI,
  LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0 behavior are unchanged.
- Checked operation semantics and execution remain deferred to later Accepted
  authorities.

## Unresolved alternatives

- Operation signature/namespace rules, parameter and resume binding identity,
  return/recovery clauses, Effect-row checking, nested propagation,
  State/Fault/cancellation, Task/Actor crossing, and runtime lowering remain
  open.
- A later Accepted authority may replace this data shape only with migration,
  span, determinism, and executable corpus evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

