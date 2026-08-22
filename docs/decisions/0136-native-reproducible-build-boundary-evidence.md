# DEC-0136: Internal Native reproducible-build boundary evidence / 内部 Native 可复现构建边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0135` | `DEC-0134` | `DEC-0133` | `DEC-0132` | `DEC-0131` | `DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-SEMANTIC-HASH-LIFECYCLE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Native
reproducible-build boundary for the bounded `BACK-3505-OBSERVATION` child. It
records declared inputs, artifact comparison, provenance, and exclusion
vocabulary without pinning a toolchain, emitting an artifact, or claiming
byte-identical builds.

本决定只授权 test-only 的拟议 Native reproducible-build 边界清单，供
`BACK-3505-OBSERVATION` 子任务使用。它记录 declared input、artifact comparison、provenance 与 exclusion 词汇，但不 pin toolchain、不生成 artifact，
也不声明 byte-identical build。

## Question

BACK-3505 proposes controlling toolchain, target, linker, environment, paths,
timestamps, build IDs, dependency lock, and codegen options so equal
declaration inputs produce byte-identical artifacts or a manifest of
unavoidable differences. Which planning vocabulary can be retained while the
Native backend/artifact, security, and Semantic Hash lifecycle authorities
remain unaccepted?

BACK-3505 提议控制 toolchain、target、linker、environment、path、timestamp、build ID、dependency lock 与 codegen option，使相同声明输入生成
byte-identical artifact，或由 manifest 列出不可消除差异。在 Native backend/artifact、安全与 Semantic Hash lifecycle 权威尚未 Accepted 时，哪些规划
词汇可以保留？

## Decision

1. `crates/ling-types/tests/native_reproducible_build_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries: toolchain/linker/
   target/environment/library/options/lock inputs; source/Typed Core/NIR/
   Profile inputs; artifact format/identity/object/executable/debug/symbol
   bytes; path/timestamp/build-ID/section/symbol/compression/archive policy;
   versioned difference manifests and byte/manifest comparison; clean/repeat/
   offline/cross-host/cross-target builds; tampered/missing inputs, provenance,
   license/TCB, cache/release; separation from Semantic IDs/source spans/
   performance; resource bounds/deterministic metadata/host exclusions;
   Unicode/diagnostics/unsupported input/migration/security/differential/
   Seed/schema/optional-toolchain boundaries.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-reproducible-build-observation/0`. These bytes are not a
   toolchain pin, artifact manifest, byte-identical result, provenance record,
   release protocol, Semantic ID, diagnostic, build command, or support claim.
3. The child adds no build script, artifact manifest, path-remapping policy,
   toolchain pin, build-ID rule, target matrix, linker integration, diagnostic,
   dependency, toolchain, public protocol, or placeholder crate. Public
   `BACK-3505` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its reproducible-build checklist
  cannot define artifact identity, input closure, difference policy, or release
  guarantees.
- Accepted `DEC-0135` through `DEC-0131` define only test-local optimization,
  ABI, codegen, backend-selection, and verifier vocabulary. They do not
  authorize a build/release contract.
- RFC-N304/RFC-N306 and candidate RFC-0011 are absent or not Accepted.
  `GAP-NATIVE-BACKEND-ABI-001` and `GAP-SEMANTIC-HASH-LIFECYCLE-001` remain
  Open; Semantic IDs cannot be reused as reproducible-build identities.
- Accepted Seed decisions and current locked/offline Cargo rules do not
  establish byte-identical Native artifacts or a Native release protocol.

## Conformance plan

- Assert all sixty provisional reproducible-build boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep toolchain/target/linker pinning, input closure, artifact manifests,
  byte/difference comparison, provenance/license/offline policy, cross-host/
  target reproduction, diagnostics, and release claims deferred until their
  authorities are Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No build, artifact, provenance,
  Semantic ID, diagnostic, dependency, release, or public protocol claim is
  registered.

## Unresolved alternatives

Toolchain/linker/target/environment/library/options/lock input closure;
source/Core/NIR/Profile identity; artifact/object/executable/debug/symbol
identity; path/timestamp/build-ID/section/symbol/compression/archive policy;
versioned difference manifests; byte/manifest comparison; clean/repeat/offline/
cross-host/target reproduction; tamper/missing input; provenance/license/TCB;
cache/release; Semantic ID/source-span/performance separation; bounds,
metadata, host exclusions, Unicode, diagnostics, unsupported input, migration,
security, differential, schema, and toolchain optionality remain open under
`BACK-3505`, `BACK-3504`, `BACK-3503`, `BACK-3502`, `BACK-3501`, `NIR-3403`,
`NIR-3402`, `NIR-3401`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-SEMANTIC-HASH-LIFECYCLE-001`, and missing RFC-N306/RFC-N304/RFC-N303/
RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
