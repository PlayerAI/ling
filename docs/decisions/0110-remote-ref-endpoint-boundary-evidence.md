# DEC-0110: Internal RemoteRef and endpoint boundary evidence / 内部 RemoteRef 与 Endpoint 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: remote-design
> 相关规范/缺口：`DEC-0109` | `DEC-0010` | `GAP-ACTOR-REMOTE-DELIVERY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed RemoteRef and
endpoint boundaries for the bounded `REM-2601-OBSERVATION` child. It checks
deterministic, duplicate-free boundary vocabulary. It does not define remote
identity, serialize references, authenticate endpoints, emit network Effects,
or specify delivery/Fault behavior.

本决定只授权 test-only 的拟议 RemoteRef 与 Endpoint 边界清单，供
`REM-2601-OBSERVATION` 子任务使用。它只检查确定性、无重复的边界词汇；不定义 remote identity、不序列化
reference、不认证 endpoint、不产生 network Effect，也不规定 delivery/Fault 行为。

## Question

The remote-actor plan requires a RemoteRef distinct from local ActorRef,
EndpointId, RemoteActorId, ProtocolVersion, CapabilityToken, and explicit
network/send outcomes, but Accepted remote authority does not define their
contracts. What evidence can be retained without implementing a remote
protocol?

Remote Actor 计划要求与本地 ActorRef 区分的 RemoteRef、EndpointId、RemoteActorId、ProtocolVersion、
CapabilityToken 以及明确的 network/send 结果，但已接受的 Remote 权威尚未定义其契约。如何在不实现 remote
协议的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/remote_ref_evidence.rs` keeps a test-local
   inventory of fourteen provisional boundaries: local-reference separation,
   remote-reference identity, endpoint and remote-actor identity, protocol
   version, capability token, endpoint authority, protocol negotiation,
   network and ActorSend Effects, delivery/Fault outcomes, incarnation, and
   the serialization boundary.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.remote-ref-observation/0`. These bytes are not a RemoteRef, endpoint
   address, capability token, network Effect, delivery result, or protocol.
3. The child adds no RemoteRef type, endpoint registry, identity allocator,
   token verifier, network Effect, transport adapter, diagnostic, Semantic ID,
   public protocol, or migration rule. Public `REM-2601` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.3 requires accepted Actor, Effect, remote-delivery,
  authentication, and security contracts before remote actors.
- `DEC-0010` governs current Seed capability authorization only; it does not
  define remote tokens, endpoint trust, or network delivery.
- `DEC-0109` keeps cross-process replay acceptance boundaries test-only while
  remote protocol authority is absent.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open; this decision records remote
  vocabulary without resolving the gap.

## Conformance plan

- Assert all fourteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep local/remote identity and equality, endpoint authority, protocol
  negotiation, capability issuance/revocation, serialization prohibition,
  network/ActorSend Effects, delivery/Fault outcomes, incarnation/liveness,
  partition/retry/order, diagnostics, resources, security, cross-process,
  differential, and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public RemoteRef, endpoint, or
  remote-delivery protocol claim is registered.

## Unresolved alternatives

Remote identity allocation/reuse, local/remote equality, endpoint discovery and
trust, protocol negotiation, capability-token lifecycle, serialization,
network/send Effects, delivery/Fault semantics, incarnation/liveness,
partition/retry/order, migration, diagnostics, resource limits, runtime ABI,
and transport behavior remain open under `GAP-ACTOR-REMOTE-DELIVERY-001` and
`REM-2601`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
