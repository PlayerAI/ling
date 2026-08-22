# DEC-0172: Internal placement-constraint boundary evidence / 内部 Placement 约束边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: compiler-quality  
> 相关规范/缺口：`DEC-0171` | `DEC-0170` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PLC-4801-OBSERVATION`. It records provisional Placement vocabulary while
RFC-H405 and the Kernel/Device, Native/backend, ownership, and support
authorities remain unresolved.

本决定只授权 `PLC-4801-OBSERVATION` 使用 test-local 的 Placement 边界清单；
在 RFC-H405 以及 Kernel/Device、Native/backend、ownership 与 support 权威尚未解决时，
只记录临时 Placement 词汇。

## Question

PLC-4801 describes Placement as constraint solving rather than an arbitrary
runtime guess, with examples such as `requires gpu`, `prefers gpu`,
`forbids remote`, `same_node_as`, `near`, and `fallback cpu`. Which planning
vocabulary can be retained as bounded evidence without creating source
syntax, a solver, a target policy, or a public protocol?

## Decision

1. `crates/ling-types/tests/placement_constraint_evidence.rs` keeps a
   test-local inventory of sixty provisional Placement, hard/soft constraint,
   device/topology, buffer/ownership, transfer/synchronization, fallback,
   explain/replay/cache, privacy, fixture, diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.placement-constraint-observation/0`. These bytes
   are evidence only; they are not Placement syntax, AST/HIR/Typed Core
   fields, solver decisions, target facts, cache keys, diagnostics, or support.
3. No Placement grammar, constraint solver, topology/capability API, fallback
   planner, dependency, target package, cache/explain protocol, diagnostic, or
   placeholder API is added. Public `PLC-4801` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:429-446` is
  non-normative and explicitly depends on RFC-H405; its examples do not define
  grammar, constraint meaning, topology, buffer identity, fallback, or replay.
- `docs/ROADMAP-1.0.md:381-431` requires explicit, explainable Placement and
  deterministic capability/fallback evidence but does not authorize a solver
  before the G4 specification gates.
- `PLC-4801-AUTHORITY-AUDIT.md` records the missing authority, and
  `DEC-0171`/`DEC-0170` keep prerequisite accelerator vocabulary test-local.

## Conformance plan

- Assert all sixty Placement boundaries and local order; compare forward and
  reverse opaque bytes; reject duplicates.
- Defer syntax, AST/Core fields, topology and capability semantics, solver,
  fallback, explain/replay/cache behavior, diagnostics, and protocol behavior
  until RFC-H405 or an Accepted replacement defines them.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Only test-local evidence is added; no Placement or target support
claim is registered.

## Unresolved alternatives

Hard/soft constraint semantics; device and topology identity; capability
predicates; buffer/address-space ownership and transfer; remote boundaries;
numeric/effect/Fault preservation; availability, cost and deterministic tie
breaking; rejection/fallback legality and user intent; explain/replay/decision
schemas; cache identity, provenance, versioning, migration and corruption;
privacy and unstable host/driver exclusions; positive/negative/conflict,
topology/capability/fallback/explain/replay/cache/differential/Unicode/
determinism fixtures; diagnostics and public protocol inventory remain open
under PLC-4801, PLC-4802, ACC-4702, ACC-4701, DIR-4501 through DIR-4503,
GPU-4601 through GPU-4605, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing RFC-H405 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
