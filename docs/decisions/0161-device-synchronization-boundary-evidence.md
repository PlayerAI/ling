# DEC-0161: Internal Device synchronization boundary evidence / 内部 Device Synchronization 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: synchronization-quality  
> 相关规范/缺口：`DEC-0160` | `DEC-0159` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-OWNERSHIP-MODEL-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DBUF-4404-OBSERVATION` Device synchronization boundary. It records
provisional queue, event/fence, await/barrier, ordering/visibility, hazard,
cancellation, cleanup, and device-loss vocabulary while Device, ownership, and
runtime authorities remain unresolved.

本决定只授权 `DBUF-4404-OBSERVATION` 使用 test-local 的拟议 Device synchronization 边界清单，
在 Device、ownership 与 runtime 权威尚未解决时，只记录临时 queue、event/fence、await/barrier、ordering/visibility、hazard、cancellation、cleanup 与 device-loss 词汇。

## Question

DBUF-4404 proposes command queues, events/fences, host await, device
barriers, cross-queue ordering, buffer hazards, cancellation, and device loss.
Which planning vocabulary can be retained as bounded evidence without defining
queue state, memory ordering, hazard proofs, lifecycle, or runtime behavior?

## Decision

1. `crates/ling-types/tests/device_synchronization_evidence.rs` keeps a
   test-local inventory of sixty provisional queue, event/fence, await/barrier,
   ordering/visibility, hazard, lifecycle, Typed Core, diagnostic, fixture,
   host-exclusion, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.device-synchronization-observation/0`. They are
   not queues, events, fences, barriers, happens-before facts, hazard proofs,
   cancellation results, Faults, diagnostics, Semantic IDs, public protocols,
   or backend support claims.
3. No queue/event/fence type, barrier or hazard checker, host-await or
   cancellation runtime, device-loss handler, dependency, diagnostic,
   protocol, or placeholder API is added. Public `DBUF-4404` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:272-283` is
  non-normative; Device synchronization behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  queue ownership, visibility/order, hazards, cleanup, determinism, Faults,
  and backends.
- Accepted Seed/VM runtime decisions do not authorize device queues or
  synchronization; RFC-0013/RFC-H401 and the plan's dependencies are not
  Accepted synchronization authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer queue/event/fence identity, await/barrier scope, cross-queue ordering,
  visibility, hazard proofs, cancellation/device loss, diagnostics, migration,
  and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no synchronization runtime, hazard decision, Fault, or support claim
exists.

## Unresolved alternatives

Queue/event/fence identity and capability; submission ownership and scope;
host await; barrier and cross-queue ordering; happens-before, visibility,
acquire/release, buffer hazards, subviews and transfer dependencies;
cancellation/timeout/device loss/Fault/drop/cleanup and committed effects;
canonical synchronization schema, diagnostics, migration, host/driver
redaction, protocol inventory, and public synchronization status remain open
under DBUF-4404, DBUF-4401 through DBUF-4403, KCHK-4101 through KCHK-4105,
GAP-KERNEL-DEVICE-001, GAP-OWNERSHIP-MODEL-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
