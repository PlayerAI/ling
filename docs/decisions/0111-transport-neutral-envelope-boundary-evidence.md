# DEC-0111: Internal transport-neutral envelope boundary evidence / 内部传输中立 Envelope 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: remote-design
> 相关规范/缺口：`DEC-0110` | `DEC-0012` | `GAP-ACTOR-REMOTE-DELIVERY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed
transport-neutral envelope boundaries for the bounded `REM-2602-OBSERVATION`
child. It checks deterministic, duplicate-free field vocabulary. It does not
define canonical wire bytes, serialize payloads, calculate checksums, or
implement transport behavior.

本决定只授权 test-only 的拟议传输中立 Envelope 边界清单，供
`REM-2602-OBSERVATION` 子任务使用。它只检查确定性、无重复的字段词汇；不定义 canonical wire bytes、不序列化
payload、不计算 checksum，也不实现 transport 行为。

## Question

The remote-envelope plan lists protocol/type/schema/message identities,
deadlines, cancellation, delivery, authentication, payload integrity, and
resource fields, while explicitly deferring serialization to an RFC. What
evidence can be retained without freezing a wire ABI?

Remote Envelope 计划列出 protocol/type/schema/message identity、deadline、cancellation、delivery、
authentication、payload integrity 与 resource 字段，同时明确将 serialization 留给 RFC。如何在不冻结 wire
ABI 的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/remote_envelope_evidence.rs` keeps a
   test-local inventory of eighteen provisional boundaries: protocol version,
   sender/receiver Semantic types, message schema/id, correlation, deadline,
   cancellation, delivery policy, authentication metadata, payload,
   payload checksum, extensions, identity/incarnation binding, integrity,
   resource limits, and migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.remote-envelope-observation/0`. These bytes are not an envelope,
   serializer, checksum, protocol version, or transport contract.
3. The child adds no envelope struct, encoder/decoder, checksum, authentication
   metadata, transport adapter, diagnostic, schema, Semantic ID, public
   protocol, or migration rule. Public `REM-2602` remains `BlockedSpec`.

## Normative basis

- The G2 execution package expressly defers serialization format to an RFC;
  its field list cannot authorize a wire ABI.
- `DEC-0012` governs existing Seed Semantic IDs/canonical bytes only, not
  remote envelope bytes or payload integrity.
- `DEC-0110` keeps RemoteRef and endpoint boundaries test-only while remote
  protocol authority is absent.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open; this decision records envelope
  vocabulary without resolving the gap.

## Conformance plan

- Assert all eighteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep canonical field encoding, required/optional/extension rules, version
  negotiation, identity/schema binding, deadline/cancellation units, delivery
  and retry semantics, payload/checksum bytes, authentication, resources,
  diagnostics, migration, partition, security, cross-process, differential,
  and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public envelope, serializer, or
  remote protocol claim is registered.

## Unresolved alternatives

Canonical bytes and field encoding, extension/version negotiation, identity and
schema binding, deadline/cancellation units, delivery/retry/order, payload
serialization/checksum, authentication/privacy, resource limits, diagnostics,
migration, transport adapters, runtime ABI, and cross-process behavior remain
open under `GAP-ACTOR-REMOTE-DELIVERY-001` and `REM-2602`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
