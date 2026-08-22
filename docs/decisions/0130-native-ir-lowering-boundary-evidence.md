# DEC-0130: Internal Native IR lowering boundary evidence / 内部 Native IR Lowering 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Core to
Native IR lowering slices and preservation boundaries for the bounded
`NIR-3402-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not emit Native IR, select an ABI, define lowering
translations, or establish an interpreter/VM/Native differential protocol.

本决定只授权 test-only 的拟议 Core 到 Native IR lowering slice 与 preservation 边界清单，供
`NIR-3402-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不生成 Native IR、不选择 ABI、不定义 lowering translation，
也不建立 interpreter/VM/Native differential protocol。

## Question

NIR-3402 proposes vertical slices from integer/bool calls through records,
ADTs, mutable places, closures, Effects, Resource/Drop, Managed handles, and
Task/Actor ABI, with differential tests at every step. Which planning
vocabulary can be retained without choosing a translation or backend contract?

NIR-3402 提议从 integer/bool call 经过 record、ADT、mutable place、closure、Effect、Resource/Drop、Managed handle 与
Task/Actor ABI 的纵向 slice，并为每一步加入 differential test。哪些规划词汇可以保留，而不会选择 translation 或 backend 契约？

## Decision

1. `crates/ling-types/tests/native_ir_lowering_evidence.rs` keeps a test-local
   inventory of forty-six provisional boundaries: the nine planned lowering
   slices, Checked-Core-only input and unsupported-form rejection, evaluation
   order and Value/aggregate/closure representation, memory/borrow/alias/
   ownership, cleanup/Drop/allocation/GC barriers, Effects/capabilities,
   Fault/cancellation, source spans and Semantic IDs, ABI/targets/
   Profiles/FFI/thread reentry/runtime library, migration/versioning,
   deterministic serialization and malformed input, interpreter/VM/Native
   differential evidence, nondeterminism/target/host-fault/metric/debug
   exclusions, bilingual diagnostics, Unicode spans, resource bounds, and
   Seed compatibility.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-ir-lowering-observation/0`. These bytes are not a lowering,
   instruction, ABI adapter, native target, diagnostic, differential trace,
   public protocol, or semantic-preservation proof.
3. The child adds no lowering pass, NIR instruction use, native target, ABI
   adapter, diagnostic, differential protocol, or placeholder crate. Public
   `NIR-3402` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its slice order cannot define
  translations, evaluation order, representation, or differential equivalence.
- Accepted `DEC-0129` defines only test-local NIR design vocabulary; accepted
  `DEC-0128` through `DEC-0125` define only Profile/interop/collector/
  object-model evidence. `DEC-0012` governs Seed Semantic ID/canonical bytes,
  not Native lowering.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and the Task/Actor
  gaps remain Open. RFC-N304 and dependent Native, memory, ownership, FFI, and
  Profile authorities are not Accepted.

## Conformance plan

- Assert all forty-six provisional lowering boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep Core-to-NIR mappings, memory/closure lowering, Effect/Fault/cleanup,
  Managed/Resource/Task/Actor operations, ABI/targets, unsupported-form
  diagnostics, differential harnesses, and Native code generation deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No lowering, Native IR, ABI,
  differential, diagnostic, Semantic ID, or public protocol claim is
  registered.

## Unresolved alternatives

Core-to-NIR mapping, evaluation/representation, ownership/cleanup/GC,
Effects/Fault/cancellation, ABI/target/Profile/FFI/reentry, unsupported-form
diagnostics, deterministic serialization, differential equivalence, migration,
and Native code generation remain open under `NIR-3402`, `NIR-3401`,
`GC-3304`, `GC-3303`, `GC-3302`, `GC-3301`,
`GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and missing
RFC-N304/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
