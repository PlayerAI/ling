# DEC-0191: Internal Node conformance boundary evidence / Node 一致性边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0190` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-DETERMINISTIC-REPLAY-001` | `GAP-STRUCTURED-TASK-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5307-OBSERVATION`. It records provisional Node-conformance vocabulary
for fixture protocols, oracles, initialization, ticks/state, rates and
inputs, deadlines/Fault/fallback, replay, target evidence, diagnostics, and
fixtures while RFC-K502 and the dependent Node, Actor, ownership, replay,
Native, and Critical authorities remain unresolved.

本决定只授权 `NODE-5307-OBSERVATION` 使用 test-local 的 Node conformance 边界清单；在 RFC-K502 以及
Node、Actor、ownership、replay、Native、Critical 等依赖权威尚未解决时，只记录临时的 fixture protocol、
oracle、initialization、tick/state、rate/input、deadline/Fault/fallback、replay、target evidence、diagnostic 与 fixture 词汇。

## Question

NODE-5307 lists initialization, multi-tick state, multi-rate execution,
stale or missing input, deadline hit/miss, fallback, restart/safe mode,
replay, and deterministic static scheduling. Which vocabulary can be retained
as bounded evidence without choosing a conformance protocol, fixture
manifest, oracle, exact timing/state/Fault rules, replay identity, or target
support claim?

## Decision

1. `crates/ling-types/tests/node_conformance_evidence.rs` keeps a test-local
   inventory of sixty provisional Node-conformance protocol, fixture, state,
   timing/input, Fault/fallback, replay/target, diagnostic, and evidence
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-conformance-observation/0`. These bytes are
   evidence only; they are not a conformance runner, fixture manifest,
   oracle, schedule, replay schema, diagnostic, protocol, or support claim.
3. No Node conformance runner, fixture schema, oracle, dependency, diagnostic
   allocation, CLI/LSP route, protocol, support claim, or placeholder API is
   added. Public `NODE-5307` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:288-300` is a
  non-normative checklist. It defines no fixture version, oracle, expected
  state/output, timing, Fault, replay, target, or compatibility contract.
- `docs/status/NODE-5307-AUTHORITY-AUDIT.md` records the missing Node
  conformance protocol, manifest, oracle, timing/Fault/replay identity,
  bridge, ownership, and target evidence authority.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Node outside Seed; later
  Node text is conceptual. `RFC-0019` covers only interpreter–VM logical
  equivalence and `RFC-0020` covers VM host controls, not Node conformance.
- `GAP-CRITICAL-PROFILE-001`, `GAP-DETERMINISTIC-REPLAY-001`,
  `GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`,
  `GAP-NATIVE-BACKEND-ABI-001`, and the ownership/Device gaps remain open;
  `PROTO-REPLAY` is Future.

## Conformance plan

- Assert all sixty Node-conformance categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer the runner, manifest, oracle, initialization/tick/state semantics,
  rate/input/deadline/Fault/fallback/restart behavior, replay, target
  evidence, diagnostics, CLI/LSP, and runtime protocol behavior until
  accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Seed conformance and interpreter–VM differential tests
are not reinterpreted as Node conformance; only test-local evidence is added.

## Unresolved alternatives

Conformance protocol/version, fixture manifest and oracle; initialization,
tick/state/multi-rate/clock/rate/input presence/staleness/order; deadline,
overrun, Fault, fallback, restart/safe mode; static schedule/WCET/memory/ABI;
reference versus target evidence; event/effect/input/output identity,
replay/divergence/corruption/privacy/migration; Node/Actor/Task bridge,
ownership/mailbox; Semantic IDs/spans, bilingual diagnostics/facts;
positive/negative, initialization, tick/state, multi-rate, stale input,
deadline, fallback, restart/safe-mode, replay, schedule, differential,
Unicode, migration fixtures; protocol inventory and public status remain open
under NODE-5307, NODE-5306, NODE-5305, RFC-K502, and the listed gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
