# DEC-0178: Internal forbidden-capability boundary evidence / 内部禁止能力边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0177` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROF-5102-OBSERVATION`. It records provisional forbidden-capability,
effect, profile-state, Typed-Core, diagnostics, fixtures, and privacy
vocabulary while RFC-K501/RFC-0012 and ownership/concurrency/Kernel/Native
authorities remain unresolved.

本决定只授权 `PROF-5102-OBSERVATION` 使用 test-local 的禁止能力边界清单；
在 RFC-K501/RFC-0012 与 ownership/concurrency/Kernel/Native 权威尚未解决时，只记录临时
forbidden capability、effect、profile state、Typed-Core、diagnostic、fixture 与 privacy 词汇。

## Question

PROF-5102 lists capabilities to reject before lowering: Managed/GC,
unbounded allocation, undeclared Clock/Random/IO/Network/Device, dynamic
loading/reflection/shell, unbounded Task/Actor topology/mailboxes, unaudited
FFI, nondeterministic numeric/Placement, and missing Fault/fallback. Which
vocabulary can be retained as bounded evidence without implementing a policy
checker or choosing Critical semantics?

## Decision

1. `crates/ling-types/tests/forbidden_capability_evidence.rs` keeps a
   test-local inventory of sixty provisional capability/effect categories,
   profile states, checked-input and lowering boundaries, diagnostics,
   privacy, fixtures, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.forbidden-capability-observation/0`. These bytes
   are evidence only; they are not a policy, rejection pass, diagnostics,
   proof, target claim, or support.
3. No forbidden-capability checker, profile policy, dependency, diagnostic,
   CLI option, protocol, support claim, or placeholder API is added. Public
   `PROF-5102` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:108-120` is
  non-normative; its list does not define taxonomy, mapping, Typed-Core
  representation, precedence, bounds, profile selection, or diagnostics.
- `docs/ROADMAP-1.0.md:118` requires explicit Critical boundaries and
  reproducible evidence but does not authorize a checker.
- `docs/status/PROF-5102-AUTHORITY-AUDIT.md` records missing
  `GAP-CRITICAL-PROFILE-001`, ownership/concurrency/Kernel/Native and Fault
  authority; `DEC-0177` remains prerequisite Profile evidence only.

## Conformance plan

- Assert all sixty forbidden-capability boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer capability/effect policy, Typed-Core rejection phase, transitive
  summaries, bounds/topology/numeric/Fault/FFI semantics, diagnostics, CLI,
  and protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Seed effect/type checks are not reinterpreted as Critical
prohibitions; only test-local evidence is added.

## Unresolved alternatives

Capability/effect taxonomy and policy; Managed/GC/allocation, Clock/Random/
IO/Network/Device, dynamic loading/reflection/shell/build, Task/Actor/mailbox,
FFI, numeric/Placement determinism, Fault/fallback; checked Typed-Core input,
lowering phase, transitive summaries, target packages; Forbidden/Unavailable/
Assumed/RuntimeChecked/Proved/Experimental states; profile selection,
conflict/migration, Semantic IDs/spans, diagnostics, privacy and unstable-host
exclusions; positive/negative/transitive/source-span/profile-matrix/bound/
effect/numeric-Fault/FFI-target/determinism/differential/Unicode fixtures;
protocol inventory and public status remain open under PROF-5102, PROF-5101,
GAP-CRITICAL-PROFILE-001, GAP-OWNERSHIP-MODEL-001,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
