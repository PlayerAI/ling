# DEC-0183: Internal memory-budget boundary evidence / 内存预算边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0182` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`BND-5203-OBSERVATION`. It records provisional memory-budget vocabulary for
allocation, layout, lifetime, queues/tasks/devices, proof states, target
binding, fallback boundaries, diagnostics, and fixtures while RFC-K504 and the
dependent ownership, concurrency, Kernel/Device, and Native/ABI authorities
remain unresolved.

本决定只授权 `BND-5203-OBSERVATION` 使用 test-local 的内存预算边界清单；在 RFC-K504 与
ownership、concurrency、Kernel/Device、Native/ABI 等依赖权威尚未解决时，只记录临时的
allocation、layout、lifetime、queue/task/device、proof、target binding、fallback、diagnostic 与
fixture 词汇。

## Question

BND-5203 proposes static-data, stack, arena/buffer, queue/mailbox, task/actor,
device, transient-peak, and error/fallback memory accounting bound to a target
ABI and compiler version. Which vocabulary can be retained as bounded evidence
without choosing units, ownership, layout, lifetime, aliasing, proof states, or
host-versus-logical budget semantics?

## Decision

1. `crates/ling-types/tests/memory_budgets_evidence.rs` keeps a test-local
   inventory of sixty provisional memory-budget categories, allocation and
   lifetime boundaries, queue/task/device relations, proof and target states,
   fallback boundaries, diagnostics, and fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.memory-budgets-observation/0`. These bytes are
   evidence only; they are not a memory model, analyzer, proof, runtime guard,
   target contract, diagnostic, protocol, or support claim.
3. No analyzer, allocation/ownership model, target binding, diagnostic
   allocation, CLI/LSP action, dependency, protocol, support claim, or
   placeholder API is added. Public `BND-5203` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:169-182` is non-normative;
  it defines no byte/unit model, allocation or ownership rules, layout,
  lifetime/alias treatment, peak/path semantics, proof state, target binding,
  or failure behavior.
- `docs/ROADMAP-1.0.md:118` and `:433-498` place Critical bounded allocation
  and reproducible evidence after the required concurrency, resource/Native,
  and restricted-lowering gates; they do not authorize a checker.
- `docs/status/BND-5203-AUTHORITY-AUDIT.md` records missing RFC-K504 and the
  dependent ownership, mailbox, Kernel/Device, Native/ABI, and profile
  authorities.
- RFC-0014 and RFC-0015 host/input limits remain safety boundaries. They are
  not a common source memory model, logical budget proof, or target guarantee.

## Conformance plan

- Assert all sixty memory-budget boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer memory accounting, proof/estimate states, target binding, runtime
  guards, fallback semantics, diagnostics, CLI/LSP, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing VM and decoder limits are not reinterpreted as Ling memory
semantics; only test-local evidence is added.

## Unresolved alternatives

Memory units and object/layout rules; allocation, ownership, regions,
borrowing, sharing, aliasing, alignment and drop timing; Value/Managed/
Resource/Device memory; static/stack/arena/buffer accounting; transient peaks,
control-flow joins, recursion, cancellation and worst-case paths;
queue/mailbox/task/actor/backpressure/device placement; proof, estimate,
assumption, unknown, overflow, unsupported and target-mismatch states; target
ABI/compiler identity, migration, cache/replay identity; runtime guards,
out-of-memory, queue overflow, device allocation failure and fallback policy;
logical versus host-safety guarantees; bilingual diagnostics and source facts;
positive/negative/boundary/ownership/queue-target/determinism/differential
fixtures; protocol inventory and public status remain open under BND-5203,
GAP-CRITICAL-PROFILE-001, GAP-OWNERSHIP-MODEL-001,
GAP-ACTOR-MAILBOX-SUPERVISOR-001, GAP-NATIVE-BACKEND-ABI-001,
GAP-KERNEL-DEVICE-001, and missing RFC-K504 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
