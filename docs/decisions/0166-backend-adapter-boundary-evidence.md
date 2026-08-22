# DEC-0166: Internal backend adapter boundary evidence / 内部 backend adapter 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: backend-quality  
> 相关规范/缺口：`DEC-0165` | `DEC-0164` | `DEC-0163` | `DEC-0157` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`GPU-4602-OBSERVATION` backend adapter boundary. It records provisional
compile, binary, allocation, transfer, launch, synchronization, capability,
Fault, lifecycle, isolation, diagnostic, and protocol vocabulary while Device
IR, runtime, ABI, and backend authorities remain unresolved.

本决定只授权 `GPU-4602-OBSERVATION` 使用 test-local 的拟议 backend adapter 边界清单，
在 Device IR、runtime、ABI 与 backend 权威尚未解决时，只记录临时 compile、binary、allocation、transfer、launch、synchronization、capability、Fault、lifecycle、isolation、diagnostic 与 protocol 词汇。

## Question

GPU-4602 sketches a narrow adapter for compiling verified Device IR, allocating
and transferring buffers, launching and synchronizing work, querying
capabilities, and mapping Faults. Which planning vocabulary can be retained as
bounded evidence without defining an adapter ABI, DeviceBinary schema,
runtime handles, or vendor contract?

## Decision

1. `crates/ling-types/tests/backend_adapter_evidence.rs` keeps a test-local
   inventory of sixty provisional input, binary, cache, allocation, transfer,
   launch, synchronization, capability, Fault, lifecycle, isolation, fixture,
   diagnostic, redaction, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.backend-adapter-observation/0`. They are not
   adapter traits, Device IR values, DeviceBinary records, runtime handles,
   capabilities, diagnostics, public protocols, or support claims.
3. No adapter trait, Device IR or DeviceBinary API, runtime handle, target
   package, dependency, capability API, diagnostic, protocol, or placeholder
   API is added. Public `GPU-4602` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:351-363` is
  non-normative; its operation sketch does not define the adapter ABI or
  backend behavior.
- `docs/ROADMAP-1.0.md:381-431` places adapter work after Kernel/Device gates
  and requires verified IR, transfers, launch, synchronization, Fault mapping,
  differential evidence, and an explicit support matrix.
- `DEC-0164` covers only test-local canonicalization vocabulary; it does not
  define adapter input, binary, cache, or target semantics.
- DIR-4501 through DIR-4503 remain `BlockedSpec`, RFC-0013 and RFC-H404 are
  not Accepted, and `GAP-KERNEL-DEVICE-001` plus
  `GAP-NATIVE-BACKEND-ABI-001` remain open. `BACKEND-GPU` is
  Unsupported/Experimental in the support matrix.

## Conformance plan

- Assert all sixty adapter boundaries and local order; compare forward/reverse
  opaque bytes; reject duplicates.
- Defer adapter ABI, DeviceBinary/cache schema, ownership/lifetime, transfer
  visibility, queue/launch/synchronization, capability negotiation, Fault and
  cleanup behavior, diagnostics, and protocol behavior until accepted
  authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no adapter, target, dependency, binary, runtime, or public protocol
claim exists.

## Unresolved alternatives

Adapter input trust boundary and verified Device IR; compile and target-spec
identity; DeviceBinary schema, canonical bytes, ownership, corruption,
migration, cache identity/invalidation; buffer allocation/ownership, transfer
visibility, queue ordering, launch dimensions, workgroup limits,
synchronization scope; capability query/identity, required/optional and
unsupported features, fallback/rejection; Fault classes, device loss,
cancellation, resource limits, cleanup; vendor/front-end isolation,
toolchain/driver identity, numeric/determinism, source maps, UTF-8 spans and
Semantic IDs; positive/negative/malformed/migration/lifecycle/differential
fixtures; diagnostics, host/driver/path/address/timestamp redaction, protocol
inventory, and public adapter status remain open under GPU-4602,
GPU-4601, DIR-4501 through DIR-4503, KCHK-4101 through KCHK-4105,
CPU-4201 through CPU-4203, SIMD-4301 through SIMD-4303, DBUF-4401 through
DBUF-4404, GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
