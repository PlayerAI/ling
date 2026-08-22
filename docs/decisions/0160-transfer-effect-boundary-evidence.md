# DEC-0160: Internal Transfer Effect boundary evidence / 内部 Transfer Effect 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: transfer-quality  
> 相关规范/缺口：`DEC-0159` | `DEC-0158` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-OWNERSHIP-MODEL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DBUF-4403-OBSERVATION` Transfer Effect boundary. It records provisional
transfer syntax, effect/capability, bytes/address spaces, ownership, lifecycle,
Fault, cost, diagnostic, and audit vocabulary while Device, ownership, and
synchronization authorities remain unresolved.

本决定只授权 `DBUF-4403-OBSERVATION` 使用 test-local 的拟议 Transfer Effect 边界清单，
在 Device、ownership 与 synchronization 权威尚未解决时，只记录临时 transfer syntax、effect/capability、bytes/address spaces、ownership、lifecycle、Fault、cost、diagnostic 与 audit 词汇。

## Question

DBUF-4403 proposes an explicit transfer expression and Semantic Graph/Audit
facts for bytes, source/destination address spaces, synchronization, possible
Fault, and `Capability<DeviceTransfer>`. Which planning vocabulary can be kept
as bounded evidence without defining transfer typing, effect rows, ownership
transitions, lifecycle, or cost semantics?

## Decision

1. `crates/ling-types/tests/transfer_effect_evidence.rs` keeps a test-local
   inventory of sixty provisional transfer syntax, effect/capability,
   address-space, byte/layout, ownership, synchronization/lifecycle, Fault,
   cost, Typed Core, diagnostic, fixture, host-exclusion, and protocol
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.transfer-effect-observation/0`. They are not
   transfer expressions, Effect Rows, capabilities, ownership transitions,
   synchronization tokens, Faults, diagnostics, Semantic IDs, public
   protocols, or backend support claims.
3. No transfer operation, effect/capability API, address-space model,
   lifecycle runtime, cost reporter, dependency, diagnostic, protocol, or
   placeholder API is added. Public `DBUF-4403` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:252-270` is
  non-normative; Transfer Effect behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  Device/capability discovery, address spaces, ownership transitions,
  synchronization, Faults, determinism, and backends.
- Existing Seed Effect/Fault decisions do not authorize DeviceTransfer or
  device execution; RFC-0013/RFC-H401 and the plan's dependencies are not
  Accepted transfer authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer transfer syntax/type/effect semantics, capability identity, bytes and
  layout, ownership transitions, lifecycle/cancellation, Fault/cost fields,
  diagnostics, migration, and protocol behavior until accepted authority
  exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no transfer operation, effect, capability, Fault, cost, or support
claim exists.

## Unresolved alternatives

Transfer syntax/result and Effect Row identity; DeviceTransfer capability
discovery/version/direction; logical byte count, address spaces, shape/layout,
alignment/bounds; copy/move/borrow/view ownership transitions; token,
synchronization, visibility/coherence, asynchronous completion, cancellation,
timeout, device loss/Fault/drop, resource limits, cost/evidence status;
canonical Semantic Graph/Audit fields, diagnostics, migration, host/driver
redaction, protocol inventory, and public Transfer Effect status remain open
under DBUF-4403, DBUF-4401/4402, KCHK-4101 through KCHK-4105,
GAP-KERNEL-DEVICE-001, GAP-OWNERSHIP-MODEL-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
