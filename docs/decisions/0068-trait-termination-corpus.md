# DEC-0068: Trait solver bounded termination corpus / Trait Solver 有界终止语料

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: type-system-design  
> 相关 RFC/缺口：`RFC-0005`, `DEC-0026`, `GAP-TRAIT-COHERENCE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a bounded evidence child of `TRAIT-1309`. It freezes
only the existing RFC-0005/DEC-0026 termination boundary and source-evidence
independence of the crate-private solver. It does not define a benchmark
protocol, wall-clock budget, LSP cancellation, or public Trait service.

本决定授权 `TRAIT-1309` 的有界证据子任务，仅固定现有 RFC-0005/DEC-0026
终止边界以及 crate-private solver 对源码证据的独立性。不定义 benchmark 协议、
墙钟预算、LSP 取消或公共 Trait 服务。

## Question

The full TRAIT-1309 target requires a production obligation graph, explainable
resource budget, and editor cancellation contract that are not accepted. What
deterministic evidence can be added without changing the accepted 64-level
semantic limit or claiming host-performance guarantees?

## Decision

1. **Bounded domain.** The child exercises only the existing internal
   `solve_obligations` boundary with the accepted concrete candidate, cycle, and
   64-level depth-limit fixtures. It uses finite source fixtures and does not
   attach the solver to a new production query or protocol.

2. **Source-evidence independence.** Equivalent source bytes under different
   logical source names MUST produce the same selected Trait/impl projection.
   Source names and spans remain diagnostic evidence and MUST NOT select an
   implementation or alter the termination result.

3. **Termination boundary.** The corpus MUST preserve the RFC-0005 64-level
   nesting limit and active-obligation cycle rejection. It MUST NOT change the
   limit, add a wall-clock/allocation/candidate budget, or infer cancellation
   precedence.

4. **Negative boundary.** No benchmark output schema, timing threshold, LSP
   request, public diagnostic, CLI command, Semantic ID, or Stable 1.0 Trait
   claim is added.

## Conformance plan

- Run the same concrete Trait fixture under distinct source names and compare
  only the deterministic selected Trait/impl/member projection.
- Keep the existing active-cycle and depth-64 negative fixtures as the semantic
  termination evidence; verify no test changes their error category or limit.
- Run the corpus offline with stable finite input and no timing or host-state
  assertion.

## Compatibility impact

- RFC-0005 Trait semantics, Seed source behavior, diagnostics, schemas, Semantic
  IDs, CLI, LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0: unchanged.
- Adds only one offline internal test and governance/status evidence.
- No public performance or cancellation contract is introduced.

## Unresolved alternatives

- Production HIR/Typed Core obligation integration, deep-chain/diamond/failure/
  cross-package benchmark corpus, deterministic resource budgets, LSP
  cancellation, and public evidence formats remain in the TRAIT-1309 parent.
- A future benchmark decision may extend this child only with explicit metric,
  variance, environment, migration, and protocol authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
