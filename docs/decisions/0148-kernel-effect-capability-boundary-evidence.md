# DEC-0148: Internal Kernel Effect and Capability boundary evidence / 内部 Kernel Effect 与 Capability 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0147` | `DEC-0010` | `DEC-0012` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`KCHK-4102-OBSERVATION` Kernel Effect and Capability-check boundary. It records
provisional rows and rejection vocabulary while Kernel, device, profile,
verifier, and backend authorities remain unresolved.

本决定只授权 `KCHK-4102-OBSERVATION` 使用 test-local 的拟议 Kernel Effect 与 Capability 检查
边界清单，在 Kernel、device、profile、verifier 与 backend 权威尚未解决时，只记录临时行与拒绝词汇。

## Question

KCHK-4102 proposes checking Kernel Effect rows and Capability closure from
Typed Core, including forbidden IO/Network/Task/Actor effects and profile or
target mismatches. Which planning vocabulary can be retained as bounded
evidence without changing the accepted Seed Effect checker or adding Kernel
admission behavior?

KCHK-4102 计划从 Typed Core 检查 Kernel Effect 行与 Capability closure，包含禁止 IO/Network/
Task/Actor effects 以及 profile/target mismatch。在不改变已接受 Seed Effect checker、也不添加
Kernel admission 行为的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/kernel_effect_capability_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering Effect and
   Capability schemas, Typed Core/program projections, inference/closure,
   entry/handler/allowed/forbidden rows, IO/Network/Task/Actor/Device/
   Resource/Managed/allocation/mutation effects, call/recursion/trait
   propagation, capability preflight and mismatch/rejection, profile/target/
   Kernel/Device scopes, diagnostics/facts/spans/Semantic IDs/Unicode,
   canonical cross-module/package projections, fixtures, unknown/duplicate/
   version/migration, checked input/verified derivative, backend/host/public
   protocol boundaries, CPU reference, determinism, and fallback.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.kernel-effect-capability-observation/0`. These bytes are not an Effect
   checker result, Capability decision, diagnostic, provenance record, Semantic
   ID, public protocol, or Kernel/backend admission claim.
3. The child adds no Kernel Effect/Capability checker, schema, diagnostic,
   Device Buffer API, backend, dependency, toolchain, CLI command, protocol, or
   placeholder API. Public `KCHK-4102` remains `BlockedSpec`.

## Normative basis

- Accepted RFC-0018 governs Seed Effect closure and Capability preflight only;
  it does not define Kernel admission, Device effects, profile/target rows, or
  a public Kernel checker.
- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:90-99` is
  non-normative, and Kernel remains reserved outside v0.0.1 under
  `docs/SEMANTICS.md`/`docs/LANGUAGE.md`.
- RFC-0013/RFC-H401 are not Accepted and `GAP-KERNEL-DEVICE-001` remains Open;
  no Kernel Effect/Capability protocol is registered.

## Conformance plan

- Assert all sixty provisional Kernel Effect/Capability boundaries and their
  test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep Kernel checker/admission, row semantics, profile/target policy,
  diagnostics, CPU reference, Device IR/backends, migration, and public
  support behavior deferred until the required authorities exist.

## Compatibility impact

- Accepted Seed Effect/Capability behavior, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-local boundary evidence. No Kernel checker, capability
  decision, diagnostic, dependency, protocol, backend, or support claim is
  registered.

## Unresolved alternatives

Kernel Effect/Capability schema and closure; forbidden/allowed rows; IO,
Network, Task, Actor, Device, Resource, Managed, allocation, mutation,
call/recursion/trait propagation; profile/target mismatch; diagnostics/facts;
CPU reference, determinism, fallback, migration, protocol inventory, and
backend support remain open under KCHK-4102, KCHK-4101, GAP-KERNEL-DEVICE-001,
and missing RFC-0013/RFC-H401 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
