# DEC-0146: Internal staged debugger capability boundary evidence / 内部分阶段调试器能力边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: debugger-quality  
> 相关规范/缺口：`DEC-0145` | `DEC-0144` | `ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DAP-3603-OBSERVATION` staged-debugger capability boundary. It records
provisional stage and inspection vocabulary while DAP, VM/Native, Task/Actor,
security, and protocol authorities remain unresolved.

本决定只授权 `DAP-3603-OBSERVATION` 使用 test-local 的拟议分阶段调试器能力边界清单，
在 DAP、VM/Native、Task/Actor、security 与 protocol 权威尚未解决时，只记录临时阶段与检查词汇。

## Question

DAP-3603 proposes staged launch/continue, breakpoints, stepping,
stack/scope/variables, conditional breakpoints/logpoints, attach, and future
Actor/Task views. Which planning vocabulary can be retained as bounded
evidence without implementing a debugger capability, condition evaluator,
attach path, or unresolved Task/Actor behavior?

DAP-3603 计划分阶段提供 launch/continue、断点、单步、stack/scope/variables、条件断点/logpoint、
attach 以及未来 Actor/Task 视图。在不实现调试器能力、条件求值器、attach 路径或未解决的
Task/Actor 行为的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/staged_debugger_capability_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering stages,
   capability negotiation, stop/breakpoint/step/source/binary/snapshot/source
   map identity, stacks/scopes/variables/Resource/Managed/ownership/Fault,
   condition and logpoint inputs/sandbox/side-effect/foreign-call/capability/
   host-I/O/allocation restrictions, attach authentication/session/cancel/
   timeout, target/profile/VM/Native selection, Task/Actor lifecycle,
   suspension/reentry/mailbox/supervision observation, malformed/unknown
   messages, positive/negative fixtures, deterministic/cross-engine evidence,
   bilingual diagnostics, Unicode, Semantic IDs, host-output exclusion, and
   protocol inventory separation.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.staged-debugger-observation/0`. These bytes are not debugger stages,
   stop/step semantics, condition evaluation, attach authorization, Task/Actor
   behavior, a diagnostic, provenance record, Semantic ID, or public protocol.
3. The child adds no debugger capability, runtime hook, condition evaluator,
   attach path, Actor/Task view, dependency, toolchain, diagnostic, protocol,
   or placeholder API. Public `DAP-3603` remains `BlockedSpec`; stale `zero`
   commands are not carried into implementation.

## Normative basis

- The G3+ execution package is non-normative and lists stages without defining
  stop/step granularity, expression evaluation, variable identity/lifetime,
  attach security, or Actor/Task observability.
- DAP-3601 and DAP-3602 remain `BlockedSpec`; their adapter, wire, lifecycle,
  extension, launch, and source-map contracts are not accepted.
- `docs/SEMANTICS.md` excludes Task and Actor execution from v0.0.1 and leaves
  their lifecycle/reentry/mailbox/supervision rules open. Accepted
  RFC-0014/RFC-0018/RFC-0019 provide experimental VM/source-map/Fault/
  cancellation/differential foundations only.
- `GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-AWAIT-REENTRY-001`,
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-NATIVE-BACKEND-ABI-001`, and the
  semantic/editor protocol gaps remain Open; no debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Conformance plan

- Assert all sixty provisional staged-debugger boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep capability negotiation, stage semantics, inspection/condition sandbox,
  attach, Task/Actor behavior, security, protocol readers/migrations, and
  public support behavior deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No debugger capability, runtime hook,
  condition evaluator, attach authorization, Task/Actor view, diagnostic,
  dependency, protocol, or support claim is registered.

## Unresolved alternatives

Stage ordering and support gates; stop/breakpoint/step/source-map/identity;
stack/scope/variable/ownership/Fault behavior; condition/logpoint sandboxing;
attach authentication/session; target/profile/VM/Native selection; Task/Actor
lifecycle, suspension/reentry, mailbox and supervision; malformed messages,
timeouts, diagnostics, Unicode, Semantic IDs, fixtures, migration, protocol
inventory, and editor integration remain open under DAP-3603, DAP-3601/3602,
DIFF-3702, the listed gaps, and missing debugger/Native/Task/Actor authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
