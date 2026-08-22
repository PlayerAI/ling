# DEC-0117: Internal Managed-graph and island boundary evidence / 内部 Managed 图与 Island 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: memory-design
> 相关规范/缺口：`DEC-0116` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed Managed-graph
and island boundaries for the bounded `MEM-3104-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not define references,
reachability, collection, pinning, borrowed views, cross-domain transfer, or
Managed runtime semantics.

本决定只授权 test-only 的拟议 Managed 图与 Island 边界清单，供
`MEM-3104-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 reference、reachability、collection、
pinning、borrowed view、cross-domain transfer 或 Managed runtime 语义。

## Question

The G3 plan names Managed identity and graphs, Value/Resource edges, island
roots, cross-thread/Actor/FFI rules, pinning, and borrowed views. Which
evidence can be retained without freezing a GC, aliasing, isolation, or ABI
contract?

G3 计划列出 Managed identity/graph、Value/Resource edges、island root、cross-thread/Actor/FFI 规则、pinning 与
borrowed view。在不冻结 GC、aliasing、isolation 或 ABI 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/managed_island_evidence.rs` keeps a test-local
   inventory of thirty-eight provisional boundaries: Managed identity/graphs,
   Value/Resource edges, island roots and reachability, cycles and sharing,
   equality/hash/serialization, collection/finalization/OOM, pinning and
   borrowed-view expiry, mutation/concurrency, cancellation/Actor/Task and
   cross-thread/FFI transfer, Native/Target/Profile boundaries, security,
   Checked Core/Semantic Graph/Audit Source/canonical bytes, diagnostics,
   Unicode spans, differential evidence, deterministic observability, and
   island escape.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.managed-island-observation/0`. These bytes are not a Managed
   reference, graph, root, collector, pin, borrowed view, transfer mode,
   isolation rule, or runtime contract.
3. The child adds no Managed type or graph, island root, edge rule, collector,
   pinning API, borrowed-view type, sharing policy, diagnostic, Semantic ID,
   public protocol, or migration rule. Public `MEM-3104` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its Managed/island sketch cannot
  authorize graph reachability, collection, aliasing, or cross-domain ABI.
- `DEC-0116` keeps Resource and Drop vocabulary test-only while memory and
  ownership authority is absent.
- `DEC-0009` governs Seed Value mutation and excludes Resource, Borrow, and
  Managed behavior.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records Managed
  vocabulary without resolving the gap.

## Conformance plan

- Assert all thirty-eight provisional Managed/island boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep graph/edge rules, roots/cycles, collection/OOM, pinning/views,
  concurrency/transfer, isolation/security, diagnostics, migration, fuzzing,
  and interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public Managed, GC, isolation, or
  cross-domain protocol claim is registered.

## Unresolved alternatives

Managed identity and reachability, graph/edge ownership, root discovery,
cycle/sharing behavior, collection/finalization/OOM, pinning and borrowed views,
concurrency/transfer, isolation/security, Native/FFI ABI, diagnostics,
optimization, migration, and differential behavior remain open under
`GAP-OWNERSHIP-MODEL-001` and `MEM-3104`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
