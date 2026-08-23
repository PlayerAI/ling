# DEC-0208: Internal Deadline Check boundary evidence / Deadline Check 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0207` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`TIM-5703-OBSERVATION`. It records provisional Node deadline, WCET-bound,
scheduler-interference, I/O-bound, margin, identity, overrun, failure, and
fixture vocabulary while Node timing and deadline authorities remain
unresolved.

本决定只授权 `TIM-5703-OBSERVATION` 使用 test-local 的 Node deadline、WCET bound、
scheduler interference、I/O bound、margin、identity、overrun、failure 与 fixture
边界清单；在 Node timing 与 deadline 权威尚未解决时，只记录临时词汇，不实现 deadline checker。

## Question

TIM-5703 proposes comparing a Node deadline with a WCET bound, scheduler
interference, an I/O bound, and a margin, then binding the conclusion to a
target, profile, and build ID. Which vocabulary can be retained as bounded
evidence without choosing Node/deadline semantics, a comparison equation,
overrun behavior, or a public schedulability protocol?

## Decision

1. `crates/ling-types/tests/deadline_check_evidence.rs` keeps a test-local
   inventory of sixty provisional Node timing, comparison, identity, overrun,
   failure, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.deadline-check-observation/0`. These bytes are
   observation evidence only; they are not a deadline checker, schedulability
   result, WCET certificate, Node runtime rule, Fault behavior, schema,
   diagnostic, protocol, or support claim.
3. The inventory retains deadline, WCET bound, scheduler interference, I/O
   bound, margin, validity condition, and target/profile/build identity as
   separate categories. Their co-presence does not define an equation,
   inequality, units, rounding, compatibility, or successful conclusion.
4. No Node syntax/Core/runtime, logical clock, deadline comparison, overrun
   Fault, evidence writer/verifier, dependency, diagnostic allocation, CLI/LSP
   route, support claim, or placeholder API is added. Public `TIM-5703`
   remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:503-514` is a
  non-normative checklist. It defines no units, equation, inequality, rounding,
  validity, identity compatibility, overrun behavior, or file format.
- `docs/status/TIM-5703-AUTHORITY-AUDIT.md` records the absent Node/deadline,
  WCET, interference, I/O-bound, target-binding, failure, and evidence
  authority.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Draft SEMANTICS/LANGUAGE Node sketches and Planning roadmap gates are not
  implementation authority. Accepted VM resource/cancellation/differential
  evidence and the internal query scheduler do not define target deadlines.
- `DEC-0206` and `DEC-0207` authorize only test-local predecessor vocabulary;
  they provide no Timing IR, timing-result semantics, or bound accepted here.

## Conformance plan

- Assert all sixty deadline-check categories and local order; compare forward/
  reverse opaque bytes; reject duplicates; retain comparison inputs, validity,
  and target/profile/build identity as distinct categories.
- Defer deadline-check implementation, Node/clock/overrun and schedulability
  semantics, schemas, diagnostics, protocols, and public support until
  Accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing VM limits, scheduler observations, performance trends,
Node/timing boundary inventories, and differential results are not
reinterpreted as a deadline conclusion; only test-local evidence is added.

## Unresolved alternatives

Node period/deadline/overrun syntax and Checked Core; logical-clock units,
activation/release and state/input/output rules; cancellation/Fault and skip/
queue/abort/degrade behavior; WCET result and assumption validity; scheduler,
interrupt and resource-interference accounting; I/O/device/FFI bounds; margin,
equation, inequality, rounding and acceptance policy; satisfied/missed/unknown/
invalid/unsupported result semantics; target/profile/build, processor,
toolchain, scheduler, clock, device package and TCB compatibility; Timing IR/
path, assumption/evidence/Semantic IDs and original UTF-8 spans; stale build,
target mismatch, missing/contradictory assumption, unknown path, malformed,
unsupported-version, migration and fail-closed behavior; diagnostics,
positive/negative/overrun/Unicode/differential fixtures, protocol inventory,
and public support remain open under TIM-5703, TIM-5702, TIM-5701,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing deadline authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
