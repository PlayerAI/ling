# DEC-0151: Internal Kernel Core/verifier boundary evidence / 内部 Kernel Core 与 Verifier 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0150` | `DEC-0147` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`KCHK-4105-OBSERVATION` Kernel Core and verifier boundary. It records
provisional vocabulary while the Kernel/device, verifier, identity, and
backend authorities remain unresolved.

本决定只授权 `KCHK-4105-OBSERVATION` 使用 test-local 的拟议 Kernel Core 与 Verifier 边界清单，
在 Kernel/device、verifier、identity 与 backend 权威尚未解决时，只记录临时词汇。

## Question

KCHK-4105 proposes a versioned, device-independent Kernel Core and an
independent verifier. Which planning vocabulary can be retained as bounded
evidence without defining a Core schema, verifier trust boundary, or backend
admission behavior?

## Decision

1. `crates/ling-types/tests/kernel_core_verifier_evidence.rs` keeps a test-local
   inventory of sixty provisional Core, witness, rule, rejection, diagnostic,
   identity, fixture, differential, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.kernel-core-verifier-observation/0`. They are not
   Kernel nodes, verifier proofs, diagnostics, Semantic IDs, public schemas,
   or backend support claims.
3. No Kernel Core schema, encoder/decoder, independent verifier, Device IR,
   backend, dependency, diagnostic, protocol, or placeholder API is added.
   Public `KCHK-4105` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:124-136` is
  non-normative; Kernel Core and verifier behavior remain outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for the Kernel subset, verifier trust
  boundary, identity, determinism, Device IR, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM verifier RFCs do
  not authorize Kernel artifacts.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer Core grammar, verifier semantics, canonical serialization, source
  maps, diagnostics, migration, CPU/device differential, and protocol
  behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Kernel verifier, backend, or support claim exists.

## Unresolved alternatives

Core node grammar and legal types/control flow; Typed Core and verified
derivative trust boundaries; witness/proof identity; effect/capability,
shape/bounds, alias/race, ownership/resource, profile/target/device, and
determinism rules; diagnostics, source maps, canonical bytes, migration,
resource limits, CPU/device evidence, protocol inventory, and backend support
remain open under KCHK-4105, KCHK-4101 through KCHK-4104,
GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
