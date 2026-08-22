# DEC-0147: Internal Kernel capability-matrix boundary evidence / 内部 Kernel 能力矩阵边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0146` | `DEC-0012` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`KCHK-4101-OBSERVATION` Kernel capability-matrix boundary. It records
provisional matrix vocabulary while Kernel, Device Buffer, verifier, CPU
reference, numeric, and backend authorities remain unresolved.

本决定只授权 `KCHK-4101-OBSERVATION` 使用 test-local 的拟议 Kernel 能力矩阵边界清单，
在 Kernel、Device Buffer、verifier、CPU reference、numeric 与 backend 权威尚未解决时，
只记录临时矩阵词汇。

## Question

KCHK-4101 proposes a machine-readable matrix for Kernel-accepted values,
control flow, effects, capabilities, ownership, buffers, determinism, and
target fallback. Which planning vocabulary can be retained as bounded evidence
without adding Kernel syntax, a checker, a schema, or a backend capability API?

KCHK-4101 计划建立 Kernel 可接受值、控制流、effects、capabilities、ownership、buffers、
determinism 与 target fallback 的机器可读矩阵。在不添加 Kernel 语法、checker、schema 或
backend capability API 的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/kernel_capability_matrix_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering matrix schema,
   capability identifiers/conditions/rejections/profile/target scope,
   source/Semantic-ID provenance, Graph/Audit projections, canonical bytes,
   migration/order, value/record/ADT/Managed/Resource/allocation/recursion/
   loops/calls/static dispatch, Effect/Capability rows and forbidden Task/
   Actor/network/IO, Device/Buffer/address-space/alias/race/bounds/overflow,
   numeric/reduction/target discovery/fallback, checked Typed Core/verified
   derivative, UTF-8/diagnostics/Unicode, fixtures/golden/round-trip,
   unsupported target, CPU reference/device differential, host exclusions,
   protocol inventory, public schema, error facts, and version compatibility.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.kernel-capability-observation/0`. These bytes are not a Kernel schema,
   checker result, capability decision, diagnostic, provenance record, Semantic
   ID, public protocol, or backend support claim.
3. The child adds no Kernel syntax, matrix schema, checker pass, Graph field,
   diagnostic, Device Buffer API, backend, dependency, toolchain, CLI command,
   protocol, or placeholder API. Public `KCHK-4101` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:69-83` is
  non-normative; its table shape and examples cannot define Kernel meaning,
  schema, diagnostics, or compatibility.
- `docs/ROADMAP-1.0.md:381-429` places Kernel/device work in G4 but is not an
  Accepted semantic authority. `docs/SEMANTICS.md` and `docs/LANGUAGE.md`
  reserve Kernel and exclude it from v0.0.1.
- RFC-0001 lists RFC-0013 as future work; RFC-0013/RFC-H401 are not Accepted
  documents. `GAP-KERNEL-DEVICE-001` remains Open and blocks KCHK-4101 and
  related device tasks.
- Existing support entries mark Kernel CPU/GPU/accelerator surfaces
  Unsupported or Experimental. No Kernel protocol is registered in
  `docs/governance/protocol-inventory.toml`.

## Conformance plan

- Assert all sixty provisional Kernel matrix boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep matrix parsing, checker/verifier semantics, Graph/Audit projections,
  diagnostics, CPU reference, Device IR/backends, numeric determinism,
  migration, and public support behavior deferred until the required authority
  and executable evidence exist.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No Kernel schema, checker, capability
  decision, Device Buffer API, backend, diagnostic, dependency, protocol, or
  support claim is registered.

## Unresolved alternatives

Kernel subset and profiles; matrix schema/identifiers/conditions/rejections;
value/layout/ADT/Managed/Resource/allocation/recursion/loop/call rules;
Effect/Capability restrictions; Device/Buffer ownership/address spaces;
alias/race/bounds/overflow; numeric/reduction determinism; target discovery/
fallback; Typed Core/verifier; CPU reference/device differential; Graph/Audit,
diagnostics, fixtures, migration, protocol inventory, and backend/editor
support remain open under KCHK-4101, GAP-KERNEL-DEVICE-001, and missing
RFC-0013/RFC-H401 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
