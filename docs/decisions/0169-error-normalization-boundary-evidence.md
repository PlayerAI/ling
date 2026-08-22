# DEC-0169: Internal error-normalization boundary evidence / 内部错误归一化边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: diagnostics-quality  
> 相关规范/缺口：`DEC-0168` | `DEC-0167` | `DEC-0013` | `DEC-0001` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`GPU-4605-OBSERVATION` error-normalization boundary. It records provisional
Fault provenance, category, precedence, retry/cancellation, structured facts,
diagnostic registry, vendor-detail, redaction, fixture, and protocol
vocabulary while GPU/Kernel/Device runtime and diagnostic authorities remain
unresolved.

本决定只授权 `GPU-4605-OBSERVATION` 使用 test-local 的拟议错误归一化边界清单，
在 GPU/Kernel/Device runtime 与 diagnostic 权威尚未解决时，只记录临时 Fault provenance、category、precedence、retry/cancellation、structured facts、diagnostic registry、vendor-detail、redaction、fixture 与 protocol 词汇。

## Question

GPU-4605 proposes stable categories for unsupported features, unavailable
devices, compilation and launch failures, device memory exhaustion, device
loss, and unsupported numeric modes, while retaining vendor detail only as a
non-stable supplement. Which planning vocabulary can be retained as bounded
evidence without allocating public diagnostics or defining Fault semantics?

## Decision

1. `crates/ling-types/tests/error_normalization_evidence.rs` keeps a
   test-local inventory of sixty provisional Fault provenance, category,
   precedence, retry/cancellation, diagnostic registry, vendor-detail,
   redaction, fixture, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.error-normalization-observation/0`. They are not
   GPU error categories, Fault mappings, diagnostic codes, structured payloads,
   vendor parsers, public protocols, or support claims.
3. No GPU error category, public code, vendor-log parser, Fault mapper,
   dependency, diagnostic schema, protocol, or placeholder API is added.
   `docs/ERROR-CODES.md` and `error-code-lock.toml` remain the only allocation
   authorities. Public `GPU-4605` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:394-408` is
  non-normative; category labels do not define provenance, precedence,
  structured facts, severity, retry/cancellation, localization, versioning,
  or code allocation.
- `docs/ROADMAP-1.0.md:381-431` requires Device Fault mapping and explicit
  unsupported-target behavior but does not authorize GPU diagnostics.
- `docs/decisions/0013-main-and-runtime-failures.md` governs current Seed
  failure boundaries; `docs/ERROR-CODES.md` and
  `docs/governance/error-code-lock.toml` remain the sole public allocation and
  compatibility sources and do not define GPU categories.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4604 remain
  `BlockedSpec`; RFC-0013 and RFC-H404 are not Accepted. The Kernel/device and
  Native/backend gaps remain open.

## Conformance plan

- Assert all sixty normalization boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Fault taxonomy, category precedence, code allocation, vendor parsing,
  structured facts, redaction, migration, diagnostics, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no GPU error category, diagnostic code, Fault mapper, vendor parser,
or public protocol claim exists.

## Unresolved alternatives

Fault taxonomy and provenance across source/verifier/compiler/runtime/backend/
device/resource/cancellation/host; category precedence for UnsupportedFeature,
DeviceUnavailable, CompileFailure, LaunchFailure, OutOfDeviceMemory,
DeviceLost, NumericModeUnsupported; retryability, cancellation and severity;
source spans, UTF-8, Semantic IDs, structured facts and bilingual rendering;
error-code registry/lock, redaction and vendor detail, unknown vendor events;
numeric/capability mismatch, malformed modules, queue/synchronization/cleanup
failures, migration and determinism; positive/negative/malformed/corrupt-
vendor/bilingual/source-map/Unicode/redaction/determinism/migration/cross-
backend fixtures; diagnostics, host/driver/path/address/timestamp/debug
exclusion, protocol inventory, and public normalization status remain open
under GPU-4605, GPU-4601 through GPU-4604, DIR-4501 through DIR-4503,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
