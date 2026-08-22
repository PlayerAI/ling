# DEC-0133: Internal Native codegen boundary evidence / 内部 Native Codegen 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0132` | `DEC-0131` | `DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed baseline
Native codegen boundary for the bounded `BACK-3502-OBSERVATION` child. It
records emission and artifact vocabulary without defining a machine target,
ABI, object format, linker, diagnostic, or build protocol.

本决定只授权 test-only 的拟议 baseline Native codegen 边界清单，供
`BACK-3502-OBSERVATION` 子任务使用。它记录 emission 与 artifact 词汇，但不定义 machine target、ABI、object format、linker、diagnostic，
也不定义 build protocol。

## Question

BACK-3502 proposes one backend for target-machine selection, function/data
emission, relocations, runtime linking, object/executable output, debug/source
maps, deterministic metadata, and explicit unsupported diagnostics. Which
planning vocabulary can be retained while the NIR, Native ABI, backend,
memory/profile, and reproducible-artifact authorities remain unaccepted?

BACK-3502 提议由一个 backend 覆盖 target-machine、function/data emission、relocation、runtime linking、object/executable、debug/source map、
deterministic metadata 与 explicit unsupported diagnostic。在 NIR、Native ABI、backend、memory/profile 与可复现 artifact 权威尚未 Accepted 时，
哪些规划词汇可以保留？

## Decision

1. `crates/ling-types/tests/native_codegen_evidence.rs` keeps a test-local
   inventory of fifty-eight provisional boundaries: target/profile/data layout
   and endianness; function/data/closure/ADT/string emission and
   Value/Resource/Managed/calling/Fault/Task/Actor/FFI representation;
   relocation/link/runtime/object/executable/section/symbol artifacts;
   debug/source identity and deterministic ordering; unsupported forms and
   bilingual source-byte diagnostics; verified NIR/ABI/profile inputs,
   allocation/cleanup, cross-target and ABI evidence; reproducibility,
   semantic/differential/sanitizer/security evidence; license/offline,
   artifact/migration/Seed/Unicode boundaries; host-output, timestamp/address,
   map-order, malformed-input, toolchain, and resource exclusions.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-codegen-observation/0`. These bytes are not machine code,
   object/executable output, relocation, linker input, target claim,
   diagnostic, build recipe, public protocol, or semantic-preservation proof.
3. The child adds no code generator, object format, linker integration, target
   manifest, diagnostic, build command, dependency, toolchain, public API, or
   placeholder crate. Public `BACK-3502` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its codegen checklist cannot
  define target, layout, ABI, object, linker, debug, diagnostic, or build
  semantics.
- Accepted `DEC-0132` and `DEC-0131` define only test-local backend-selection
  and verifier vocabulary. They do not select a backend or authorize code
  emission.
- RFC-N304/RFC-N306 and candidate RFC-0011 are absent or not Accepted.
  RFC-0001 remains Draft under DEC-0018 and excludes an LLVM/Cranelift Native
  Backend from the Seed release.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Profile/
  Critical/Task/Actor gaps remain Open. Accepted Seed Typed Core/interpreter/VM
  decisions do not define machine artifacts.

## Conformance plan

- Assert all fifty-eight provisional codegen boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep code emission, target/layout/ABI/object/link/runtime behavior,
  unsupported diagnostics, debug/source maps, reproducibility, and public
  build/support protocols deferred until their authorities are Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No native artifact, target, ABI,
  diagnostic, build, dependency, Semantic ID, or public protocol claim is
  registered.

## Unresolved alternatives

Target/profile/data-layout/endianness and machine representation; function,
closure, ADT, string, Value/Resource/Managed, calling/Fault/Task/Actor/FFI
ABI; relocation/link/runtime/object/executable formats; debug/source identity;
deterministic metadata; unsupported diagnostics; NIR/ABI/profile verification;
allocation/cleanup; cross-target, reproducible, semantic/differential,
sanitizer/security, license/offline, artifact/migration, malformed-input,
toolchain, and public-support semantics remain open under `BACK-3502`,
`BACK-3501`, `NIR-3403`, `NIR-3402`, `NIR-3401`, `GC-3304`, `GC-3303`,
`GC-3302`, `GC-3301`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-OWNERSHIP-MODEL-001`, and missing RFC-N306/RFC-N304/RFC-N303/RFC-0007
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
