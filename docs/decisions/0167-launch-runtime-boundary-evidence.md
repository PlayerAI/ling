# DEC-0167: Internal launch and runtime boundary evidence / 内部 launch 与 runtime 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: runtime-quality  
> 相关规范/缺口：`DEC-0166` | `DEC-0165` | `DEC-0164` | `DEC-0163` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`GPU-4603-OBSERVATION` launch and runtime boundary. It records provisional
discovery, capability matching, module loading, binding, launch, queue,
synchronization, Fault, cleanup, metrics, explain, fixture, diagnostic, and
redaction vocabulary while Device IR, adapter, runtime, and backend
authorities remain unresolved.

本决定只授权 `GPU-4603-OBSERVATION` 使用 test-local 的拟议 launch 与 runtime 边界清单，
在 Device IR、adapter、runtime 与 backend 权威尚未解决时，只记录临时 discovery、capability matching、module loading、binding、launch、queue、synchronization、Fault、cleanup、metrics、explain、fixture、diagnostic 与 redaction 词汇。

## Question

GPU-4603 lists device discovery, capability matching, module loading, buffer
binding, launch dimensions, queue submission, synchronization, device loss,
cleanup, and metrics/explain output. Which planning vocabulary can be retained
as bounded evidence without defining a runtime scheduler, resource guarantee,
metrics protocol, or device-loss semantics?

## Decision

1. `crates/ling-types/tests/launch_runtime_evidence.rs` keeps a test-local
   inventory of sixty provisional runtime, discovery, capability, module,
   buffer, launch, queue, synchronization, Fault, cleanup, metrics, explain,
   fixture, diagnostic, and redaction boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.launch-runtime-observation/0`. They are not
   runtime APIs, schedulers, device identities, capability records, resource
   handles, diagnostics, public protocols, or support claims.
3. No runtime, scheduler, discovery API, module loader, buffer/queue handle,
   dependency, target package, metrics schema, diagnostic, protocol, or
   placeholder API is added. Public `GPU-4603` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:365-376` is
  non-normative; its runtime phases do not define discovery identity,
  capability matching, module format, ownership, queue ordering,
  synchronization, device loss, cleanup, metrics, or explain semantics.
- `docs/ROADMAP-1.0.md:381-431` requires a supported GPU path with transfer,
  launch, synchronization, Fault mapping, differential evidence,
  fallback/rejection, and an explicit support matrix; it does not authorize a
  runtime API.
- DIR-4501 through DIR-4503 and GPU-4601/4602 remain `BlockedSpec`; RFC-0013
  and RFC-H404 are not Accepted. `GAP-KERNEL-DEVICE-001` plus
  `GAP-NATIVE-BACKEND-ABI-001` remain open, and `BACKEND-GPU` is
  Unsupported/Experimental in the support matrix.

## Conformance plan

- Assert all sixty launch/runtime boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer discovery, capability matching, module loading, binding, launch,
  queue/synchronization, device loss, cleanup, metrics/explain, diagnostics,
  and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no device runtime, scheduler, resource, target, or public protocol
claim exists.

## Unresolved alternatives

Runtime and ABI boundary; device discovery and stable identity; capability
matching and feature versions; module loading and binary validation; buffer
binding/layout/ownership; launch dimensions and workgroup/grid limits;
queue submission/order, synchronization and visibility; cancellation, device
loss, Fault, cleanup on success/Error/Fault, resource budgets; metrics/explain
stability, replayability and diagnostic-only fields; host/vendor isolation,
target/toolchain/driver identity, numeric/determinism, source maps, UTF-8
spans and Semantic IDs; unsupported hardware, fallback/rejection; positive,
negative, discovery, capability, corruption, binding, launch, synchronization,
device-loss, cleanup, cancellation, resource, Unicode, migration, and
differential fixtures; diagnostics, host/driver/path/address/timestamp/debug
redaction, protocol inventory, and public runtime status remain open under
GPU-4603, GPU-4601/4602, DIR-4501 through DIR-4503,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
