# DEC-0162: Internal Device IR schema boundary evidence / 内部 Device IR Schema 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: device-ir-quality  
> 相关规范/缺口：`DEC-0161` | `DEC-0157` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DIR-4501-OBSERVATION` backend-neutral Device IR schema boundary. It records
provisional IR types/operations, memory/control flow, synchronization/numeric,
capability/source-map, canonicalization, compatibility, diagnostic, and
protocol vocabulary while RFC-H404 and Kernel/backend authorities remain
unresolved.

本决定只授权 `DIR-4501-OBSERVATION` 使用 test-local 的拟议 backend-neutral Device IR schema 边界清单，
在 RFC-H404 与 Kernel/backend 权威尚未解决时，只记录临时 IR types/operations、memory/control flow、synchronization/numeric、capability/source-map、canonicalization、compatibility、diagnostic 与 protocol 词汇。

## Question

DIR-4501 proposes workgroup/grid, scalar/vector/tensor types, address spaces,
memory operations, control flow, barriers/atomics, shape/layout, numeric mode,
source maps, capabilities, and required backend features. Which planning
vocabulary can be retained as bounded evidence without defining a Device IR
schema, encoder/decoder, verifier, or backend contract?

## Decision

1. `crates/ling-types/tests/device_ir_schema_evidence.rs` keeps a test-local
   inventory of sixty provisional Device IR schema, type/operation, memory,
   control-flow, synchronization/numeric, source-map/capability,
   canonicalization, compatibility, diagnostic, fixture, host-exclusion, and
   protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.device-ir-schema-observation/0`. They are not IR
   nodes, schema fields, encodings, verifiers, capabilities, target features,
   diagnostics, Semantic IDs, public protocols, or backend support claims.
3. No Device IR type, schema, encoder/decoder, validator, canonicalizer,
   capability registry, dependency, diagnostic, protocol, or placeholder API is
   added. Public `DIR-4501` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:285-304` is
  non-normative and explicitly depends on RFC-H404; Device IR behavior remains
  outside v0.0.1.
- RFC-H404 is absent from docs and governance registries and is not an
  Accepted authority.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel/device memory, synchronization, numeric, IR, ABI, layout, target,
  capability, and backend behavior.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer IR type/operation semantics, memory/control-flow/barrier/atomic rules,
  shape/layout/numeric modes, capability negotiation, source maps,
  canonicalization, schema migration, diagnostics, and protocol behavior until
  accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Device IR schema, encoding, verifier, backend, or support claim
exists.

## Unresolved alternatives

IR identity/version and workgroup/grid; scalar/vector/tensor types and shape;
address spaces and memory operations/effects; control flow; barriers/atomics
and memory order; shape/layout/index/bounds; numeric modes and exact encoding;
source maps, capability/required-feature negotiation, unsupported targets,
Fault/cancellation; canonical constants, target-independent/specialization
hashes, migration/corruption; diagnostics, host/driver redaction, protocol
inventory, and public Device IR status remain open under DIR-4501,
DBUF-4401 through DBUF-4404, SIMD-4301 through SIMD-4303,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
