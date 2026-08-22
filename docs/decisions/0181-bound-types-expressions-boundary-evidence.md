# DEC-0181: Internal Bound types/expressions boundary evidence / 内部 Bound 类型与表达式边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0180` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`BND-5201-OBSERVATION`. It records provisional Bound syntax, domains, units,
resource relationships, proof states, diagnostics, and fixture vocabulary while
RFC-K504 and the dependent Critical/resource authorities remain unresolved.

本决定只授权 `BND-5201-OBSERVATION` 使用 test-local 的 Bound 边界清单；在 RFC-K504 与 Critical、
resource、ownership、concurrency、Kernel、Native 权威尚未解决时，只记录临时的 Bound 语法、
domain、unit、resource relation、proof state、diagnostic 与 fixture 词汇。

## Question

BND-5201 proposes compile-time constants, Profile parameters, range types,
collection capacities, loop/recursion bounds, Task/Actor counts, stack/arena
budgets, and message sizes. Which vocabulary can be retained as bounded
evidence without choosing grammar, units, arithmetic, proof states, or resource
semantics?

## Decision

1. `crates/ling-types/tests/bound_types_expressions_evidence.rs` keeps a
   test-local inventory of sixty provisional Bound/type/expression categories,
   checked-core boundaries, resource relations, proof states, diagnostics, and
   fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.bound-types-expressions-observation/0`. These
   bytes are evidence only; they are not syntax, types, a solver, proof,
   resource budget, diagnostic, protocol, or support claim.
3. No Bound AST/HIR/Typed-Core node, constraint solver, profile parameter API,
   diagnostic allocation, CLI option, dependency, protocol, support claim, or
   placeholder API is added. Public `BND-5201` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:138-155` is
  non-normative; it defines no grammar, type rules, units, constant evaluation,
  symbolic relations, overflow, profile limits, or evidence state.
- `docs/ROADMAP-1.0.md:118` requires boundedness and reproducible evidence
  but does not authorize a Bound language feature.
- `docs/status/BND-5201-AUTHORITY-AUDIT.md` records missing RFC-K504 and open
  Critical, ownership, concurrency, Kernel/Native, numeric, and effect gaps.
- Existing parser/type/bytecode limits and solver recursion guards remain
  implementation safety only; they are not source-level Bound semantics.

## Conformance plan

- Assert all sixty Bound/type/expression boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer grammar, type/constraint rules, units, arithmetic, proof/runtime
  states, resource integration, diagnostics, CLI, and protocol behavior until
  accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing internal parser/type/bytecode safety limits are not
reinterpreted as Ling Bound semantics; only test-local evidence is added.

## Unresolved alternatives

Bound grammar and checked representation; constant/Profile parameter scope;
range/capacity types; units, domains, variance and Nat/Int arithmetic;
overflow/underflow, unknown/symbolic values, canonical bytes; collection,
loop, recursion, Task/Actor, stack/arena, message and Device soundness;
ownership/effect/capability/scheduler/cancellation/Fault/fallback relations;
proof, runtime-guarded, forbidden and assumed states; profile/target limits;
bilingual diagnostics and source facts; positive/negative/arithmetic/symbolic/
Unicode/differential fixtures; protocol inventory and public status remain open
under BND-5201, PROF-5104, GAP-CRITICAL-PROFILE-001,
GAP-ACTOR-MAILBOX-SUPERVISOR-001, GAP-OWNERSHIP-MODEL-001,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing RFC-K504
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
