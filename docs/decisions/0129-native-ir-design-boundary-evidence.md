# DEC-0129: Internal Native IR design boundary evidence / 内部 Native IR 设计边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed
backend-neutral Native IR design boundaries for the bounded
`NIR-3401-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not define an IR, instruction set, ABI, serializer,
verifier, debug schema, or lowering semantics.

本决定只授权 test-only 的拟议 backend-neutral Native IR 设计边界清单，供
`NIR-3401-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 IR、instruction set、ABI、serializer、
verifier、debug schema 或 lowering 语义。

## Question

NIR-3401 lists typed SSA/control flow, Value/Resource representation, checked
borrow or alias facts, explicit cleanup, ABI, Fault edges, source/debug
mapping, Effect boundaries, and deterministic serialization. Which boundary
vocabulary can be retained without inventing a Native IR contract?

NIR-3401 列出 typed SSA/control flow、Value/Resource representation、checked borrow 或 alias facts、explicit cleanup、ABI、
Fault edge、source/debug mapping、Effect boundary 与 deterministic serialization。哪些边界词汇可以保留，而不会发明 Native IR 契约？

## Decision

1. `crates/ling-types/tests/native_ir_design_evidence.rs` keeps a test-local
   inventory of forty-six provisional boundaries: typed SSA/control flow,
   blocks/phi/evaluation order, Value/Resource/Managed and aggregate/closure
   representation, borrow/alias/ownership facts, explicit Drop/cleanup,
   function/data ABI and layout, Fault/unwind, Effects/capabilities,
   Task/Actor/FFI/target packages, source/debug/definition mapping,
   deterministic serialization and version/rejection behavior, backend
   neutrality, Typed Core/Graph/Audit projections, Semantic IDs, differential
   evidence, Unicode spans, migration, and security bounds.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-ir-design-observation/0`. These bytes are not an IR node,
   instruction, ABI record, serializer, verifier, debug location, diagnostic,
   public protocol, or lowering contract.
3. The child adds no Native IR crate, instruction set, ABI record, serializer,
   verifier, debug schema, diagnostic, or placeholder backend API. Public
   `NIR-3401` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative and explicitly depends on
  RFC-N304; its checklist cannot define a public IR, binary format, ABI, or
  semantic lowering rule.
- Accepted `DEC-0128` through `DEC-0125` record only Profile/interop/
  collector/object-model vocabulary. `DEC-0012` governs Seed Semantic ID and
  canonical bytes, not a Native IR schema.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-CRITICAL-PROFILE-001` remain Open. RFC-N304 and its dependent Native,
  memory, ownership, and Profile authorities are not Accepted.

## Conformance plan

- Assert all forty-six provisional Native IR design boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep IR instructions/schema, SSA/phi validity, memory/ownership operands,
  cleanup/Fault/Effect edges, ABI/layout, FFI/target packages,
  source/debug mapping, serialization, verifier, and differential semantics
  deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No Native IR, ABI, serializer,
  diagnostic, Semantic ID, or public protocol claim is registered.

## Unresolved alternatives

NIR version and instruction set, SSA/phi and evaluation order, representation
and ownership facts, cleanup/Fault/Effect edges, ABI/layout/target policy,
FFI/runtime operations, source/debug mapping, serialization/versioning,
verifier, migration, security, and interpreter/VM/Native differential
semantics remain open under `NIR-3401`, `GC-3304`, `GC-3303`, `GC-3302`,
`GC-3301`, `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`,
`GAP-CRITICAL-PROFILE-001`, and missing RFC-N304/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
