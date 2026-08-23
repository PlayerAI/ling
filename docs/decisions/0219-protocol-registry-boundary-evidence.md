# DEC-0219: Internal Protocol Registry boundary evidence / 内部协议注册表边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: protocol governance
> 相关规范/缺口：`DEC-0218` | `ROADMAP-1.0` | `GAP-REGISTER` | `PROTOCOL-INVENTORY` | `SCHEMA-REGISTRY`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PROTO-6201-OBSERVATION`. It
protects `docs/governance/protocol-inventory.toml` as the repository's single
protocol inventory and records provisional registry vocabulary without
creating the plan's duplicate path or promoting any protocol.

本决定授权 `PROTO-6201-OBSERVATION` 使用有界证据，保护
`docs/governance/protocol-inventory.toml` 作为仓库唯一协议清单，并记录临时
注册表词汇；不创建计划中的重复路径，也不晋级任何协议。

## Question

Which protocol-registry facts can be enforced before G6 has Accepted universal
registry semantics and Stable per-protocol compatibility evidence?

## Decision

1. `docs/governance/protocol-inventory.toml` remains the only machine-readable
   protocol inventory. `docs/protocols/registry.toml` must remain absent unless
   a later Accepted migration decision replaces the current source atomically.
2. `tools/xtask/src/protocols.rs` verifies the machine source and generated
   report exist and that the lower-authority proposed path does not become a
   second registry.
3. `crates/ling-types/tests/protocol_registry_evidence.rs` records sixty
   test-local identity, lifecycle, policy, category, failure, linkage, and
   corpus boundaries with deterministic ordering and duplicate rejection.
4. Opaque bytes tagged `ling.protocol-registry-observation/0` are test evidence
   only; they are not registry bytes, a schema, a Semantic ID input, or a
   public protocol.
5. The current inventory remains 27 records: 21 current public, 1 internal,
   and 5 Future; current public states remain 11 Experimental, 10 Preview, and
   0 Stable. Future records retain empty versions and fixtures.
6. No owner-field semantics, universal schema, public reader/writer, migration
   tool, Stable promotion, diagnostic, CLI route, or new protocol is added.
   Public `PROTO-6201` remains `BlockedSpec`.

## Normative basis

- Root repository governance names `docs/governance/protocol-inventory.toml`
  as the single inventory for implemented and planned public protocols.
- `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:86-107` proposes a
  stale second path but is non-normative and cannot override that rule.
- `docs/status/PROTO-6201-AUTHORITY-AUDIT.md` records missing ownership,
  lifecycle, universal schema, compatibility, diagnostic, and corpus authority.
- Existing protocol and schema verifiers already fail closed on duplicate or
  missing IDs, invalid lifecycle claims, missing versions/markers/paths,
  unaccepted Preview/Stable authority, and generated drift.
- `DEC-0218` exposes no public feature-state protocol or consumer that would
  require a new registry entry.

## Conformance plan

- Assert all sixty local registry categories, explicit order, duplicate
  rejection, and order-independent opaque bytes.
- Assert the canonical inventory and report exist and the proposed duplicate
  registry path does not.
- Run protocol, schema, support, status, and governance gates without changing
  the current 27-record lifecycle counts.
- Defer owner semantics, Stable registry contracts, migrations, and universal
  compatibility corpora until Accepted authority and per-protocol evidence exist.

## Compatibility impact

Existing protocol records, versions, lifecycle states, public schemas,
canonical encodings, language/runtime behavior, diagnostics, CLI/LSP,
dependencies, Semantic IDs, source spans, and Unicode 17.0.0 remain unchanged.

## Unresolved alternatives

Registry owner identity and responsibility; stable registry identity/version;
public versus internal/planned lifecycle transitions; supersession; missing
and unknown fields; reader/writer/N-1/migration rules; canonical registry
encoding; size/depth/security limits; per-protocol Accepted authority,
compatibility, golden/corrupt corpus, diagnostics, release artifacts and
independent review remain open under PROTO-6201, PROTO-6202 through PROTO-6204,
incomplete G1-G5 exits, ROADMAP-1.0, Draft schema policy, and registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
