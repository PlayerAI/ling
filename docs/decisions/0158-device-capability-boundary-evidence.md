# DEC-0158: Internal Device capability boundary evidence / 内部 Device Capability 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: device-quality  
> 相关规范/缺口：`DEC-0157` | `DEC-0150` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-OWNERSHIP-MODEL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DBUF-4401-OBSERVATION` Device and capability boundary. It records provisional
device, address-space, buffer, view, transfer, synchronization, and raw-pointer
exclusion vocabulary while Kernel/device and ownership authorities remain
unresolved.

本决定只授权 `DBUF-4401-OBSERVATION` 使用 test-local 的拟议 Device 与 capability 边界清单，
在 Kernel/device 与 ownership 权威尚未解决时，只记录临时 device、address-space、buffer、view、transfer、synchronization 与 raw-pointer exclusion 词汇。

## Question

DBUF-4401 proposes DeviceId, DeviceKind, DeviceCapability, AddressSpace,
Buffer, views, TransferToken, Fence/Event, and a raw-device-pointer boundary.
Which planning vocabulary can be retained as bounded evidence without
defining device identity, ownership, capability discovery, or runtime APIs?

## Decision

1. `crates/ling-types/tests/device_capability_evidence.rs` keeps a test-local
   inventory of sixty provisional device, capability, address-space, buffer,
   view, transfer, synchronization, ownership, fixture, diagnostic, and
   protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.device-capability-observation/0`. They are not
   Device values, capabilities, ownership proofs, pointers, diagnostics,
   Semantic IDs, public protocols, or support claims.
3. No Device/Buffer type, capability registry, view/token API, Fence/Event
   runtime, raw-pointer interface, dependency, diagnostic, protocol, or
   placeholder API is added. Public `DBUF-4401` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:221-238` is
  non-normative; Device and capability behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  address spaces, buffers, synchronization, ownership, determinism,
  capability discovery, and backends.
- RFC-0013/RFC-H401 and the plan's Device dependencies are not Accepted
  authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer Device identity, capability discovery, address spaces, Buffer/views,
  transfer/synchronization, raw-pointer rejection, diagnostics, migration,
  differential, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Device API, capability, or support claim exists.

## Unresolved alternatives

Device identity/kind/capability/version/set; address spaces; Buffer element/
shape/layout/identity; views, tokens, Fence/Event; raw-pointer prohibition;
Typed Core/profiles/targets/effects/capabilities; ownership/alias/bounds/
resources; transfer direction/order, async lifetime, cancellation/drop,
device loss/Faults; canonical schema/version/migration, fixtures, CPU/device
differential, diagnostics, host exclusion, protocol inventory, and public
Device status remain open under DBUF-4401, SIMD-4301 through SIMD-4303,
KCHK-4101 through KCHK-4105, GAP-KERNEL-DEVICE-001,
GAP-OWNERSHIP-MODEL-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
