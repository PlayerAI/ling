# DEC-0159: Internal Buffer ownership boundary evidence / 内部 Buffer Ownership 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: ownership-quality  
> 相关规范/缺口：`DEC-0158` | `DEC-0150` | `ROADMAP-1.0` | `GAP-OWNERSHIP-MODEL-001` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DBUF-4402-OBSERVATION` Buffer ownership boundary. It records provisional
ownership, borrowing, view, mapping, transfer-lifetime, cleanup, and
task/actor-crossing vocabulary while ownership, Device Buffer, and concurrency
authorities remain unresolved.

本决定只授权 `DBUF-4402-OBSERVATION` 使用 test-local 的拟议 Buffer ownership 边界清单，
在 ownership、Device Buffer 与 concurrency 权威尚未解决时，只记录临时 ownership、borrow、view、mapping、transfer-lifetime、cleanup 与 task/actor-crossing 词汇。

## Question

DBUF-4402 proposes host/device ownership, Copy/Move and borrow states,
exclusive writes, shared reads, alias and subview proofs, mapping/pinning,
asynchronous cleanup, cancellation, and task/actor crossing. Which planning
vocabulary can be retained as bounded evidence without defining ownership,
alias, cleanup, or scheduling behavior?

## Decision

1. `crates/ling-types/tests/buffer_ownership_evidence.rs` keeps a test-local
   inventory of sixty provisional ownership, borrow, alias, subview, mapping,
   transfer-lifetime, cleanup, Fault, crossing, Typed Core, diagnostic,
   fixture, host-exclusion, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.buffer-ownership-observation/0`. They are not
   ownership states, borrow proofs, views, transfer tokens, cleanup events,
   diagnostics, Semantic IDs, public protocols, or backend support claims.
3. No ownership checker, Buffer/view API, mapping/pinning runtime,
   transfer-lifetime state machine, cancellation/drop behavior, dependency,
   diagnostic, protocol, or placeholder API is added. Public `DBUF-4402`
   remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:240-250` is
  non-normative; Buffer ownership behavior remains outside v0.0.1.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-KERNEL-DEVICE-001` remain Open for
  ownership calculus, address spaces, transfer, synchronization, cleanup,
  determinism, and backends.
- RFC-0013/RFC-H401 and the plan's ownership dependencies are not Accepted
  authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer ownership/borrow states, alias/subview proofs, mapping/pinning,
  transfer completion, cancellation/drop, crossing, diagnostics, migration,
  and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no ownership decision, view, cleanup rule, or support claim exists.

## Unresolved alternatives

Ownership categories and Copy/Move/borrow identity; shared-read and
exclusive-write scopes; alias/subview bounds/layout/identity; mapping/pinning,
visibility/coherence; transfer ownership and asynchronous completion;
cancellation/timeout/device loss/Fault/drop waiting; task/actor crossing;
Typed Core/effect/capability/resource witnesses; canonical schema/version,
diagnostics, migration, host/driver exclusion, protocol inventory, and public
Buffer status remain open under DBUF-4402, DBUF-4401, KCHK-4104/4105,
GAP-OWNERSHIP-MODEL-001, GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
