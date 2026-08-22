# DEC-0182: Internal loop/recursion checks boundary evidence / 内部循环与递归检查边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0181` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-OWNERSHIP-MODEL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`BND-5202-OBSERVATION`. It records provisional termination states, loop and
recursion proof vocabulary, resource/concurrency relations, work-queue action
boundaries, diagnostics, and fixtures while RFC-K504 and transaction/profile
authority remain unresolved.

本决定只授权 `BND-5202-OBSERVATION` 使用 test-local 的循环与递归检查边界清单；在 RFC-K504、
transaction、Profile 与 resource 权威尚未解决时，只记录临时的 termination state、loop/recursion
proof、resource/concurrency relation、work-queue action、diagnostic 与 fixture 词汇。

## Question

BND-5202 proposes `StaticallyBounded`, `ProvedTerminating`, `RuntimeGuarded`,
and `Forbidden/Unknown` states, plus an explicit work-queue code action. Which
vocabulary can be retained as bounded evidence without choosing a termination
calculus, proof trust boundary, runtime guard, or semantics-preserving rewrite?

## Decision

1. `crates/ling-types/tests/loop_recursion_checks_evidence.rs` keeps a
   test-local inventory of sixty provisional termination states, loop/recursion
   cases, proof/resource/concurrency relations, transformation obligations,
   diagnostics, and fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.loop-recursion-checks-observation/0`. These bytes
   are evidence only; they are not a checker, proof, runtime guard, code
   action, diagnostic, protocol, or support claim.
3. No termination checker, Bound state in Typed Core, runtime guard, work-queue
   transformer, diagnostic allocation, CLI/LSP action, dependency, protocol,
   support claim, or placeholder API is added. Public `BND-5202` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:156-167` is
  non-normative; it defines no termination logic, ranking functions, state
  transitions, runtime guard, or transaction semantics.
- `docs/ROADMAP-1.0.md:118` requires boundedness and reproducible evidence
  after concurrency/resources but does not authorize a termination checker or
  source transformation.
- `docs/status/BND-5202-AUTHORITY-AUDIT.md` records missing RFC-K504,
  dependent Critical/concurrency/resource authority, and the distinction
  between RFC-0015 VM frame limits and source termination proof.

## Conformance plan

- Assert all sixty loop/recursion boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer termination calculus, proof states, runtime guards, work-queue
  transformation, diagnostics, CLI/LSP/code actions, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing RFC-0015 frame/resource limits are not reinterpreted as
termination proof; only test-local evidence is added.

## Unresolved alternatives

Termination calculus for loops, direct/mutual/higher-order recursion and data
size; ranking/size-change, assumptions and proof trust; Effect/Fault/
concurrency/Task/Actor/mailbox/backpressure/cancellation/ordering/Device/
Native/numeric/fallback relations; stack/heap/arena and runtime guards;
Forbidden/Unknown states and profile policy; work-queue eligibility and state/
ownership/effect/ordering/allocation/cancellation/Fault/source-map equivalence;
user consent, rollback and output; bilingual diagnostics and provenance;
positive/negative/counterexample/transformation/Unicode/determinism/
differential fixtures; protocol inventory and public status remain open under
BND-5202, BND-5201, GAP-CRITICAL-PROFILE-001,
GAP-ACTOR-MAILBOX-SUPERVISOR-001, GAP-OWNERSHIP-MODEL-001, and missing RFC-K504
and transaction authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
