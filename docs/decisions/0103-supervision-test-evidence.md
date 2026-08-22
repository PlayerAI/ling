# DEC-0103: Internal supervision test evidence / 内部监督测试证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: actor-supervision
> 相关规范/缺口：`DEC-0101` | `DEC-0102` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an internal, publish-disabled structural test
corpus for the bounded `SUP-2403-OBSERVATION` child. The corpus names future
supervision scenarios and checks deterministic observation identities. It does
not execute Actors, restart children, consume mailboxes, restore state, or
assert recovery outcomes.

本决定只授权 publish-disabled、内部的结构化测试语料，供
`SUP-2403-OBSERVATION` 子任务使用。语料只命名未来监督场景并检查观察 identity 的确定性；不执行
Actor，不重启 child，不消费 mailbox，不恢复 state，也不断言恢复结果。

## Question

The supervision plan lists recovery and cleanup scenarios, but accepted
authority does not define their state machine, fixture schema, runtime trace,
or expected outcomes. What test evidence can be added without claiming
supervision conformance?

监督计划列出了恢复与清理场景，但已接受的权威尚未定义状态机、fixture schema、runtime trace 或
预期结果。如何在不声称监督符合性的前提下增加测试证据？

## Decision

1. `crates/ling-concurrency/tests/supervision_evidence.rs` records the seven
   planned scenario names—single child Fault, multiple child Faults, Fault
   during restart, budget exhaustion/escalation, parent termination, state
   restore failure, and mailbox cleanup—plus a vocabulary-only case.
2. Each fixture contains only opaque observation identities, optional opaque
   Actor identities, and the structural labels already authorized by
   `DEC-0101`. The tests verify non-empty structural evidence, deterministic
   ordering, and the complete label vocabulary; they do not interpret labels as
   runtime results.
3. The child adds no fixture wire schema, runtime harness, scheduler, restart
   policy, mailbox operation, state restore, Fault result, diagnostic, Semantic
   ID, CLI/LSP command, public protocol, or migration rule. Public `SUP-2403`
   remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor supervision, runtime,
  determinism/replay, and differential contracts before recovery behavior can
  be exposed.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset and do not define supervision fixtures or outcomes.
- `DEC-0101` and `DEC-0102` authorize only non-executable Supervisor and
  budget/circuit observation boundaries.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open; this decision records test
  vocabulary without resolving the gap.

## Conformance plan

- Assert that all seven planned scenario names are present and each has at
  least one structural observation.
- Compare forward and reversed fixture insertion order and require identical
  path-free canonical observation bytes.
- Cover every Supervisor observation label as vocabulary without assigning
  runtime meaning or expected recovery outcomes.
- Keep runtime traces, fixture schemas, Fault outcomes, cleanup policy, stress,
  replay, differential, and migration fixtures deferred to the parent
  authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled tests. No public supervision, restart,
  mailbox, or Actor behavior claim is registered.

## Unresolved alternatives

Fixture schema and versioning, state-machine transitions, Fault taxonomy,
restart/budget/backoff/circuit outcomes, mailbox cleanup, cancellation,
deterministic seed/replay, runtime differential comparison, resource limits,
privacy, diagnostics, and migration remain open under the supervision gap and
`SUP-2403`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
