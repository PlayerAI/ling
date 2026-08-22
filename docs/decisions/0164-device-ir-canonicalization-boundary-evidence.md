# DEC-0164: Internal Device IR canonicalization boundary evidence / 内部 Device IR 规范化边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: device-ir-quality  
> 相关规范/缺口：`DEC-0163` | `DEC-0162` | `DEC-0157` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DIR-4503-OBSERVATION` Device IR canonicalization boundary. It records
provisional schema, identity, ordering, encoding, target-specialization,
migration, diagnostics, and protocol vocabulary while Device IR and backend
authorities remain unresolved.

本决定只授权 `DIR-4503-OBSERVATION` 使用 test-local 的拟议 Device IR 规范化边界清单，
在 Device IR 与 backend 权威尚未解决时，只记录临时 schema、identity、ordering、encoding、target-specialization、migration、diagnostic 与 protocol 词汇。

## Question

DIR-4503 proposes deterministic Device IR node and block ordering, canonical
constants, a target-independent hash, a separate target-specialization hash,
host/driver exclusion, and schema-version migration tests. Which planning
vocabulary can be retained as bounded evidence without defining canonical
bytes, hash domains, schema compatibility, or public cache identity?

## Decision

1. `crates/ling-types/tests/device_ir_canonicalization_evidence.rs` keeps a
   test-local inventory of sixty provisional schema, identity, ordering,
   encoding, target, migration, fixture, diagnostic, host-exclusion, and
   protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.device-ir-canonicalization-observation/0`. They
   are not Device IR nodes, canonical bytes, hashes, schema records, migration
   witnesses, diagnostics, Semantic IDs, public protocols, or backend support
   claims.
3. No canonicalizer, serializer, hash API, schema registry, migration reader
   or writer, dependency, diagnostic, protocol, or placeholder API is added.
   Public `DIR-4503` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:317-326` is
  non-normative; Device IR canonicalization remains outside v0.0.1.
- `DEC-0012` governs current Semantic IDs and canonical projections; it does
  not define Device IR identity, target specialization, or hardware hashes.
- DIR-4501 and DIR-4502 remain `BlockedSpec`; RFC-H404 is absent.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain open for
  Device IR, target, capability, layout, numeric, ABI, and backend behavior.

## Conformance plan

- Assert all sixty canonicalization boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer canonical ordering, constant and numeric encodings, hash domains,
  target profiles, schema compatibility, corruption, migration, redaction,
  diagnostics, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Device IR canonicalizer, hash, schema, migration, or support
claim exists.

## Unresolved alternatives

Device IR schema and identity; node/block/operation/dependency ordering;
constant pools and integer/float/opaque/shape/layout encodings; numeric,
effect, ownership, synchronization, Fault, and capability fields; source maps
and Semantic IDs; target-independent versus target-specialization inputs;
feature profiles and unsupported targets; extension, unknown-field, corruption,
redaction, version, and migration policy; determinism, Unicode/source-map
fixtures, diagnostics, host/driver exclusion, protocol inventory, and public
canonicalization status remain open under DIR-4501/4502, KCHK-4101 through
KCHK-4105, CPU-4201 through CPU-4203, SIMD-4301 through SIMD-4303,
DBUF-4401 through DBUF-4404, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
