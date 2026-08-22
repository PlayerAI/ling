# DEC-0165: Internal backend spike and selection boundary evidence / 内部 backend spike 与选择边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: backend-quality  
> 相关规范/缺口：`DEC-0164` | `DEC-0163` | `DEC-0157` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`GPU-4601-OBSERVATION` backend spike and selection boundary. It records
candidate technologies, verified-artifact, target/capability, evaluation,
support-status, fixture, diagnostic, and host-exclusion vocabulary while
Kernel, Device IR, runtime, ABI, and backend authorities remain unresolved.

本决定只授权 `GPU-4601-OBSERVATION` 使用 test-local 的拟议 backend spike 与选择边界清单，
在 Kernel、Device IR、runtime、ABI 与 backend 权威尚未解决时，只记录候选技术、verified-artifact、target/capability、评估、support-status、fixture、diagnostic 与 host-exclusion 词汇。

## Question

GPU-4601 lists SPIR-V/Vulkan, WGSL/WebGPU, CUDA/PTX, an MLIR bridge, and
vendor SDKs together with platform, API, source/debug, numeric, hardware,
license, reproducibility, and maintenance criteria. Which planning vocabulary
can be retained as bounded evidence without selecting a backend or creating a
target, dependency, ABI, or support protocol?

## Decision

1. `crates/ling-types/tests/backend_spike_selection_evidence.rs` keeps a
   test-local inventory of sixty provisional candidate, artifact,
   target/capability, evaluation, support-status, fixture, diagnostic,
   redaction, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.backend-spike-selection-observation/0`. They are
   not backend selections, target identities, capability records, benchmarks,
   dependencies, diagnostics, public protocols, or support claims.
3. No backend crate, dependency, target package, toolchain probe, benchmark,
   adapter, capability API, diagnostic, protocol, or placeholder API is added.
   Public `GPU-4601` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:328-349` is
  non-normative; candidate paths and criteria do not select a backend.
- `docs/ROADMAP-1.0.md:381-431` requires G4 Kernel/Device gates, a verified
  backend-neutral artifact, runtime/transfer/synchronization/Fault semantics,
  differential evidence, and an explicit support matrix; it does not select a
  technology or authorize a public spike result.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` exclude GPU and Native behavior
  from the v0.0.1 Seed subset.
- RFC-0013 is not Accepted, RFC-H404 is absent, and
  `GAP-KERNEL-DEVICE-001` plus `GAP-NATIVE-BACKEND-ABI-001` remain open.
  `BACKEND-GPU` is Unsupported/Experimental in the support matrix.

## Conformance plan

- Assert all sixty backend spike and selection boundaries and local order;
  compare forward/reverse opaque bytes; reject duplicates.
- Defer candidate selection, target/capability identity, toolchain/driver
  probing, benchmarks, support-status transitions, ABI/runtime behavior,
  diagnostics, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no backend, target, dependency, toolchain, support, or public
protocol claim exists.

## Unresolved alternatives

Candidate path and version; verified input artifact and Device IR/Kernel
boundary; target identity/profile and capability discovery; required/optional
features, unsupported-target, fallback and rejection policy; platform/API,
source/debug, numeric/determinism, synchronization, transfer, Fault, resource,
launch and ABI behavior; toolchain/driver identity, hardware/CI, license,
reproducibility, cache identity, maintenance and benchmark evidence; CPU/GPU
differential and malformed/Unicode/migration fixtures; Experimental/Preview/
Supported transitions; diagnostics, timestamp/address/path/driver redaction,
protocol inventory, and public backend status remain open under GPU-4601,
DIR-4501 through DIR-4503, KCHK-4101 through KCHK-4105, CPU-4201 through
CPU-4203, SIMD-4301 through SIMD-4303, DBUF-4401 through DBUF-4404,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
