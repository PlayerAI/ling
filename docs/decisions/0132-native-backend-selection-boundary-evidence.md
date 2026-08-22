# DEC-0132: Internal Native backend selection boundary evidence / 内部 Native Backend 选择边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0131` | `DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Native
backend-selection comparison boundary for the bounded
`BACK-3501-OBSERVATION` child. It records candidate and evidence vocabulary
without selecting a backend, installing a toolchain, running benchmarks, or
claiming target/support compatibility.

本决定只授权 test-only 的拟议 Native backend 选择比较边界清单，供
`BACK-3501-OBSERVATION` 子任务使用。它记录 candidate 与 evidence 词汇，但不选择 backend、不安装 toolchain、不运行 benchmark，
也不声明 target/support compatibility。

## Question

BACK-3501 proposes an isolated comparison of Cranelift, LLVM, and an optional
small C/Wasm transition backend using build time, debug information, target
coverage, JIT/AOT, license, and reproducible-build dimensions. Which planning
vocabulary can be retained while RFC-N306 and the NIR/ABI contracts remain
unaccepted?

BACK-3501 提议在 RFC-N306 前隔离比较 Cranelift、LLVM 与可选的最小 C/Wasm 过渡后端，维度包括 build time、debug info、target coverage、
JIT/AOT、license 与 reproducible build。在 RFC-N306 以及 NIR/ABI 契约尚未 Accepted 时，哪些规划词汇可以保留？

## Decision

1. `crates/ling-types/tests/native_backend_selection_evidence.rs` keeps a
   test-local inventory of fifty-four provisional boundaries: candidate names
   and comparison-only scope; NIR/ABI/target/profile/Core eligibility;
   toolchain, compiler, target, flags, standard library, linker, and runtime
   inputs; cold/warm build and resource bounds; debug/source maps, JIT/AOT, and
   target coverage; license, supply chain, TCB, offline lock, generated code,
   reproducibility, environment and host-noise exclusions; deterministic
   corpus/metrics and cross-target evidence; semantic, ABI/FFI, Fault,
   Resource/Managed, Task/Actor, artifact/review, recommendation, migration,
   Seed, Unicode, differential, security, and toolchain-optionality boundaries.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-backend-selection-observation/0`. These bytes are not a
   backend choice, dependency declaration, benchmark, target claim, build
   recipe, license approval, public protocol, or performance/reproducibility
   result.
3. The child adds no backend dependency, toolchain, build script, code
   generator, benchmark corpus, target support entry, public API, or
   placeholder crate. Public `BACK-3501` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its candidate list and comparison
  dimensions cannot define an eligible NIR, ABI, target, profile, or toolchain
  contract.
- RFC-N306 is absent or not Accepted. RFC-0001 remains Draft under DEC-0018
  and treats LLVM/Cranelift Native Backend as a non-goal for the Seed release.
- Accepted `DEC-0131`, `DEC-0130`, and `DEC-0129` define only test-local
  verifier/lowering/design vocabulary. They do not authorize Native codegen,
  backend selection, or support claims.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Profile/Task/
  Actor gaps remain Open. The accepted Seed Typed Core/interpreter/VM boundary
  remains the only executable authority.

## Conformance plan

- Assert all fifty-four provisional backend-selection boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep backend probes, dependency/toolchain changes, benchmark metrics,
  license/TCB conclusions, target support, code generation, and public
  protocols deferred until their authorities and data-only artifact schema are
  Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No backend, target, dependency,
  benchmark, diagnostic, Semantic ID, build, or public protocol claim is
  registered.

## Unresolved alternatives

NIR/ABI/profile/target eligibility; candidate scope and backend operation
coverage; toolchain and runtime-library versions; comparison corpus, metrics,
resource limits, debug/source maps, JIT/AOT, license/TCB/supply chain, offline
and generated-code policy; reproducibility and host-noise exclusions;
semantic/ABI/FFI/Fault/Resource/Managed/Task/Actor preservation; artifact and
review lifecycle; recommendation, migration, security, differential, and
support semantics remain open under `BACK-3501`, `NIR-3403`, `NIR-3402`,
`NIR-3401`, `GC-3304`, `GC-3303`, `GC-3302`, `GC-3301`,
`GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and missing
RFC-N306/RFC-N304/RFC-N303/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
