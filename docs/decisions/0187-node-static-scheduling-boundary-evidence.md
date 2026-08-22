# DEC-0187: Internal Node static-scheduling boundary evidence / Node 静态调度边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0186` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-STRUCTURED-TASK-001` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-DETERMINISTIC-REPLAY-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5303-OBSERVATION`. It records provisional static-scheduling vocabulary
for graph order, rates/clocks, multi-rate bridges, release/deadline, priority,
overrun, target evidence, manifests, diagnostics, and fixtures while RFC-K502
and the dependent Node/concurrency/replay authorities remain unresolved.

本决定只授权 `NODE-5303-OBSERVATION` 使用 test-local 的静态调度边界清单；在 RFC-K502 与 Node、
concurrency、replay、Critical 等依赖权威尚未解决时，只记录临时的 graph order、rate/clock、
multi-rate bridge、release/deadline、priority、overrun、target evidence、manifest、diagnostic 与
fixture 词汇。

## Question

NODE-5303 proposes a topological or legally cyclic schedule with dependency
analysis, rate/clock bridges, priority/period, release/deadline, overrun
policy, and a scheduler manifest. Which vocabulary can be retained as bounded
evidence without choosing graph ordering, bridge state, schedulability/WCET,
runtime Fault, replay, or manifest semantics?

## Decision

1. `crates/ling-types/tests/node_static_scheduling_evidence.rs` keeps a
   test-local inventory of sixty provisional scheduling categories, graph/rate/
   bridge/release/deadline relations, target/replay evidence, diagnostics, and
   fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-static-scheduling-observation/0`. These bytes
   are evidence only; they are not a scheduler, schedulability proof, bridge,
   manifest, runtime, diagnostic, protocol, or support claim.
3. No Node scheduler, multi-rate bridge, WCET engine, manifest schema,
   dependency, diagnostic allocation, CLI/LSP route, protocol, support claim,
   or placeholder API is added. Public `NODE-5303` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:242-252` is
  non-normative; it defines no graph identity/order, clock/rate units, bridge,
  priority/release/deadline, overrun, or manifest compatibility rules.
- `docs/SEMANTICS.md:1380-1425` is a conceptual Node outline and
  `:1914-1931` reserves Node; it does not authorize a scheduler.
- `docs/decisions/0019-incremental-query-boundary.md:39-49` covers only an
  internal deterministic compiler-query scheduler, not Node runtime behavior.
- `docs/status/NODE-5303-AUTHORITY-AUDIT.md` records missing RFC-K502,
  schedulability, target, concurrency, replay, and manifest authority.

## Conformance plan

- Assert all sixty Node static-scheduling boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer graph/schedule analysis, bridges, schedulability/WCET, manifest,
  diagnostics, CLI/LSP, and runtime protocol behavior until accepted authority
  exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Internal query scheduling and VM limits are not reinterpreted as
Node scheduling or real-time guarantees; only test-local evidence is added.

## Unresolved alternatives

Graph/node/edge identity and topological/legal-cycle order; cycle breaks;
clock/rate/period/phase; multi-rate bridge interpolation/decimation,
buffering/backpressure/loss/state ownership; priority, release/deadline,
jitter, preemption/cooperative execution, admission/schedulability/WCET;
target/compiler, interrupt/cache/bus assumptions; release/overrun, Fault,
cancellation, restart/recovery/fallback; replay/determinism; manifest version/
migration; Critical Profile, Task/Actor, Kernel/Device, memory/queue bounds;
unknown schedulability, unsupported bridge, target mismatch; bilingual
diagnostics and facts; graph/schedule, rate/clock, bridge, cycle, deadline,
target, virtual-clock, replay, Unicode and differential fixtures; protocol
inventory and public status remain open under NODE-5303, NODE-5302,
GAP-CRITICAL-PROFILE-001, GAP-STRUCTURED-TASK-001,
GAP-ACTOR-AWAIT-REENTRY-001, GAP-ACTOR-MAILBOX-SUPERVISOR-001,
GAP-DETERMINISTIC-REPLAY-001, and missing RFC-K502 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
