# DEC-0179: Internal Profile Composition boundary evidence / 内部 Profile 组合边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0178` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-SEMANTIC-HASH-LIFECYCLE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROF-5103-OBSERVATION`. It records provisional profile layers, composition
operators, conflicts, canonicalization, identity, configuration, diagnostics,
and fixture vocabulary while RFC-K501/RFC-0012 and Semantic ID migration
authority remain unresolved.

本决定只授权 `PROF-5103-OBSERVATION` 使用 test-local 的 Profile 组合边界清单；
在 RFC-K501/RFC-0012 与 Semantic ID 迁移权威尚未解决时，只记录临时的 profile layer、组合操作、
冲突、canonicalization、identity、配置、diagnostic 与 fixture 词汇。

## Question

PROF-5103 proposes controlled composition of a base profile, target profile,
and mission constraints, with explicit conflicts and an effective profile that
participates in build identity and Semantic ID. Which vocabulary can be retained
as bounded evidence without selecting an algebra, schema, precedence, or
identity migration?

## Decision

1. `crates/ling-types/tests/profile_composition_evidence.rs` keeps a test-local
   inventory of sixty provisional composition boundaries covering layers,
   policies, merge choices, conflicts, canonical identity, configuration,
   diagnostics, and offline fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.profile-composition-observation/0`. These bytes
   are evidence only; they are not a profile schema, merge algorithm, digest,
   identity input, diagnostic, protocol, or support claim.
3. No profile composition API, effective-profile schema, profile digest,
   Program ID change, dependency, diagnostic, CLI option, protocol, support
   claim, or placeholder API is added. Public `PROF-5103` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:122-124` is non-normative;
  it defines no profile schema, layer scope, merge algebra, precedence,
  conflict class, canonical encoding, or identity migration.
- `docs/ROADMAP-1.0.md:145-149` identifies future artifact and Semantic Graph/
  ID compatibility surfaces but does not authorize composition or identity
  changes.
- Accepted DEC-0012 fixes Seed Definition/Body/Program ID domains and
  versioned canonical bytes; profile inputs require a separately Accepted
  Semantic Schema/ID migration.
- `docs/status/PROF-5103-AUTHORITY-AUDIT.md` records the open Critical Profile
  and Semantic Hash lifecycle gaps and the absent RFC-K501/RFC-0012 authority.

## Conformance plan

- Assert all sixty profile-composition boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer profile schema, merge/precedence semantics, conflict diagnostics,
  effective-profile serialization, build/cache/Program ID integration, CLI,
  and protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing `ling-db` profile/target workspace-input cache dimensions
are not reinterpreted as Profile Composition semantics; only test-local
evidence is added.

## Unresolved alternatives

Profile schema/version/lifecycle; base/target/mission scope; field presence,
defaults, unknown fields; merge operators, precedence, override/intersection/
subtraction/monotonicity; conflict classes and incompatible targets/packages;
impossible constraints; effective-profile scope and canonical bytes; digest,
build/cache/artifact/replay identity; Program ID and Semantic Graph relation;
configuration precedence; source visibility; migration and compatibility;
bilingual diagnostics and structured facts; positive/negative/layer-order/
conflict/identity-migration/cache-replay/differential/Unicode fixtures; protocol
inventory and public status remain open under PROF-5103, PROF-5102,
GAP-CRITICAL-PROFILE-001, GAP-SEMANTIC-HASH-LIFECYCLE-001, and missing
RFC-K501/RFC-0012 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
