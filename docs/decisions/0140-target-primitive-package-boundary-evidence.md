# DEC-0140: Internal Target Primitive Package boundary evidence / 内部 Target Primitive Package 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: target-runtime
> 相关规范/缺口：`DEC-0139` | `DEC-0138` | `DEC-0137` | `DEC-0128` | `DEC-0127` | `DEC-0124` | `DEC-0009` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`FFI-3604-OBSERVATION` Target Primitive Package and `lingabi` boundary. It
records vocabulary and deterministic ordering while package, trust, capability,
ABI, target, and TCB authorities remain unresolved.

本决定只授权 `FFI-3604-OBSERVATION` 使用 test-local 的拟议 Target Primitive Package 与
`lingabi` 边界清单。在 package、trust、capability、ABI、target 与 TCB 权威尚未解决时，
只记录词汇和确定性顺序。

## Question

FFI-3604 proposes a trusted target package with `package.toml`,
`primitives.lingabi`, implementation files, proof/tests, capabilities, and a
TCB declaration. Which planning vocabulary can be retained as bounded evidence
without creating a package format, selecting an admission authority, or making
target primitives executable?

FFI-3604 计划提供包含 `package.toml`、`primitives.lingabi`、implementation files、
proof/tests、capabilities 与 TCB declaration 的 trusted target package。在不创建 package
format、不选择 admission authority、不让 target primitive 可执行的前提下，哪些规划词汇
可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/target_primitive_package_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering target and
   package identity/version/source, manifest/`lingabi` schemas, primitive
   identity/signature/layout, capability/profile/target availability,
   dependency/lock/canonical/unknown/migration rules, implementation language
   and unsafe boundary, trust/signature/TCB/proof, compiler/backend/runtime
   assumptions, license/revocation/update, artifact/shim/toolchain/offline and
   deterministic inputs, Semantic-ID/source-span views, ABI/calling/ownership/
   borrow/Resource/Managed/thread/reentry/blocking/Error/Fault/bounds rules,
   capability/profile/target rejection, unsupported primitives, diagnostics,
   Unicode, sanitizer/fuzz, cross-target evidence, public-protocol exclusion,
   and Seed compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.target-primitive-observation/0`. These bytes are not a target package,
   `lingabi` schema, primitive signature, capability grant, trust/TCB decision,
   proof, artifact, Semantic ID, diagnostic, or public protocol.
3. The child adds no target directory, package manifest, `lingabi` reader,
   primitive registry, capability/TCB checker, target selector, proof verifier,
   build integration, dependency, toolchain, diagnostic, protocol, or
   placeholder API. Public `FFI-3604` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its Target Primitive checklist
  cannot define package identity, target selection, `lingabi` fields, primitive
  signatures, capability semantics, proof acceptance, TCB membership, or update/
  revocation behavior.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` reserve future Target Primitive
  and trusted FFI boundaries but do not authorize a target package or executable
  primitive for the v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, `PROTO-ABI`, and `PROTO-EVIDENCE` are not Accepted authorities.
- Accepted `DEC-0139`, `DEC-0138`, `DEC-0137`, and `DEC-0128` authorize only
  test-local vocabulary; they do not supply package, trust, or TCB semantics.

## Conformance plan

- Assert all sixty provisional Target Primitive Package boundaries and their
  test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep package/`lingabi` schemas, discovery/locking, target/profile selection,
  primitive lowering, capability/TCB admission, proof/test verification,
  provenance/revocation, diagnostics, migration, sanitizer/fuzz, and
  cross-target behavior deferred until the required authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No target package, `lingabi`,
  capability, TCB, target, diagnostic, dependency, protocol, or support claim
  is registered.

## Unresolved alternatives

Target/package identity and versioning; manifest/`lingabi` schema and migration;
primitive signature/layout/ABI; capability/profile/target availability;
dependency/lock/artifact identity; implementation language and unsafe boundary;
trust/signature/TCB/proof/compiler/backend/runtime/hardware assumptions;
license/revocation/update; toolchain/offline/deterministic builds; Semantic-ID
and source-span projection; ownership/borrow/Resource/Managed/thread/reentry/
blocking/Error/Fault/bounds; diagnostics, Unicode, sanitizer/fuzz, cross-target,
and public protocol rules remain open under FFI-3604, FFI-3605,
GAP-NATIVE-BACKEND-ABI-001, GAP-OWNERSHIP-MODEL-001,
GAP-OWNERSHIP-PUBLIC-LIFETIME-001, and missing RFC-N305/RFC-0007/RFC-0011
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
