# DEC-0065: Handler AST projection / Handler AST 投影

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: effect-system-design
> 相关 RFC/缺口：`RFC-0006`, `DEC-0063`, `DEC-0064`, `GAP-EFFECT-HANDLER-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes the next bounded EFF-2103 child: lowering the
DEC-0064 handler CST into an explicit, span-preserving AST value. It does not
authorize HIR resolution, type/effect checking, evaluation, bytecode, VM, or a
public protocol.

本决定授权 EFF-2103 的下一有界子任务：将 DEC-0064 Handler CST 降级为显式、
保留 span 的 AST 值。它不授权 HIR 解析、类型/Effect 检查、求值、字节码、VM
或公共协议。

## Question

DEC-0064 fixes the lossless parser shape but currently requires the AST lowerer
to reject it. What data-only AST representation can preserve operation names,
parameter patterns, the contextual resume marker, clause bodies, and original
spans while guaranteeing that later unchecked compiler stages cannot consume
it?

DEC-0064 固定了无损 parser 形状，但目前要求 AST lowerer 拒绝它。需要什么纯数据
AST 表示保留 operation 名称、参数 pattern、上下文 resume 标记、clause body 和
原始 span，同时确保后续未检查编译阶段不能消费它？

## Decision

1. **AST data shape.** Add `ExpressionKind::Handle` containing a body
   expression and source-order `HandlerClause` values. Each clause contains
   its span, a `QualifiedName` operation, zero or more parameter `Pattern`s,
   an optional `Name` for the contextual `resume` marker, and its body
   expression.

2. **Lossless lowering.** The AST lowerer MUST retain each original UTF-8
   source span and source spelling. It MUST reject malformed child shapes,
   duplicate/missing required children, and a missing operation clause with the
   existing structured `LowerError` variants. Clause order is source order;
   no operation is resolved or reordered.

3. **Checked-only boundary.** The AST value is unresolved data. HIR lowering
   MUST reject `ExpressionKind::Handle` with a structured unsupported-handler
   error until an Accepted decision defines operation signatures, binding
   identity, resume typing/cardinality, return/recovery clauses, effect-row
   checking, and checked Typed Core publication. No evaluator, bytecode, VM,
   Semantic Graph, or protocol may consume this AST value.

4. **Compatibility.** The CST remains contextual, so Seed identifiers are not
   globally reserved. Existing AST/HIR values and all Seed behavior are
   unchanged for non-handler source. Canonical identity and determinism do not
   include host paths, allocation order, map order, or debug formatting.

## Conformance plan

- Lower one and multiple operation clauses, parameter patterns, the optional
  resume marker, nested match/handler bodies, Unicode names, BOM, and CRLF;
  assert exact AST fields, source order, source spelling, and original spans.
- Reject malformed CST child shapes and verify the AST lowerer never fabricates
  an unresolved operation or resume binding.
- Verify HIR lowering rejects the AST handler with a structured error and that
  no evaluator, bytecode, VM, schema, Semantic ID, or protocol path consumes it.
- Compare repeated lowering of equivalent source bytes for deterministic data
  and preserve ordinary Seed identifiers named `handle`, `operation`, and
  `resume`.

## Compatibility impact

- Seed source, diagnostics, Semantic IDs, schemas, CLI, LSP, bytecode, VM,
  protocols, ABI, and Unicode 17.0.0 behavior: None for non-handler source.
- Adds an Experimental unresolved AST data variant and a structured internal
  HIR rejection; no public protocol or diagnostic allocation is added.
- Operation resolution, checking, checked Core construction, execution, and
  migration remain deferred to later Accepted authorities.

## Unresolved alternatives

- The exact operation signature and namespace rules, argument/resume binding
  identity, return/recovery clauses, nested propagation, State/Fault/
  cancellation interaction, and Task/Actor crossing remain open.
- A later Accepted authority may supersede this AST shape only with migration,
  span, determinism, and executable corpus evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

