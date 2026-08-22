# DEC-0125: Internal Managed object-model boundary evidence / 内部 Managed 对象模型边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: managed-runtime
> 相关规范/缺口：`DEC-0124` | `DEC-0117` | `DEC-0009` | `ROADMAP-1.0` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed contracts
that a future Managed object model must settle for the bounded
`GC-3301-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not define object representation, identity, collection,
roots, barriers, weak references, finalization, allocation, OOM behavior,
profiles, FFI, diagnostics, or runtime semantics.

本决定只授权 test-only 的拟议 Managed 对象模型契约清单，供
`GC-3301-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义对象表示、身份、
collection、root、barrier、weak reference、finalization、allocation、OOM 行为、Profile、FFI、
诊断或运行时语义。

## Question

GC-3301 lists an invisible object header, type metadata, root and write-barrier
interfaces, weak-reference and finalization policy, OOM-as-Fault, and pointer
identity. Which boundary vocabulary can be retained as implementation-planning
evidence without prematurely selecting a runtime contract?

GC-3301 列出不可见 object header、type metadata、root 与 write-barrier interface、weak reference 与
finalization policy、OOM-as-Fault 以及 pointer identity。哪些边界词汇可以作为实现规划证据保留，
而不会提前选择运行时契约？

## Decision

1. `crates/ling-types/tests/managed_object_model_evidence.rs` keeps a
   test-local inventory of forty provisional boundaries: object identity and
   private representation, type metadata, roots and handles, reachability and
   cycles, mutation barriers, weak references and finalization, allocation and
   OOM, pointer/address observability, Managed/Resource/ownership boundaries,
   pinning and borrowed views, Profiles, FFI roots, projections, diagnostics,
   Unicode spans, deterministic traversal, differential evidence, migration,
   security, and collection boundaries.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.managed-object-model-observation/0`. These bytes are not an object
   header, metadata schema, root handle, collector trace, barrier protocol,
   weak reference, finalizer, allocator, OOM Fault, pointer identity, public
   protocol, or runtime contract.
3. The child adds no Managed runtime crate, object layout, collector, root or
   handle API, barrier, weak reference, finalizer, allocator policy, OOM
   diagnostic, public protocol, or placeholder G3 API. Public `GC-3301` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative and cannot authorize a Managed
  runtime representation or observable identity rule.
- `DEC-0124` records the immediately preceding bounded evidence slice without
  resolving ownership or property semantics; `DEC-0117` keeps Managed-island
  vocabulary test-only while the object/ownership authority is absent.
- `DEC-0009` governs Seed mutable places and does not introduce Managed values,
  roots, or pointer identity.
- `GAP-OWNERSHIP-MODEL-001` remains Open, and RFC-N303/RFC-0007 are not
  Accepted; this decision records vocabulary without resolving those gaps.

## Conformance plan

- Assert all forty provisional object-model boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep object layout, reachability, collection, barrier ordering, weak and
  finalizer behavior, OOM classification, Profile/FFI rules, diagnostics, and
  differential semantics deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No object-model diagnostic, schema,
  Semantic ID, or public protocol claim is registered.

## Unresolved alternatives

Object layout and metadata versioning, logical versus pointer identity, root
lifetimes, cycle handling, barrier memory ordering, weak-reference and
finalization policy, OOM recovery, Profile/FFI/pinning behavior, diagnostics,
migration, and interpreter/VM/Native differential semantics remain open under
`GC-3301`, `GAP-OWNERSHIP-MODEL-001`, and missing RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
