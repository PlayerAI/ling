# DEC-0123: Internal ownership-diagnostic and repair boundary evidence / 内部 Ownership Diagnostic 与 Repair 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: diagnostic-design
> 相关规范/缺口：`DEC-0122` | `DEC-0001` | `DEC-0002` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed ownership
diagnostic and repair boundaries for the bounded `OWN-3206-OBSERVATION`
child. It checks deterministic, duplicate-free vocabulary. It does not
allocate error codes, define ownership meanings, rank repairs, publish JSON
fields, create LSP code actions, or define ownership semantics.

本决定只授权 test-only 的拟议 ownership diagnostic 与 repair 边界清单，供
`OWN-3206-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不分配 error code、不定义 ownership meaning、repair ranking、JSON field、LSP code action 或 ownership 语义。

## Question

The G3 plan asks for resource origin, move/borrow start, conflicting use,
region boundary, conflict persistence, and ranked fixes, with structured JSON
Repair data for a future LSP adapter. Which evidence can be retained without
freezing a future ownership diagnostic or repair protocol?

G3 计划要求 resource origin、move/borrow start、conflicting use、region boundary、conflict persistence 与 ranked fix，
并为未来 LSP adapter 提供结构化 JSON Repair data。在不冻结未来 ownership diagnostic 或 repair protocol 的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-diagnostics/tests/ownership_diagnostic_evidence.rs` keeps a
   test-local inventory of forty-five provisional boundaries: ownership facts
   and root-cause/severity ordering, Seed diagnostic interaction, structured
   Repair schema/ranking, edits/preconditions/stale span/version/applicability,
   alternatives/localization/safety, LSP and source/CST/AST/HIR/Checked Core/
   Graph/Audit Source projections, UTF-8 spans/Semantic IDs/schema versioning,
   migration/Profile/FFI/Native/Task/Actor/Fault boundaries, Unicode and
   host-text/unchecked-AST rejection, negative fixtures, differential
   evidence, and Seed migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.ownership-diagnostic-observation/0`. These bytes are not a diagnostic,
   error code, Fact, Repair, ranking, edit, code action, public schema, or
   ownership contract.
3. The child adds no ownership diagnostic, error-code allocation, repair
   ranking, JSON field, Semantic ID, LSP code action, public protocol, or
   migration rule. Public `OWN-3206` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its diagnostic checklist cannot
  authorize error meanings, repair ranking, source projections, or LSP fields.
- Accepted DEC-0001 and DEC-0002 govern existing diagnostic code allocation,
  bilingual messages, and UTF-8 byte spans; they do not authorize future
  ownership meanings or repairs.
- `DEC-0122` keeps Drop/cleanup vocabulary test-only while ownership authority
  is absent.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain
  Open; this decision records diagnostic vocabulary without resolving either
  gap.

## Conformance plan

- Assert all forty-five provisional ownership-diagnostic and repair boundaries
  and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep ownership meanings, code allocations, repair ranking, edits, LSP,
  migration, fuzzing, and interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Accepted Seed diagnostics, source acceptance, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No new `L-*` code, diagnostic field,
  repair protocol, or public ownership claim is registered.

## Unresolved alternatives

Ownership error taxonomy, resource/move/borrow/region facts, repair schema and
ranking, edits/preconditions, LSP mapping, public schema/versioning, migration,
Profile/FFI/Native/Task/Actor boundaries, diagnostics, and differential
semantics remain open under `GAP-OWNERSHIP-MODEL-001`,
`GAP-OWNERSHIP-PUBLIC-LIFETIME-001`, `OWN-3206`, and missing RFC-N305/RFC-0007
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
