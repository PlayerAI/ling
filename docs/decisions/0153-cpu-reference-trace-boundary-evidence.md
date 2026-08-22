# DEC-0153: Internal CPU reference-trace boundary evidence / 内部 CPU 参考 Trace 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0152` | `DEC-0151` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`CPU-4202-OBSERVATION` reference-trace boundary. It records provisional event,
provenance, limit, and redaction vocabulary while scalar Kernel execution and
trace authorities remain unresolved.

本决定只授权 `CPU-4202-OBSERVATION` 使用 test-local 的拟议参考 Trace 边界清单，
在标量 Kernel execution 与 Trace 权威尚未解决时，只记录临时事件、provenance、限制与脱敏词汇。

## Question

CPU-4202 proposes a test-mode trace containing logical work items, buffer
reads/writes, indexes, operations, and Faults. Which planning vocabulary can
be retained as bounded evidence without defining event semantics, ordering, or
a stable protocol?

## Decision

1. `crates/ling-types/tests/cpu_reference_trace_evidence.rs` keeps a
   test-local inventory of sixty provisional event, execution, provenance,
   limit, redaction, fixture, differential, diagnostic, and protocol
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.cpu-reference-trace-observation/0`. They are not
   trace events, execution results, diagnostics, Semantic IDs, public
   protocols, or support claims.
3. No trace producer, event schema, serializer, CLI flag, runtime hook,
   dependency, diagnostic, protocol, or placeholder API is added. Public
   `CPU-4202` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:153-164` is
  non-normative and explicitly limits trace output to test/explanation use.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel execution, event ordering,
  determinism, Faults, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM tracing does not
  define a Kernel reference trace.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer event meanings, ordering, sampling, redaction, limits, canonical
  serialization, diagnostics, migration, differential, and protocol behavior
  until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no trace format, CLI, or support claim exists.

## Unresolved alternatives

Event identity/order/payload and observation points; work-item/index and
buffer/view identity; operation/reduction/atomic/barrier/Fault semantics;
provenance, numeric/determinism, sampling, event/byte limits, truncation,
redaction and sensitive-data exclusion; canonicalization, corruption,
migration, fixtures, CPU/device differential, diagnostics, protocol inventory,
and public trace status remain open under CPU-4202, CPU-4201,
KCHK-4101 through KCHK-4105, GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
