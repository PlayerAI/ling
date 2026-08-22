# DEC-0067: Effect model deterministic property corpus / Effect 模型确定性性质语料

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: effect-system-design  
> 相关 RFC/缺口：`RFC-0006`, `DEC-0062`, `GAP-EFFECT-HANDLER-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a bounded evidence child of `EFF-2105`. It freezes
only deterministic in-process properties of the accepted RFC-0006/DEC-0062
Effect model. It does not authorize a source generator, handler execution,
interpreter/VM equivalence, or a public fuzzing compatibility gate.

本决定授权 `EFF-2105` 的有界证据子任务，仅固定已接受的 RFC-0006/DEC-0062
Effect 模型的进程内确定性性质。不授权源码生成器、Handler 执行、解释器/VM
等价性或公共 fuzz 兼容门禁。

## Question

The full EFF-2105 property target depends on handler runtime and differential
contracts that are still absent. Which evidence can be added now without
inventing those semantics or interpreting unresolved AST/HIR?

完整 EFF-2105 依赖尚未定义的 Handler runtime 和 differential 合约。当前在不
发明这些语义、也不解释未解析 AST/HIR 的前提下，可以先加入哪些证据？

## Decision

1. **Bounded domain.** The child covers only `EffectId`, `EffectTypeRef`,
   `EffectLabel`, `EffectRowModel`, `EffectOperation`, `HandlerContract`,
   `HandlerCore`, and `EffectConstraintSolver`, using at most four labels,
   three row variables, and three handler clauses per generated case.

2. **Deterministic enumeration.** The corpus uses fixed finite permutations and
   hand-built canonical values. It MUST not depend on host randomness, wall
   clock, filesystem order, thread scheduling, allocation order, or external
   network state. Every case has a stable name and bounded execution cost.

3. **Properties.** The corpus MUST prove that presentation order and duplicate
   inputs do not change canonical rows, graph/core bytes, operation ordering,
   handler residual subtraction, or solver substitutions. It MUST also cover
   open-tail preservation, nested residual subtraction, and declared resume
   cardinality rejection using the existing checked model APIs.

4. **Negative boundary.** The child MUST NOT generate source programs, create
   unresolved or fabricated Typed-Core identities, execute handlers, compare
   interpreter and VM results, model Fault/cancellation/State masking, or
   publish a fuzz protocol, schema, diagnostic, Semantic ID, CLI, LSP, or
   bytecode surface.

5. **Evidence boundary.** Source spans may be attached only to diagnostics and
   MUST remain absent from canonical bytes. The corpus is compatibility evidence
   for the accepted model, not an implementation of the parent EFF-2105 task.

## Conformance plan

- Enumerate all bounded permutations of a fixed label set and compare canonical
  names and bytes against sorted/deduplicated baselines.
- Enumerate equal-row constraints in different insertion orders and compare
  substitutions, normalized rows, and conflict projections.
- Construct nested first-order `HandlerCore` values and compare residual rows,
  canonical bytes, and resume-mode rejection across clause permutations.
- Repeat the corpus in offline test processes and keep every case below the
  declared bounds; verify no test reaches evaluator, bytecode, VM, or LSP code.

## Compatibility impact

- v0.0.1 Seed source, accepted v0.2 model semantics, diagnostics, schemas,
  Semantic IDs, CLI, LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0:
  unchanged.
- Adds only offline test evidence and this internal decision/status metadata.
- No public protocol, runtime behavior, release claim, or Stable 1.0 support is
  introduced.

## Unresolved alternatives

- Well-typed source/Core generation, shrinking, handler execution, residual-row
  observations, Fault/cancellation behavior, and interpreter/VM differential
  equivalence remain in the EFF-2105 parent and require accepted runtime and
  generator authorities.
- A future property target may supersede this bounded corpus only with explicit
  seed/reproducibility, resource-limit, migration, and differential evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
