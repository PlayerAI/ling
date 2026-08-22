# DEC-0126: Internal Managed collector boundary evidence / 内部 Managed Collector 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: managed-runtime
> 相关规范/缺口：`DEC-0125` | `DEC-0124` | `DEC-0121` | `DEC-0094` | `DEC-0013` | `ROADMAP-1.0` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed contracts
that a future Managed collector must honor for the bounded
`GC-3302-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not select a collector, implement a heap, define pause or
safe-point behavior, expose roots or metrics, classify OOM, or define Task,
Actor, Profile, or runtime semantics.

本决定只授权 test-only 的拟议 Managed collector 契约清单，供
`GC-3302-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不选择 collector、不实现 heap、
不定义 pause 或 safe-point 行为、不暴露 root 或 metrics、不分类 OOM，也不定义 Task、Actor、Profile 或运行时语义。

## Question

GC-3302 asks for a first collector choice, pause behavior, root registration,
cycle handling, safe points, Task/Actor interaction, memory limits, metrics,
and stress/fuzz evidence. Which boundary vocabulary can be preserved for
planning without making an implementation strategy or host behavior into Ling
semantics?

GC-3302 要求第一版 collector 选择、pause 行为、root registration、cycle handling、safe point、Task/Actor 交互、
memory limit、metrics 以及 stress/fuzz 证据。哪些边界词汇可以作为规划证据保留，而不会把实现策略或宿主行为变成 Ling 语义？

## Decision

1. `crates/ling-concurrency/tests/managed_collector_evidence.rs` keeps a
   test-local inventory of forty-three provisional boundaries: collector and
   heap choice, root registration/lifetime across stack, closure, global,
   Task, Actor, callback, Native Island, and suspension contexts, reachability
   and cycles, barriers and ordering, safe points and pauses, progress and
   attachment, Task/Actor cancellation/restart/shutdown, memory limits and
   allocation failure, OOM/recovery, metrics, stress/property/fuzz evidence,
   deterministic bounds, differential evidence, Unicode spans, host opacity,
   Profiles, and Resource Drop separation.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.managed-collector-observation/0`. These bytes are not a collector,
   heap, root registry, pause trace, safe-point protocol, scheduler hook,
   allocation limit, OOM Fault, metrics schema, stress oracle, fuzz target,
   public protocol, or runtime contract.
3. The child adds no collector algorithm, Managed heap, scheduler hook, root
   registry, memory-limit API, metrics schema, OOM diagnostic, public protocol,
   or placeholder G3 API. Public `GC-3302` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative and cannot select observable
  collector behavior or authorize a public collector API.
- Accepted `DEC-0125` records only Managed object-model vocabulary;
  `DEC-0124` and `DEC-0121` do not resolve ownership or suspension semantics.
- Accepted `DEC-0094` provides only bounded internal scheduler observation, and
  `DEC-0013` preserves compile/host/internal/runtime-fault separation; neither
  defines a Managed heap, OOM Fault, or public metrics protocol.
- `GAP-OWNERSHIP-MODEL-001`, `GAP-STRUCTURED-TASK-001`, and
  `GAP-ACTOR-AWAIT-REENTRY-001` remain Open. RFC-N303/RFC-0007 are not
  Accepted; this decision records vocabulary without resolving those gaps.

## Conformance plan

- Assert all forty-three provisional collector boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep collector selection, heap/root behavior, pauses, safe points, cycles,
  memory limits, OOM classification, metrics, stress/fuzz semantics,
  Task/Actor behavior, Profile/FFI rules, diagnostics, and differential
  contracts deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No collector diagnostic, metrics
  schema, Semantic ID, or public protocol claim is registered.

## Unresolved alternatives

Collector algorithm and pause model, root registration and safe-point
placement, cycle/barrier ordering, Task/Actor cancellation and restart,
memory-limit/OOM recovery, metrics exposure and bounds, stress/fuzz oracle
semantics, Profile/FFI behavior, diagnostics, migration, and
interpreter/VM/Native differential semantics remain open under `GC-3302`,
`GC-3301`, `GAP-OWNERSHIP-MODEL-001`, `GAP-STRUCTURED-TASK-001`,
`GAP-ACTOR-AWAIT-REENTRY-001`, and missing RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
