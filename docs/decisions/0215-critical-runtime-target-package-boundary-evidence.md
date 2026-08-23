# DEC-0215: Internal Critical Runtime/Target Package boundary evidence / 内部 Critical Runtime/Target Package 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-backend
> 相关规范/缺口：`DEC-0214` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-KERNEL-DEVICE-001` | `PROTO-ABI` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CBK-5903-OBSERVATION`. It records provisional Critical-profile, scheduling,
memory/resource, lifecycle/watchdog, Target Package, identity, evidence, and
fixture vocabulary while runtime, ABI, target, and evidence semantics remain
unresolved.

本决定只授权 `CBK-5903-OBSERVATION` 使用 test-local 的 Critical profile、
scheduling、memory/resource、lifecycle/watchdog、Target Package、identity、
evidence 与 fixture 边界清单；在 runtime、ABI、target 和 evidence 语义尚未
解决时，只记录临时词汇，不声明已实现 Critical runtime 或 Target Package。

## Question

CBK-5903 proposes a static scheduler, no general heap, bounded stack,
deterministic startup, watchdog/safe state, qualified target primitives,
declared host services, and target-specific evidence. Which vocabulary can be
retained as bounded evidence without defining executable runtime, timing,
memory, target, ABI, or safety semantics?

## Decision

1. `crates/ling-types/tests/critical_runtime_target_package_evidence.rs`
   keeps a test-local inventory of sixty provisional profile/Core, scheduling/
   time, memory/resource, lifecycle/Fault/watchdog, Target Package/primitive,
   identity, evidence/trust, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.critical-runtime-target-package-observation/0`.
   These bytes are observation evidence only; they are not a schedule,
   resource bound, runtime state, target manifest, diagnostic, protocol, or
   support claim.
3. Static scheduling, no-general-heap, bounded stack, deterministic startup,
   watchdog/safe state, qualified primitives, host-service declaration, and
   target-specific evidence remain distinct categories. Their presence
   establishes none of those behaviors.
4. `UndeclaredHostService` records the proposed rejection boundary only. No
   host service, capability, device primitive, clock, watchdog, or FFI access
   is authorized.
5. No Critical runtime/scheduler, heap/stack checker, startup/watchdog state
   machine, Target Package or primitive registry, ABI/FFI dependency, evidence
   verifier, CLI/LSP route, diagnostic allocation, public protocol, support
   claim, or placeholder API is added. Public `CBK-5903` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:611-620` is a
  non-normative checklist. It defines no schedule/time semantics, resource
  bounds, lifecycle transitions, safe-state observables, target identity, or
  evidence acceptance.
- `docs/status/CBK-5903-AUTHORITY-AUDIT.md` records missing Critical profile,
  Node/runtime, ownership/memory, target/ABI/primitive, watchdog/Fault,
  evidence, diagnostics, and executable fixture authority.
- `docs/IMPLEMENTATION.md` excludes Native, Resource/Borrow, Task/Actor/Node/
  Kernel, proof, and Critical runtime work from Seed. Critical, Native/ABI,
  ownership, and kernel/device gaps remain Open.
- `PROTO-ABI` and `PROTO-EVIDENCE` are Future. The support matrix keeps Native
  and Critical capabilities Unsupported/Unavailable.
- Accepted RFC-0014 and RFC-0019 authorize only the portable bytecode/VM route
  and Interpreter–VM evidence; they define no Critical runtime or target.
- `DEC-0214` authorizes only test-local validator vocabulary and implements no
  backend, runtime, or target behavior.

## Conformance plan

- Assert all sixty Critical-runtime/Target-Package categories and local order;
  compare forward/reverse opaque bytes; reject duplicates; retain scheduler,
  heap/stack, startup, watchdog/safe-state, target primitive, host-service,
  evidence, and protocol boundaries together.
- Defer runtime, scheduling/time, memory/resource, Target Package/ABI,
  watchdog/Fault, evidence/trust, diagnostics, protocols, and support until
  Accepted authority and offline target fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, interpreter/VM runtime, dependencies, support matrix, and Unicode
17.0.0 remain unchanged. Existing VM limits, host effects, concurrency
observations, and target-package planning are not reinterpreted as Critical
runtime guarantees; only test-local boundary evidence is added.

## Unresolved alternatives

Critical Core/profile and permitted effects; startup/initialization/shutdown/
reset/cancellation/Fault/overrun/watchdog/safe-state/recovery lifecycle;
static schedule, admission, priority/tie-breaking, clock/tick/deadline/WCET,
interrupt and I/O semantics; queue/recursion/heap/stack/frame/allocation bounds;
ownership/alias/Resource/Drop/cleanup and host-service boundary; versioned
Target Primitive Package, target/device/clock/watchdog primitive identities,
capabilities, ABI/calling convention/layout/FFI/thread/reentry/toolchain/
artifact identity; licensing and reproducible offline builds; timing/memory/
target evidence, assumptions, TCB, independent verification, provenance,
migration and nondeterminism; fail-closed profile/schedule/deadline/stack/heap/
primitive/host-service/watchdog/target/evidence failures; bilingual stable
diagnostics and exits; positive, negative, malformed, fault-injection,
boundedness, reset, target, repeated-build, cross-target, Unicode 17.0.0,
BOM/CRLF, and source-span fixtures; protocol inventory and truthful support
remain open under CBK-5903, CBK-5901, CBK-5902,
GAP-CRITICAL-PROFILE-001, GAP-NATIVE-BACKEND-ABI-001,
GAP-OWNERSHIP-MODEL-001, GAP-KERNEL-DEVICE-001, PROTO-ABI, PROTO-EVIDENCE,
and missing Critical runtime/target authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
