# DEC-0063: First-order handler Typed Core projection / 一阶 Handler Typed Core 投影

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: effect-system-design  
> 相关 RFC/缺口：`RFC-0006`, `DEC-0062`, `GAP-EFFECT-HANDLER-001`, `GAP-EFFECT-STATE-MASKING-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes the bounded in-process first-order handler Core
projection used by the EFF-2103 child slice. It is a checked-data boundary,
not a source-language grammar, evaluator, bytecode instruction, or public
protocol. Source parsing and full lowering remain with the EFF-2103 parent.

本决定授权 EFF-2103 子切片使用的进程内一阶 Handler Core 投影。它是 checked
data 边界，不是源代码语法、求值器、字节码指令或公共协议。源代码解析和完整
lowering 仍属于 EFF-2103 父任务。

## Question

RFC-0006 and DEC-0062 define canonical rows, operations, residual subtraction,
resume cardinality, and deterministic inference, but leave the checked Core
container and its resume-use invariant open. What minimal representation can be
published without inventing source syntax or runtime semantics?

RFC-0006 与 DEC-0062 已定义 canonical row、operation、residual subtraction、
resume cardinality 和确定性推导，但尚未规定 checked Core 容器及 resume-use
不变量。本决定规定一个最小表示，同时不发明源语法或运行时语义。

## Decision

1. **Checked-only container.** `HandlerCore` contains an input
   `EffectRowModel`, an accepted `HandlerContract`, a canonical body node ID,
   a canonical return type reference, and the original UTF-8 source span as
   evidence. The body node ID is an opaque checked-core identity supplied by a
   caller; the projection never interprets an unresolved AST node.

2. **Clause projection.** Each `HandlerCoreClause` contains the existing
   `HandlerClause`, a canonical clause body node ID, and a declared
   `ResumeUse` of `Never`, `Once`, or `Many`. The clause owner and operation
   signature must match RFC-0006. Clause labels are sorted and duplicate-free
   through `HandlerContract`.

3. **Resume invariant.** `ResumeUse::Never` is valid only for a `Never` or
   non-resuming operation; `Once` permits at most one resume; `Many` permits
   any declared use. A Core constructor rejects a use that exceeds the
   operation's declared `ResumeMode`. It does not count or execute resumes.

4. **Residual and nesting.** The Core constructor computes the residual row by
   applying the lexical contract to the input row. Nested handlers consume the
   inner residual before an outer handler is constructed. The input tail is
   preserved. A caller that requires a closed boundary may call
   `require_closed`; a non-pure residual produces the registered
   `L-EFFECT-0003` unhandled-residual diagnostic.

5. **State and Capability.** Only an exact explicitly declared operation can
   remove a matching `State<T>` label. The Core projection never masks State,
   creates or removes Capability facts, handles Faults, or crosses Task/Actor,
   Replay, Remote, Native, GPU, or FFI boundaries. Those behaviors require
   their own accepted authorities.

6. **Scope and identity.** Scope is lexical and first-order. Body IDs,
   operation labels, rows, return types, and clause order are canonical and
   path-free. Original spans are retained only as evidence and never enter
   canonical bytes, Semantic IDs, or a public schema.

7. **No fallback or execution.** An unhandled residual is a checked error at an
   explicitly requested closed boundary. The Core projection does not provide
   dynamic fallback, continuation storage, handler execution, interpreter/VM
   ABI, source syntax, CLI, LSP, or wire behavior.

## Conformance plan

- Construct single and nested first-order handler Core values with pure,
  closed, open, Clock, Random, and parameterized State rows.
- Verify operation-owner matching, duplicate clause rejection, residual-row
  computation, open-tail preservation, canonical clause ordering, and
  deterministic canonical bytes under insertion and source-name changes.
- Verify Never/Once/Many resume-use boundaries and reject over-resumption.
- Verify `require_closed` produces bilingual `L-EFFECT-0003` facts with the
  original UTF-8 span, while exact State handling never changes Capability
  facts.
- Verify invalid/unresolved body IDs are rejected before publication and no
  evaluator, bytecode, VM, protocol, or Semantic Graph field is created.

## Compatibility impact

- Seed source syntax, existing checker behavior, Semantic IDs, schemas, CLI,
  LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0 data: None.
- Adds an Experimental v0.2 in-process Core value and one registered diagnostic
  code used only when a caller explicitly requests a closed residual boundary.
- Source syntax, complete parser/lowering integration, Audit Source expansion,
  runtime execution, and Task/Actor crossing remain deferred to later accepted
  authorities.

## Unresolved alternatives

- Exact source grammar and AST/HIR lowering for `handle`/`with operation` remain
  in the EFF-2103 parent task and require a later accepted decision.
- Resume continuation storage, runtime Fault/cancellation interaction,
  interpreter/VM ABI, Audit Source/Semantic Graph schema, and public protocol
  migration remain EFF-2104/2105 or separate authorities.
- A future authority may add non-first-order handlers only with a superseding
  accepted decision, migration evidence, and differential fixtures.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
