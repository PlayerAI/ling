# DEC-0112: Internal remote-delivery boundary evidence / 内部 Remote Delivery 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: remote-design
> 相关规范/缺口：`DEC-0111` | `DEC-0013` | `GAP-ACTOR-REMOTE-DELIVERY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed remote-delivery
and failure boundaries for the bounded `REM-2603-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not choose a delivery
guarantee, implement retries/deduplication, define ordering, or expose a
remote Fault.

本决定只授权 test-only 的拟议 Remote Delivery 与 failure 边界清单，供
`REM-2603-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不选择 delivery guarantee、不实现 retry/
deduplication、不定义 ordering，也不暴露 remote Fault。

## Question

The delivery plan allows only modes explicitly supported by an Accepted RFC,
rejects unconditional “Exactly Once” claims, and lists timeout, disconnect,
duplicates, reordering, stale incarnations, restart, schema mismatch, and
capability revocation. What evidence can be retained without freezing those
guarantees?

Delivery 计划只允许 Accepted RFC 明确支持的模式，拒绝无条件的“Exactly Once”声明，并列出 timeout、disconnect、
duplicate、reorder、stale incarnation、restart、schema mismatch 与 capability revoke。如何在不冻结这些
保证的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/remote_delivery_evidence.rs` keeps a
   test-local inventory of eighteen provisional boundaries: delivery class,
   AtMostOnce, AtLeastOnce, IdempotentRetry, the ExactlyOnce boundary,
   delivery identity, idempotence key, deduplication, ordering, causality,
   timeout, disconnect, duplicate, reorder, stale incarnation, remote
   restart, schema mismatch, and Capability revocation.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.remote-delivery-observation/0`. These bytes are not a delivery policy,
   retry ledger, deduplication store, ordering guarantee, or Fault protocol.
3. The child adds no delivery-policy type, retry/deduplication algorithm,
   ordering contract, remote Fault, capability-revocation path, transport
   adapter, diagnostic, Semantic ID, public protocol, or migration rule.
   Public `REM-2603` remains `BlockedSpec`.

## Normative basis

- The G2 execution package is non-normative and prohibits treating a first
  implementation as language semantics.
- `DEC-0013` governs Seed main/runtime Faults only, not remote delivery
  outcomes or retry guarantees.
- `DEC-0111` keeps the envelope field vocabulary test-only while remote wire
  authority is absent.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open; this decision records delivery
  vocabulary without resolving the gap.

## Conformance plan

- Assert all eighteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep delivery guarantees, loss/duplicate behavior, idempotence-key scope,
  ordering/causality, timeout/disconnect/partition, stale incarnation,
  restart, schema/capability failures, retries, replay, diagnostics,
  resources, security, cross-process, differential, and runtime fixtures
  deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public delivery guarantee or
  remote protocol claim is registered.

## Unresolved alternatives

Delivery class guarantees, exactly-once boundary, identity and deduplication,
idempotence, ordering/causality, timeout/disconnect, partition/retry/restart,
schema/capability failure, Fault representation, migration, diagnostics,
resource limits, runtime ABI, and transport behavior remain open under
`GAP-ACTOR-REMOTE-DELIVERY-001` and `REM-2603`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
