# DEC-0152: Internal CPU scalar-reference boundary evidence / 内部 CPU 标量参考边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0151` | `DEC-0150` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`CPU-4201-OBSERVATION` scalar reference boundary. It records provisional
execution vocabulary while Kernel execution, numeric, Fault, and differential
authorities remain unresolved.

本决定只授权 `CPU-4201-OBSERVATION` 使用 test-local 的拟议标量参考执行边界清单，
在 Kernel execution、numeric、Fault 与 differential 权威尚未解决时，只记录临时词汇。

## Question

CPU-4201 proposes a direct scalar path for element-wise maps, indexing,
conditionals, bounded loops, buffer access, reductions, and explicit Faults.
Which planning vocabulary can be retained as bounded evidence without
defining Kernel execution semantics or an oracle relation?

## Decision

1. `crates/ling-types/tests/cpu_scalar_reference_evidence.rs` keeps a
   test-local inventory of sixty provisional input, operation, memory,
   reduction, Fault, numeric, output, diagnostic, fixture, differential, and
   protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.cpu-scalar-reference-observation/0`. They are not
   execution results, Fault semantics, diagnostics, Semantic IDs, backend
   protocols, or support claims.
3. No Kernel evaluator, scalar backend, Device Buffer API, reduction
   implementation, Fault mapping, dependency, diagnostic, protocol, or
   placeholder API is added. Public `CPU-4201` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:137-151` is
  non-normative; the CPU reference remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel execution, ownership,
  numeric determinism, Faults, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM foundations do
  not define a Kernel execution oracle.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer execution semantics, numeric modes, Fault mapping, canonical output,
  differential equivalence, diagnostics, migration, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no CPU backend, oracle, or support claim exists.

## Unresolved alternatives

Verified input and work-item model; map/index/conditional/loop/buffer and
reduction semantics; shape/bounds/alias/race/ownership/resource rules;
numeric modes and tolerances; cancellation and Fault identity/provenance;
canonical output/trace, diagnostics, fixtures, migration, CPU/device
differential, target rejection, protocol inventory, and backend support remain
open under CPU-4201, KCHK-4101 through KCHK-4105,
GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
