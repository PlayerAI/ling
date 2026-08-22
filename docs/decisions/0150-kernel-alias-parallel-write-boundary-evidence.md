# DEC-0150: Internal Kernel alias/parallel-write boundary evidence / 内部 Kernel Alias 与并行写边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0149` | `DEC-0147` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`KCHK-4104-OBSERVATION` Kernel alias and parallel-write conflict boundary. It
records provisional alias/proof vocabulary while ownership, address-space,
race, synchronization, and device authorities remain unresolved.

本决定只授权 `KCHK-4104-OBSERVATION` 使用 test-local 的拟议 Kernel alias 与并行写冲突边界清单，
在 ownership、address-space、race、synchronization 与 device 权威尚未解决时，只记录临时词汇。

## Question

KCHK-4104 proposes alias, borrow, disjointness, race, synchronization, and
parallel-write checks for Kernel buffers. Which planning vocabulary can be
retained as bounded evidence without adding ownership rules, race proofs, or
Kernel admission behavior?

## Decision

1. `crates/ling-types/tests/kernel_alias_parallel_write_evidence.rs` keeps a
   test-local inventory of sixty provisional alias, borrow, write-conflict,
   proof, bounds, buffer/address/ownership/device/profile, Effect/Capability,
   synchronization, determinism, Typed Core, diagnostics, fixtures, CPU/
   device differential, host-exclusion, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.kernel-alias-write-observation/0`. They are not
   alias proofs, race results, ownership decisions, diagnostics, Semantic IDs,
   public protocols, or backend support claims.
3. No Kernel alias checker, ownership API, race detector, Device Buffer API,
   backend, dependency, diagnostic, protocol, or placeholder API is added.
   Public `KCHK-4104` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:113-122` is
  non-normative; Kernel alias/race rules remain outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for ownership/address spaces,
  synchronization, alias/race proofs, numeric determinism, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer alias/borrow/write/race/synchronization semantics, verifier,
  diagnostics, CPU reference, device differential, migration, and protocol
  behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no alias proof, conflict decision, backend, or support claim exists.

## Unresolved alternatives

Alias/borrow identity and scope; disjoint/overlap/range/shape/index/bounds
proofs; parallel read/write conflicts; address spaces, buffer ownership,
synchronization, race/determinism, Typed Core/verifier, CPU/device evidence,
diagnostics, migration, protocol inventory, and backend support remain open
under KCHK-4104, KCHK-4101/4103, GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
