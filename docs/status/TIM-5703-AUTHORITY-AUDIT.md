# TIM-5703 Authority Audit

- Task: `TIM-5703` — Deadline Check
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:503-514`
- Release: G5
- Status: `BlockedSpec`

## Decision

TIM-5703 is `BlockedSpec`. The execution plan proposes comparing a Node
deadline with a WCET bound, scheduler interference, an I/O bound, and a margin,
then binding the conclusion to a target, profile, and build ID. It does not
define Node syntax or execution, clock units, overrun behavior, WCET evidence,
interference accounting, I/O bounds, or the identity and compatibility of a
deadline result.

No accepted specification authorizes a deadline checker or a runtime/build
claim derived from those values. Implementing one now would invent Critical
Node semantics and could present an unproven timing estimate as a safety
guarantee.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:503-514` is a non-normative checklist. It does not
  establish a comparison equation, units, clock source, inequality/rounding
  rules, margin policy, target compatibility, or failure/overrun semantics.
- `docs/SEMANTICS.md:1380-1425` sketches synchronous Node periods, deadlines,
  WCET/memory bounds, and says a deadline is a Contract/Target assumption
  combination whose evidence records processor, compiler, scheduler, interrupt,
  cache/bus, and method. `SEMANTICS` is Draft in
  `docs/governance/authority.toml`; the sketch is not an accepted Node or
  target contract.
- `docs/LANGUAGE.md:857-865` shows `node`, `every`, and `deadline` as future
  surface design. `LANGUAGE` is Draft, and v0.0.1 Seed does not implement this
  syntax. `docs/ROADMAP-1.0.md:439-466` and `:492-498` are Planning gates that
  require future Node deadline/overrun semantics and repeatable tests; they do
  not authorize a checker.
- `GAP-CRITICAL-PROFILE-001` remains Open and explicitly leaves Node timing and
  Fault semantics, boundedness, Critical claims, and evidence schema
  unresolved. Its candidate RFC-0012 is not present or Accepted.
- `PROTO-EVIDENCE` is Planned public/Future without a version, schema,
  canonical encoding, reader/writer, verification, migration policy, or
  fixtures. No deadline or timing protocol is registered separately.
- Accepted RFC-0014 supplies deterministic VM step/frame/heap limits but no
  target WCET or deadline semantics. RFC-0020 explicitly excludes wall-clock
  deadlines and scheduler/replay protocols; RFC-0019 excludes instruction
  counts and host timing from its differential projection. DEC-0019's scheduler
  governs internal compiler queries only.

## Evidence in this repository

There is no Node runtime, virtual-clock model, deadline comparison, WCET
certificate, scheduler-interference analyzer, I/O-bound model, target/profile
binding, overrun Fault, report schema, or deadline fixture under `crates/` or
`tests/`. Existing VM resource/cancellation tests and compiler query scheduling
tests have different accepted scopes. No `ling` CLI, LSP request, diagnostic,
or public protocol claims TIM-5703 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. Node period/deadline/overrun syntax and Checked Core semantics, including
   logical-clock units, activation/release rules, synchronous state/input/output
   behavior, cancellation/Fault behavior, and whether missed deadlines skip,
   queue, abort, or degrade work.
2. A sound WCET/bound result contract linked to TIM-5701/TIM-5702, including
   path and loop assumptions, static-versus-measured status, unknown or
   infeasible paths, I/O and device/FFI bounds, and the non-proof status of
   empirical observations.
3. Scheduler and interference semantics: priority/order, preemption or
   cooperative execution, interrupts, other Nodes/Tasks/Actors, resource
   contention, margin units/rounding, and the conditions under which a
   comparison is valid or must be rejected.
4. Target/profile/build identity and compatibility rules for processor,
   instruction costs, cache/memory/bus, compiler/toolchain, scheduler,
   clock, device package, and TCB. Host paths, addresses, wall-clock timing,
   allocator layout, and debug output must not become Ling identity.
5. Versioned result/evidence and failure behavior for missing or contradictory
   assumptions, unsupported targets, stale builds, unknown paths, malformed
   records, schema migration, and deadline failure, with registered bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics and fail-closed process semantics.
6. Offline positive, negative, boundary, overrun, target/profile variation,
   migration, Unicode 17.0.0, BOM/CRLF, source-span, repeated-run
   determinism, and differential fixtures. Evidence must disclose limits and
   must never claim that a bounded check proves all platforms or inputs.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no Node syntax,
clock/deadline runtime, WCET checker, overrun Fault, evidence writer/verifier,
or placeholder API, and it introduces no stale `zero` names.

TIM-5703 remains deferred until Critical Profile, Node, Timing IR, measurement
and static-analysis, boundedness, target/ABI, scheduler, device/FFI,
Contract/Proof, and evidence authorities are Accepted with executable
fixtures.
