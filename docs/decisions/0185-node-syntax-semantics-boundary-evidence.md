# DEC-0185: Internal Node syntax/semantics boundary evidence / Node 语法与语义边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0184` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5301-OBSERVATION`. It records provisional Node syntax, ports, state,
logical-tick, clock, composition, deadline/overrun, target evidence,
diagnostic, and fixture vocabulary while RFC-K502 and the dependent Critical,
ownership, concurrency, Native/ABI, Kernel/Device, numeric/effect, and
transaction authorities remain unresolved.

本决定只授权 `NODE-5301-OBSERVATION` 使用 test-local 的 Node 边界清单；在 RFC-K502 与 Critical、
ownership、concurrency、Native/ABI、Kernel/Device、numeric/effect、transaction 等依赖权威尚未
解决时，只记录临时的 syntax、port、state、logical tick、clock、composition、deadline/overrun、
target evidence、diagnostic 与 fixture 词汇。

## Question

NODE-5301 proposes a synchronous reactive `node` declaration with periodic
execution, inputs, outputs, persistent state, deadlines, and explicit Fault
behavior. Which vocabulary can be retained as bounded evidence without
choosing grammar, tick/sampling/commit semantics, clock units, scheduler,
target/WCET, or runtime contracts?

## Decision

1. `crates/ling-types/tests/node_syntax_semantics_evidence.rs` keeps a
   test-local inventory of sixty provisional Node syntax/semantics categories,
   timing/state/composition relations, target evidence, diagnostics, and
   fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-syntax-semantics-observation/0`. These bytes
   are evidence only; they are not Node syntax, a Checked Core form, a clock,
   scheduler, runtime, diagnostic, protocol, or support claim.
3. No `node` parser, `NodeStep` Core variant, scheduler, runtime, dependency,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `NODE-5301` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:199-224` is
  non-normative and depends on absent RFC-K502; it defines no grammar,
  type/effect rules, tick boundaries, sampling/commit ownership, clock units,
  initialization, composition, or Fault transitions.
- `docs/SEMANTICS.md:372-404` limits v0.0.1 to the Seed Core and excludes
  `NodeStep`; `:1380-1425` is a conceptual sketch and `:1914-1931` reserves
  Node for a future version.
- `docs/LANGUAGE.md:857-866` is a surface-design example, not accepted Node
  grammar or timing/lowering authority.
- `docs/status/NODE-5301-AUTHORITY-AUDIT.md` records missing RFC-K502 and
  dependent timing, boundedness, ownership, concurrency, backend, and
  transaction authority.

## Conformance plan

- Assert all sixty Node syntax/semantics boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Node grammar/Core/runtime, clock/scheduler, timing/WCET, Fault,
  diagnostics, CLI/LSP, and protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Node sketches and VM limits are not reinterpreted as
accepted syntax, real-time guarantees, deadline semantics, or target support;
only test-local evidence is added.

## Unresolved alternatives

Node declarations, ports, state and types; `every`/deadline duration units;
logical ticks, input sampling/output commit, initialization/restart,
absence/presence, clock domains/conversion, evaluation order and composition;
Effect/Capability/Task/Actor/Kernel/Device/FFI relations; persistent state and
visibility; missed ticks, overrun, Fault/recovery, cancellation/fallback;
scheduler/virtual clock, WCET/cache/bus assumptions, target/compiler/evidence
identity; Critical Profile, allocation/GC/recursion, mailbox/ownership;
unknown timing/resource; bilingual diagnostics and facts; virtual-clock,
state/restart, absence/presence, composition, deadline/target/migration,
Unicode and differential fixtures; protocol inventory and public status remain
open under NODE-5301, GAP-CRITICAL-PROFILE-001,
GAP-ACTOR-MAILBOX-SUPERVISOR-001, GAP-OWNERSHIP-MODEL-001,
GAP-NATIVE-BACKEND-ABI-001, GAP-KERNEL-DEVICE-001, and missing RFC-K502
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
