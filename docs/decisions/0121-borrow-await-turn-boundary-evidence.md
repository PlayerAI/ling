# DEC-0121: Internal cross-suspension and Actor-turn boundary evidence / 内部跨 suspension 与 Actor Turn 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: concurrency-design
> 相关规范/缺口：`DEC-0120` | `DEC-0119` | `DEC-0009` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed
cross-suspension and Actor-turn boundaries for the bounded
`OWN-3204-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not define `await`, suspension, pinning, state-machine
lowering, Actor reentry, message sendability, cancellation, Drop, diagnostics,
or ownership semantics.

本决定只授权 test-only 的拟议跨 suspension 与 Actor Turn 边界清单，供
`OWN-3204-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 `await`、suspension、pinning、state-machine
lowering、Actor reentry、message sendability、cancellation、Drop、diagnostic 或 ownership 语义。

## Question

The G3 plan sketches rejection of stack/turn-local borrows across suspension,
explicit pinned or owned state-machine fields, Actor-state lifetime limits,
ordinary-borrow rejection at remote message boundaries, and actionable fixes.
Which evidence can be retained without freezing an await, Actor, or lifetime
contract?

G3 计划列出 stack/turn-local borrow 不得跨 suspension、显式 pinned/owned state-machine field、Actor-state lifetime 限制、
remote message boundary 拒绝普通 borrow 与可操作修复。在不冻结 await、Actor 或 lifetime 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-concurrency/tests/borrow_await_turn_evidence.rs` keeps a
   test-local inventory of thirty-seven provisional boundaries: stack/turn
   local borrows, suspension/await and cross-suspension borrow, pinned/owned
   fields and state-machine lowering, Actor-state borrow/turn end/reentry,
   remote message borrow/sendability, copy/shorten/move/split-state fixes,
   cancellation/timeout/Drop/Fault/partial initialization, Region/Borrow and
   Resource/Managed interactions, Task/Actor boundaries, cross-package and
   FFI/Native ABI, Capability/security, diagnostics, projections, Unicode
   spans, interleaving/replay, differential evidence, and Seed migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.borrow-await-turn-observation/0`. These bytes are not an await,
   suspension state, pinning decision, lifetime result, Actor transition,
   message schema, diagnostic, or ownership contract.
3. The child adds no suspension Core, state-machine lowering, cross-turn borrow
   checker, Actor reentry rule, message gate, diagnostic, Semantic ID, public
   protocol, or migration rule. Public `OWN-3204` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its high-risk checks cannot
  authorize suspension, pinning, Actor reentry, sendability, or lifetime
  behavior.
- `DEC-0120` keeps region/lifetime vocabulary test-only while ownership
  authority is absent.
- `DEC-0119` keeps Borrow vocabulary test-only, and `DEC-0009` governs Seed
  mutable-place writes while excluding Borrow and suspension semantics.
- `GAP-ACTOR-AWAIT-REENTRY-001` and `GAP-OWNERSHIP-MODEL-001` remain Open;
  this decision records boundary vocabulary without resolving either gap.

## Conformance plan

- Assert all thirty-seven provisional suspension/Actor boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep await/suspension state, pinning, reentry, sendability, cancellation,
  Drop, diagnostics, migration, fuzzing, replay/interleaving, and
  interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Accepted Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public await, Actor, message, or
  ownership protocol claim is registered.

## Unresolved alternatives

Suspension and await state-machine identity, pinning/owned field eligibility,
cross-turn borrow and Actor reentry, message sendability/schema, cancellation/
timeout/Drop/Fault, Task/Actor ordering, FFI/Native ABI, Capability/security,
diagnostics, migration, and differential semantics remain open under
`GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-OWNERSHIP-MODEL-001`, `OWN-3204`, and the
missing RFC-N302/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
