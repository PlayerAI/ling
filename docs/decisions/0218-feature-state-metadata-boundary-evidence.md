# DEC-0218: Internal Feature-State Metadata boundary evidence / 内部功能状态元数据边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: stabilization
> 相关规范/缺口：`DEC-0217` | `ROADMAP-1.0` | `GAP-REGISTER` | `TASK-STATUS` | `SUPPORT-MATRIX`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only bounded test evidence for
`STAB-6103-OBSERVATION`. It preserves the existing internal distinction
between implementation state and compatibility stability and keeps the
generated governance fixture explicitly non-public and unimplemented.

本决定只授权 `STAB-6103-OBSERVATION` 使用有界测试证据，保留内部的实现状态
与兼容稳定性之间的区别，并继续明确生成的治理 fixture 不是公共协议且命令尚未
实现。

## Question

Which feature-state metadata boundaries can be tested before Ling has an
Accepted public schema, lifecycle policy, command, or cross-tool consumer
contract?

## Decision

1. `crates/ling-types/tests/feature_state_metadata_evidence.rs` keeps sixty
   test-local identity, state, lifecycle, consumer, compatibility, and evidence
   categories with deterministic ordering and duplicate rejection.
2. Opaque bytes tagged `ling.feature-state-metadata-observation/0` are test
   evidence only; they are not a schema, Semantic ID input, or public protocol.
3. `tools/xtask/src/status.rs` tests two closed and distinct vocabularies:
   current implementation state is `Unavailable | Partial | Implemented`,
   while stability is `Experimental | Preview | Stable | Deprecated |
   Removed`. Values from one domain are rejected in the other.
4. The existing `ling.governance.feature-state-fixture/1` remains internal,
   deterministic, `implemented: false`, and `public_contract: false`.
   `ling support --format json` remains a proposed, rejected command.
5. Status, traceability, and support-matrix consistency checks remain the
   internal source-of-truth chain. No feature is promoted or demoted.
6. No public CLI/build/package/LSP/Zed schema or consumer, diagnostic,
   transition policy, compatibility promise, or placeholder API is added.
   Public `STAB-6103` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md:58-67` names stability states but does not define a
  public protocol or consumer behavior.
- `docs/status/STAB-6103-AUTHORITY-AUDIT.md` records missing identity,
  lifecycle, schema, compatibility, diagnostic, and cross-tool authority.
- The accepted GOV-0109 internal registry already verifies status against
  traceability and the `1.0-draft` support matrix without creating a public
  command.
- `DEC-0217` preserves truthful omission/rejection of plan-only CLI entry
  points, including `features` and `support`.

## Conformance plan

- Assert all sixty local categories, explicit order, duplicate rejection, and
  order-independent opaque bytes.
- Assert the exact current-state and stability vocabularies and reject
  cross-domain values.
- Verify the internal fixture remains unimplemented and non-public through the
  existing status gate.
- Defer public schema, lifecycle transitions, consumers, compatibility, and
  diagnostics until Accepted authority exists.

## Compatibility impact

Accepted language/runtime behavior, current CLI, diagnostics, schemas,
Semantic IDs, source spans, support states, dependencies, and Unicode 17.0.0
remain unchanged. Only internal validation and test evidence is added.

## Unresolved alternatives

Public feature/profile/target identities; canonical source of truth; lifecycle
transition and ownership rules; public schema and version; unknown/missing/
conflicting fields and states; reader/writer/N-1/migration policy; CLI, build,
package, LSP, documentation and Zed consumers; diagnostics; compatibility and
release binding; positive, negative, malformed, migration, repeated-build,
Unicode, source-span and cross-tool fixtures remain open under STAB-6103,
STAB-6102, incomplete G1-G5 exits, ROADMAP-1.0, the draft SUPPORT-MATRIX, and
registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
