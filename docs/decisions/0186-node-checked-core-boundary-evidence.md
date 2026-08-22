# DEC-0186: Internal Node Checked Core boundary evidence / Node Checked Core 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0185` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5302-OBSERVATION`. It records provisional Checked Core vocabulary for
Node ports, state, ticks, clocks, graph identity, feedback/cycles,
Fault/Contract relations, target evidence, diagnostics, and fixtures while
RFC-K502 and the dependent Node authorities remain unresolved.

本决定只授权 `NODE-5302-OBSERVATION` 使用 test-local 的 Checked Core 边界清单；在 RFC-K502 与
Node、Critical、ownership、concurrency、Native/ABI、Kernel/Device 等依赖权威尚未解决时，只记录
临时的 port、state、tick、clock、graph identity、feedback/cycle、Fault/Contract、target evidence、
diagnostic 与 fixture 词汇。

## Question

NODE-5302 proposes a checked representation for Node ports, state cells, tick
transitions, clocks/periods/deadlines, dependency graphs, delayed feedback,
Fault transitions, and Contract hooks. Which vocabulary can be retained as
bounded evidence without choosing a Core schema, ownership/commit semantics,
graph identity, fixed-point proof, or lowering contract?

## Decision

1. `crates/ling-types/tests/node_checked_core_evidence.rs` keeps a test-local
   inventory of sixty provisional Node Checked Core categories, graph/cycle/
   fixed-point obligations, state/port relations, target evidence,
   diagnostics, and fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-checked-core-observation/0`. These bytes are
   evidence only; they are not a Core schema, node type, graph checker,
   fixed-point solver, diagnostic, protocol, or support claim.
3. No Node AST/Typed-Core variant, cycle solver, dependency, diagnostic
   allocation, CLI/LSP route, protocol, support claim, or placeholder API is
   added. Public `NODE-5302` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:226-240` is
  non-normative; it defines no field types, units, ownership, graph identity,
  fixed-point theorem, or lowering/evaluation contract.
- `docs/SEMANTICS.md:372-404` excludes `NodeStep` from the Seed implementation;
  `:1380-1425` is a conceptual sketch and `:1914-1931` reserves Node.
- `docs/LANGUAGE.md:857-866` provides a surface example, not accepted Core or
  lowering authority.
- `docs/status/NODE-5302-AUTHORITY-AUDIT.md` records missing RFC-K502 and
  dependent graph, timing, ownership, resource, target, and transaction
  authority.

## Conformance plan

- Assert all sixty Node Checked Core boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Core schema/lowering, graph/cycle/fixed-point checking, diagnostics,
  CLI/LSP, and protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. The conceptual `NodeStep` and generic verifier behavior are not
reinterpreted as Node Core semantics; only test-local evidence is added.

## Unresolved alternatives

Core schema/version; port types and identities; state cells, tick transitions,
clock/period/deadline; dependency edges, feedback delay, instantaneous cycles,
fixed points and proof; graph identity/order/canonical bytes; source spans and
Semantic IDs; ownership/mutability/aliasing, initialization, presence,
sampling/commit, visibility, restart/cancellation; Effect/Capability/
Task/Actor/Kernel/Device/FFI relations; Fault/Contract, resource/recursion/
mailbox bounds; target/WCET/compiler/evidence identity; unknown/unsupported
graphs; bilingual diagnostics and facts; graph/cycle/fixed-point/state/clock/
Fault/target/migration/Unicode/differential fixtures; protocol inventory and
public status remain open under NODE-5302, NODE-5301,
GAP-CRITICAL-PROFILE-001, GAP-ACTOR-MAILBOX-SUPERVISOR-001,
GAP-OWNERSHIP-MODEL-001, GAP-NATIVE-BACKEND-ABI-001,
GAP-KERNEL-DEVICE-001, and missing RFC-K502 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
