# DEC-0106: Internal effect-recorder boundary evidence / 内部 Effect Recorder 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0105` | `GAP-EFFECT-HANDLER-001` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed recordable
effect boundaries for the bounded `REP-2503-OBSERVATION` child. It names the
planned boundaries and checks deterministic duplicate-free ordering. It does
not observe execution, define handler semantics, serialize payloads, or expose
a recorder hook.

本决定只授权 test-only 的拟议可录制 Effect 边界清单，供 `REP-2503-OBSERVATION` 子任务使用。它记录计划
中的边界并检查确定性与无重复排序；不观察执行，不定义 handler 语义，不序列化 payload，也不暴露
recorder hook。

## Question

The execution plan lists Clock, Random, external input, network receive,
file/device reads, and scheduler nondeterminism as possible recording
boundaries, but Accepted RFC-C201/C205 authority does not define operation
identity, recordability, reconstruction, failure, or privacy. What evidence
can be retained without implementing a recorder?

执行计划列出了 Clock、Random、外部输入、网络接收、文件/设备读取和 scheduler nondeterminism 作为可能的
录制边界，但已接受的 RFC-C201/C205 权威尚未定义 operation identity、recordability、reconstruction、
failure 或 privacy。如何在不实现 recorder 的情况下保留证据？

## Decision

1. `crates/ling-effects/tests/effect_recorder_evidence.rs` keeps a test-local
   inventory of six provisional boundaries: `Clock`, `Random`,
   `ExternalInput`, `NetworkReceive`, `FileDeviceRead`, and
   `SchedulingNondeterminism`.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.effect-recorder-observation/0`. These bytes are not an event log.
3. The child adds no Effect recorder, operation identity, event sink, payload
   serializer, redaction policy, scheduler hook, diagnostic, Semantic ID,
   CLI/LSP command, public protocol, or migration rule. Public `REP-2503`
   remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Effect Row/Handler and Replay
  contracts before effect recording can be exposed.
- Existing Seed closed effect rows and `DEC-0021` compiler-query scheduling do
  not define runtime recordability or replay.
- `DEC-0105` keeps replay fields test-only while wire authority is absent.
- `GAP-EFFECT-HANDLER-001` and `GAP-DETERMINISTIC-REPLAY-001` remain Open; this
  decision records boundary vocabulary without resolving either gap.

## Conformance plan

- Assert all six provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep Effect Row/Handler operation identity, recordable/reconstructible
  semantics, handler masking, recorder lifecycle/failure, scheduler/Task/Actor
  interaction, payloads, privacy, diagnostics, resources, cross-process,
  differential, and migration fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public recorder or replay claim is
  registered.

## Unresolved alternatives

Effect operation IDs and boundaries, recordability versus reconstruction,
handler nesting/masking, unhandled operations, recorder failure, sensitive
values, scheduler nondeterminism, event order, privacy/redaction, schema,
resource limits, diagnostics, runtime ABI, and migration remain open under
`GAP-EFFECT-HANDLER-001`, `GAP-DETERMINISTIC-REPLAY-001`, and `REP-2503`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
