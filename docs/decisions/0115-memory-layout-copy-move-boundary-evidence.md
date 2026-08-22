# DEC-0115: Internal Value-layout and Copy/Move boundary evidence / 内部 Value 布局与 Copy/Move 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: memory-design
> 相关规范/缺口：`DEC-0061` | `DEC-0008` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed Value-layout
and Copy/Move boundaries for the bounded `MEM-3102-OBSERVATION` child. It
checks deterministic, duplicate-free vocabulary. It does not define memory
kinds, representation, ownership, Copy/Move legality, ABI, serialization, or
optimization semantics.

本决定只授权 test-only 的拟议 Value 布局与 Copy/Move 边界清单，供
`MEM-3102-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 memory kind、representation、ownership、
Copy/Move 合法性、ABI、serialization 或 optimization 语义。

## Question

The G3 plan names inline values, register/stack optimization, Ling-defined
Copy/Move, implicit-copy restrictions, generic and closure interactions,
serialization and ABI overflow/padding/endianness, and differential evidence.
Which evidence can be retained without freezing a memory or ownership ABI?

G3 计划列出 inline value、register/stack optimization、Ling 定义的 Copy/Move、implicit-copy 限制、generic 与
closure interaction、serialization/ABI 的 overflow/padding/endianness 以及 differential 证据。在不冻结 memory
或 ownership ABI 的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/memory_layout_evidence.rs` keeps a test-local
   inventory of thirty-seven provisional boundaries: Value kind and
   representation, inline/register/stack choices, Copy/Move and implicit or
   explicit operations, closures/recursive aggregates/generic and Trait
   interactions, Resource and separate-compilation boundaries,
   equality/hash/serialization, Semantic Graph/Audit Source/canonical bytes,
   size/alignment/overflow/padding/endianness/discriminants/niches/pointer
   identity, optimization equivalence, diagnostics, ownership, Native ABI,
   Profile constraints, interpreter/VM/Native differentials, Unicode spans,
   and migration compatibility.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.memory-layout-observation/0`. These bytes are not a layout, Copy/Move
   rule, ownership judgment, ABI, serializer, diagnostic, or optimization
   contract.
3. The child adds no memory kind, layout type, Copy/Move trait, ownership
   checker, ABI field, serializer, diagnostic, Semantic ID, public protocol, or
   migration rule. Public `MEM-3102` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its layout and optimization notes
  cannot authorize observable representation or Copy/Move behavior.
- `DEC-0061` authorizes only the existing Seed completed-type Value
  classification and does not define Managed/Resource, ownership, or layout.
- `DEC-0008` and `DEC-0009` govern Seed value and mutation boundaries only;
  they do not define future Copy/Move traits or Native ABI.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records memory
  vocabulary without resolving the ownership gap.

## Conformance plan

- Assert all thirty-seven provisional memory/layout boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep memory kinds, Copy/Move legality, layout/ABI/serialization, ownership,
  diagnostics, optimization proof, profiles, migration, fuzzing, and
  interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public memory, ownership, ABI, or
  Copy/Move claim is registered.

## Unresolved alternatives

Memory-kind classification, representation and layout obligations, Copy/Move
legality, implicit copies, Resource interaction, ownership diagnostics,
serialization and canonical bytes, Native/FFI ABI, Profile constraints,
optimization proof, migration, and differential behavior remain open under
`GAP-OWNERSHIP-MODEL-001` and `MEM-3102`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
