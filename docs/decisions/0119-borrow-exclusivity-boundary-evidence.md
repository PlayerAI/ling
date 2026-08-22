# DEC-0119: Internal borrow-exclusivity boundary evidence / 内部 Borrow Exclusivity 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: ownership-design
> 相关规范/缺口：`DEC-0118` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed borrow-
exclusivity boundaries for the bounded `OWN-3202-OBSERVATION` child. It
checks deterministic, duplicate-free vocabulary. It does not define borrow
types, overlap algebra, automatic borrowing, lifetimes, diagnostics, or
exclusivity semantics.

本决定只授权 test-only 的拟议 Borrow Exclusivity 边界清单，供
`OWN-3202-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 borrow type、overlap algebra、automatic
borrow、lifetime、diagnostic 或 exclusivity 语义。

## Question

The G3 plan sketches immutable/mutable borrow exclusivity, place overlap,
field splitting, conservative index aliases, automatic reborrows, temporary
lifetime, iterator mutation, and suspension restrictions. Which evidence can
be retained without freezing a public borrow or lifetime contract?

G3 计划列出 immutable/mutable borrow exclusivity、place overlap、field splitting、conservative index alias、
automatic reborrow、temporary lifetime、iterator mutation 与 suspension restriction。在不冻结 public borrow 或
lifetime 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/borrow_exclusivity_evidence.rs` keeps a
   test-local inventory of thirty-four provisional boundaries:
   immutable/mutable borrows and alias identity, place overlap/field splitting
   and index/dynamic projections, automatic borrow/reborrow/call-site
   coercion, temporary lifetimes/extensions, iterator/container mutation,
   mutable places and closures, branch/loop lifetime, return/public escape,
   Task/Actor/suspension/Pin/Region, FFI/Native, Copy/Move and
   Resource/Managed/Trait interactions, diagnostics, projections, Unicode
   spans, differential evidence, deterministic approximation, and Seed
   migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.borrow-exclusivity-observation/0`. These bytes are not a borrow,
   alias relation, overlap result, lifetime, dataflow result, diagnostic, or
   ownership contract.
3. The child adds no borrow type, exclusivity checker, overlap solver,
   automatic borrow insertion, temporary lifetime rule, diagnostic, Semantic
   ID, public protocol, or migration rule. Public `OWN-3202` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its exclusivity sketch cannot
  authorize borrow syntax, alias compatibility, or lifetime inference.
- `DEC-0118` keeps Place/Move vocabulary test-only while ownership authority
  is absent.
- `DEC-0009` governs Seed mutable-place writes and excludes Borrow, `&mut`,
  implicit references, and Borrow Edges.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records borrow
  vocabulary without resolving the gap.

## Conformance plan

- Assert all thirty-four provisional borrow boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep exclusivity judgments, overlap/index policy, automatic reborrows,
  lifetimes, iterators, suspension/Actor boundaries, diagnostics, migration,
  fuzzing, and interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Accepted Seed source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No public Borrow, lifetime, or
  exclusivity protocol claim is registered.

## Unresolved alternatives

Borrow type and alias identity, overlap/field/index rules, automatic borrow and
reborrow, temporary lifetime, iterator mutation, public lifetime/Region,
suspension/Task/Actor escape, FFI/Native behavior, diagnostics, migration, and
differential semantics remain open under `GAP-OWNERSHIP-MODEL-001` and
`OWN-3202`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
