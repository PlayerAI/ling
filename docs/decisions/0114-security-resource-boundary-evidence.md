# DEC-0114: Internal security and resource boundary evidence / 内部安全与资源边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: remote-design
> 相关规范/缺口：`DEC-0113` | `DEC-0010` | `GAP-ACTOR-REMOTE-DELIVERY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed security and
resource boundaries for the bounded `REM-2605-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not define quotas,
authentication, authorization, replay protection, schema behavior, decoder
behavior, or remote runtime semantics.

本决定只授权 test-only 的拟议安全与资源边界清单，供
`REM-2605-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义配额、认证、授权、重放保护、schema
行为、decoder 行为或 remote runtime 语义。

## Question

The security plan names bounded frames and messages, decoder depth and
allocation, mailbox and connection limits, authentication and Capability
boundaries, replay/rate controls, schema rejection, and differential/fuzz
evidence. Which evidence can be retained without freezing a remote security
protocol, host quota policy, or denial-of-service guarantee?

安全计划列出 frame/message 大小、decoder depth 与 allocation、mailbox 与 connection 限制、authentication 与
Capability 边界、replay/rate 控制、schema rejection 以及 differential/fuzz 证据。在不冻结 remote security
协议、host quota policy 或拒绝服务保证的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-concurrency/tests/remote_security_resource_evidence.rs`
   keeps a test-local inventory of thirty-one provisional boundaries:
   frame/message limits, decoder depth/allocation, mailbox ingress,
   connection and in-flight retry limits, replay window and rate limit,
   authentication/authorization hooks, trust roots, Capability issuance,
   attenuation and revocation, endpoint binding, privacy, replay protection,
   unknown/malformed schema, oversized input, resource exhaustion,
   duplicate/replay and rate exhaustion, decoder fuzzing, Unicode source
   spans, interpreter/VM/runtime differential evidence, loopback and
   independent transports, and the business-code boundary.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.remote-security-resource-observation/0`. These bytes are not a quota,
   authentication credential, authorization decision, replay window, schema
   gate, decoder, transport, Capability, Fault, or runtime contract.
3. The child adds no resource policy, decoder, ingress limiter, authentication
   provider, Capability lifecycle, replay protector, rate limiter, schema
   validator, diagnostic, Semantic ID, public protocol, or migration rule.
   Public `REM-2605` remains `BlockedSpec`.

## Normative basis

- The G2 execution package is non-normative; its security checklist cannot
  authorize a public quota, authentication ABI, decoder behavior, or network
  protocol.
- `DEC-0113` keeps reference transport and codec vocabulary test-only while
  remote authority is absent.
- `DEC-0010` governs current Seed Capability authorization only; it does not
  define remote credentials, trust roots, revocation, or network security.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open; this decision records
  security/resource vocabulary without resolving the gap.

## Conformance plan

- Assert all thirty-one provisional security and resource boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep quota/accounting, decoder/schema behavior, authentication,
  authorization, Capability issuance/attenuation/revocation, privacy,
  replay/rate guarantees, transport equivalence, diagnostics, migration,
  fuzzing, differential, and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public security, quota, or remote
  protocol claim is registered.

## Unresolved alternatives

Resource accounting and quota scope, decoder limits, host versus Profile
policy, trust roots, credential and Capability lifecycle, endpoint binding,
privacy, replay/rate semantics, schema/version failure, Fault mapping,
transport/runtime ownership, diagnostics, migration, fuzz corpus, and
cross-process behavior remain open under
`GAP-ACTOR-REMOTE-DELIVERY-001` and `REM-2605`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
