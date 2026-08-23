# DEC-0211: Internal Reproducible Build Binding boundary evidence / 内部可重复构建绑定边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0210` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE` | `PROTO-BUILD-METADATA`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`EVD-5803-OBSERVATION`. It records provisional manifest, environment,
identity, artifact, nondeterminism, failure, and fixture vocabulary while
RFC-K507, build identity, artifact equivalence, and evidence protocols remain
unresolved.

本决定只授权 `EVD-5803-OBSERVATION` 使用 test-local 的 manifest、environment、
identity、artifact、nondeterminism、failure 与 fixture 边界清单；在 RFC-K507、
build identity、artifact equivalence 和 evidence protocol 尚未解决时，只记录
临时词汇，不声明已实现可重复构建。

## Question

EVD-5803 proposes rebuilding from an Evidence Bundle manifest in a controlled
environment and comparing source/Semantic IDs, object/binary hashes, accepted
documented nondeterminism, and generated-source/proof provenance. Which
vocabulary can be retained as bounded evidence without defining a hermetic
input closure, artifact identity, equivalence relation, or public protocol?

## Decision

1. `crates/ling-types/tests/reproducible_build_binding_evidence.rs` keeps a
   test-local inventory of sixty provisional manifest/environment, identity,
   artifact/provenance, nondeterminism/exclusion, result/failure, diagnostic,
   and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.reproducible-build-binding-observation/0`. These
   bytes are observation evidence only; they are not a build manifest,
   artifact digest, equivalence result, diagnostic, protocol, or support
   claim.
3. Source and Semantic identity, object and binary hashes, controlled and
   hermetic environments, accepted nondeterminism, and repeated/cross-host
   builds remain distinct local categories. Their presence does not define
   those concepts or prove that any build is reproducible.
4. No build runner, sandbox/container integration, artifact producer,
   normalization rule, nondeterminism registry, CLI/LSP route, diagnostic
   allocation, dependency, public protocol, support claim, or placeholder API
   is added. Public `EVD-5803` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:557-564` is a
  non-normative checklist. It defines no hermetic closure, environment,
  artifact identity, hash domain, equivalence relation, or nondeterminism
  policy.
- `docs/status/EVD-5803-AUTHORITY-AUDIT.md` records the absent RFC-K507,
  Evidence Bundle/build-metadata schemas, artifact comparison rules,
  provenance linkage, diagnostics, and executable fixtures.
- `PROTO-EVIDENCE` and `PROTO-BUILD-METADATA` are Planned public/Future, and
  `GAP-CRITICAL-PROFILE-001` remains open; none is implementation authority.
- Accepted Semantic/package/lock identities and internal query determinism
  have narrower scopes. They do not define object/binary artifact identity or
  cross-environment reproducibility.
- `DEC-0210` authorizes only test-local independent-verifier vocabulary; it
  does not define build bytes, manifests, environments, or claims that could
  be verified.

## Conformance plan

- Assert all sixty reproducible-build-binding categories and local order;
  compare forward/reverse opaque bytes; reject duplicates; retain source and
  Semantic IDs, object and binary hashes, hermetic-build and accepted-
  nondeterminism boundaries together.
- Defer controlled rebuilds, artifact equivalence, nondeterminism policy,
  diagnostics, protocols, and public support until Accepted authority and
  offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing identity, bytecode round-trip, cache, scheduler, and build
evidence is not reinterpreted as reproducible-build proof; only test-local
boundary evidence is added.

## Unresolved alternatives

Versioned hermetic manifests and canonical bytes; complete source/dependency/
toolchain/target/profile/environment/TCB input closure; object/binary/archive/
debug/symbol identity and hash domains; byte identity versus semantic
equivalence; path, timestamp, archive-order, metadata, and generated-input
normalization; accepted nondeterminism registry, reason, scope, and comparison;
generated source/proof provenance; repeated clean/warm-cache and cross-process/
host builds; offline/network isolation; stale/missing/hash/identity/
nondeterminism/malformed/corrupt/version/migration failures; bilingual stable
diagnostics and exits; positive, negative, repeated-build, cross-host,
corruption, Unicode 17.0.0, BOM/CRLF, and provenance fixtures; protocol
inventory and public support remain open under EVD-5803, EVD-5801, EVD-5802,
EVD-5804, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE,
PROTO-BUILD-METADATA, and missing reproducible-build authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
