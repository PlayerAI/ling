# DEC-0190: Internal Node/Actor boundary evidence / Node/Actor 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0189` | `ROADMAP-1.0` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-STRUCTURED-TASK-001` | `GAP-DETERMINISTIC-REPLAY-001` | `GAP-CRITICAL-PROFILE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5306-OBSERVATION`. It records provisional Node/Actor bridge vocabulary
for identities, envelopes, sampled input and bounded output, mailbox policy,
ownership, turns, replay, profiles, diagnostics, and fixtures while
RFC-K502, RFC-0008, RFC-0009, RFC-0010, and the dependent Node/Native/Device
authorities remain unresolved.

本决定只授权 `NODE-5306-OBSERVATION` 使用 test-local 的 Node/Actor 边界清单；在 RFC-K502、RFC-0008、
RFC-0009、RFC-0010 以及 Node/Native/Device 等依赖权威尚未解决时，只记录临时的 identity、envelope、
sampled input、bounded output、mailbox policy、ownership、turn、replay、profile、diagnostic 与 fixture 词汇。

## Question

NODE-5306 proposes an explicit bridge from an Actor through a sampled input
queue into a Node and from a Node through a bounded output event back to an
Actor. Which vocabulary can be retained as bounded evidence without choosing
queue capacity, backpressure/drop/expiry, sampling/commit clocks, ordering,
ownership/serialization, Actor turn/reentry, Fault/restart, replay, or a
hard-real-time support claim?

## Decision

1. `crates/ling-types/tests/node_actor_boundary_evidence.rs` keeps a
   test-local inventory of sixty provisional Node/Actor identity, envelope,
   queue, delivery, ownership, turn/lifecycle, replay/profile, diagnostic,
   and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-actor-boundary-observation/0`. These bytes
   are evidence only; they are not a mailbox, bridge envelope, queue,
   scheduler, ownership model, replay schema, diagnostic, protocol, or
   support claim.
3. No Node/Actor queue or bridge runtime, envelope schema, dependency,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `NODE-5306` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:277-286` is
  non-normative; it names the two bridge directions and the hard-real-time
  non-waiting rule but defines no envelope, queue, clock, ownership,
  capacity, delivery, restart, or fallback contract.
- `docs/SEMANTICS.md:1283-1425` and `docs/LANGUAGE.md:827-866` are
  conceptual Actor/Node sketches; `docs/SEMANTICS.md:1914-1931` reserves
  these systems features beyond Seed.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-ACTOR-AWAIT-REENTRY-001`,
  `GAP-STRUCTURED-TASK-001`, `GAP-DETERMINISTIC-REPLAY-001`, and
  `GAP-CRITICAL-PROFILE-001` leave the bridge contracts open. `PROTO-REPLAY`
  remains Future.
- No RFC-0008, RFC-0009, RFC-0010, or RFC-K502 is Accepted for this boundary.
  Existing Actor/mailbox observations, VM host controls, and compiler-query
  scheduling are not Node/Actor delivery authority.

## Conformance plan

- Assert all sixty Node/Actor boundary categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer bridge queues/envelopes, capacity/backpressure/drop/expiry,
  sampling/commit, ordering, ownership/serialization, turn/reentry,
  supervision/Fault/restart, replay, diagnostics, CLI/LSP, and runtime
  protocol behavior until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing structural Actor/mailbox models and host cancellation are
not reinterpreted as bridge delivery or hard-real-time semantics; only
test-local evidence is added.

## Unresolved alternatives

Node/Actor identity and bridge/envelope version; port/message schema and
serialization; ownership/Move/Borrow/Managed and privacy; mailbox capacity,
admission, backpressure, drop/expiry/stale input; sampling/commit and clock
conversion; delivery order and simultaneous-event tie-break; bounded memory;
Actor turn/await/reentry, cancellation, supervision/restart/shutdown,
Fault/fallback; hard-real-time non-waiting, network/service boundary,
Critical/Native/Device/target profiles; replay/effect records, privacy,
corruption/divergence and migration; bilingual diagnostics and facts;
capacity, stale/drop, ordering, sampling, ownership, restart/replay,
Unicode, differential fixtures; protocol inventory and public status remain
open under NODE-5306, NODE-5305, RFC-K502, RFC-0008, RFC-0009, RFC-0010, and
the listed gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
