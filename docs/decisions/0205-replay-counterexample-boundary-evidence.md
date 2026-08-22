# DEC-0205: Internal Replay Counterexample boundary evidence / 反例重放边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0204` | `ROADMAP-1.0` | `GAP-DETERMINISTIC-REPLAY-001` | `GAP-CRITICAL-PROFILE-001` | `PROTO-REPLAY` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`MC-5604-OBSERVATION`. It records provisional model-counterexample conversion,
replay, scheduler, runtime, source-link, failure, privacy, and fixture
vocabulary while the replay/model-check/runtime authorities remain unresolved.

本决定只授权 `MC-5604-OBSERVATION` 使用 test-local 的 model-counterexample conversion、replay、scheduler、
runtime、source-link、failure、privacy 与 fixture 边界清单；在 replay/model-check/runtime 等权威尚未解决时，
只记录临时词汇，不实现重放协议。

## Question

MC-5604 proposes converting a model-check counterexample into a deterministic
scheduler/replay fixture, running it in a reference runtime, and linking it to
source positions. Which vocabulary can be retained as bounded evidence
without choosing conversion semantics, a replay schema, scheduler policy,
runtime route, or public replay protocol?

## Decision

1. `crates/ling-concurrency/tests/replay_counterexample_evidence.rs` keeps a
   test-local inventory of sixty provisional conversion, replay, scheduler,
   runtime, event, identity, source-link, failure, privacy, diagnostic, and
   fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.replay-counterexample-observation/0`. These bytes
   are observation evidence only; they are not a counterexample converter,
   replay schema, reader/writer, scheduler trace, runtime route, diagnostic,
   protocol, or support claim.
3. No replay protocol, reader/writer, scheduler trace, counterexample
   converter, reference-runtime hook, dependency, diagnostic allocation,
   CLI/LSP route, support claim, or placeholder API is added. Public
   `MC-5604` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:471-473` is a
  non-normative sentence. It defines no event vocabulary, scheduler policy,
  logical clock, effect boundary, source linkage, failure behavior, or file
  format.
- `docs/status/MC-5604-AUTHORITY-AUDIT.md` records the absent replay schema,
  deterministic scheduler contract, converter, runtime route, and fixtures.
- `GAP-DETERMINISTIC-REPLAY-001` and `GAP-CRITICAL-PROFILE-001` remain open;
  `PROTO-REPLAY` and `PROTO-EVIDENCE` are Future.
- Accepted RFC-0019 differential execution, RFC-0020 host cancellation, and
  DEC-0019 incremental scheduling are not model-counterexample replay
  authorities.
- RFC-K501/K502/K504/K505/K506/K507 and dependent concurrency/runtime
  authorities remain absent or unresolved.

## Conformance plan

- Assert all sixty replay-counterexample categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer converter/replay/runtime implementation, event/scheduler/effect/
  source-link semantics, schemas, diagnostics, protocols, and public support
  until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing differential/cancellation/query-scheduler evidence is not
reinterpreted as model-counterexample replay; only test-local evidence is
added.

## Unresolved alternatives

Checked-model/exploration-result/counterexample conversion; replay fixture,
schema, reader/writer and reference-runtime route; model/runtime/scheduler/
counterexample/replay identities; scheduler policy, logical clock, inputs,
host effects, state snapshots/checksums, Fault/restart and event ordering;
mailbox/backpressure/reentry/drop/expiry/cancellation/ownership/capability/
resource semantics; stable Semantic IDs, original UTF-8 spans and checked
snapshot identity; provenance/checksum/signature/redaction/privacy;
divergence, malformed/corrupt/unknown-field/unsupported-version/migration/
unavailable-input/unsupported-Fault and fail-closed behavior; diagnostics,
positive/negative/divergence/source-link/Unicode/determinism fixtures,
protocol inventory, and public support remain open under MC-5604, MC-5603,
RFC-K506, GAP-DETERMINISTIC-REPLAY-001, GAP-CRITICAL-PROFILE-001,
PROTO-REPLAY, PROTO-EVIDENCE, and missing replay authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
