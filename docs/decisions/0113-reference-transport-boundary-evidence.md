# DEC-0113: Internal reference-transport boundary evidence / 内部参考 Transport 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: remote-design
> 相关规范/缺口：`DEC-0112` | `DEC-0110` | `GAP-ACTOR-REMOTE-DELIVERY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed reference
transport and codec boundaries for the bounded `REM-2604-OBSERVATION` child.
It checks deterministic, duplicate-free vocabulary. It does not implement
loopback/TCP/QUIC, define a transport interface, encode/decode frames, grant
business-message capabilities, or expose Typed Faults.

本决定只授权 test-only 的拟议参考 Transport 与 codec 边界清单，供
`REM-2604-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不实现 loopback/TCP/QUIC，不定义 transport
interface，不编解码 frame，不授予 business-message capability，也不暴露 Typed Fault。

## Question

The plan proposes deterministic loopback tests plus independent TCP/QUIC
adapters, isolates business-message deserialization capabilities, and maps
codec failures to Typed Faults. Which evidence can be retained without
freezing an adapter ABI or transport equivalence claim?

计划提出 deterministic loopback 测试与独立 TCP/QUIC adapter，隔离 business-message deserialization capability，
并将 codec failure 映射为 Typed Fault。如何在不冻结 adapter ABI 或 transport equivalence 声明的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/remote_transport_evidence.rs` keeps a
   test-local inventory of eighteen provisional boundaries: transport
   interface, loopback, TCP/QUIC adapters, framing, codec, decoder budget,
   business-decode capability, endpoint/version negotiation, Typed Fault,
   timeout, disconnect, partition, backpressure, cancellation, determinism,
   and independent-process behavior.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.remote-transport-observation/0`. These bytes are not a transport,
   frame, codec, capability grant, Fault, or loopback/network equivalence
   contract.
3. The child adds no transport trait, loopback scheduler, TCP/QUIC adapter,
   frame codec, decoder, Capability, Typed Fault, diagnostic, Semantic ID,
   public protocol, or migration rule. Public `REM-2604` remains `BlockedSpec`.

## Normative basis

- The G2 execution package is non-normative; its reference-transport choices
  cannot authorize a wire format, adapter API, or Capability boundary.
- `DEC-0112` keeps delivery and failure vocabulary test-only while transport
  and remote runtime authority is absent.
- `DEC-0110` keeps RemoteRef and endpoint boundaries test-only.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open; this decision records
  transport vocabulary without resolving the gap.

## Conformance plan

- Assert all eighteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep transport/codec interface, framing, version/endpoint negotiation,
  decoder budgets, business-message capabilities, Typed Faults,
  loopback-versus-independent semantics, timeout/disconnect/partition,
  backpressure/cancellation, security, diagnostics, migration,
  cross-process, differential, and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public transport, adapter, codec,
  or Fault protocol claim is registered.

## Unresolved alternatives

Transport interface, loopback and TCP/QUIC adapters, framing/codec ownership,
decoder limits, Capability isolation, endpoint/version negotiation, Typed Fault
mapping, timeout/disconnect/partition, backpressure/cancellation,
loopback/network equivalence, security, diagnostics, migration, runtime ABI,
and cross-process behavior remain open under
`GAP-ACTOR-REMOTE-DELIVERY-001` and `REM-2604`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
