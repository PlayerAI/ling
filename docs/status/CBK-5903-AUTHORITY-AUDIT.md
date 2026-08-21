# CBK-5903 Authority Audit

- Task: `CBK-5903` — Critical Runtime/Target Package
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:611-620`
- Release: G5
- Status: `BlockedSpec`

## Decision

CBK-5903 is `BlockedSpec`. The execution plan lists a static scheduler, no
general heap, bounded stack, deterministic startup, watchdog/safe state,
qualified target primitives, explicit host-service declarations, and
target-specific evidence. It supplies no Critical runtime lifecycle, timing
model, memory model, target ABI, primitive-package schema, fault policy, or
evidence contract. The G5 package also requires the unresolved G2/G3/G4
boundaries and CBK-5901/CBK-5902.

No accepted authority permits a Critical runtime or target package. Implementing
one now would invent scheduling, clock/deadline, startup/shutdown, allocation,
stack, watchdog, Fault, capability, host-service, target, ABI, and safety
evidence semantics, while the support matrix explicitly marks Critical and
Native capabilities unavailable.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:611-620` is a non-normative checklist. It does not
  define static-schedule semantics, time and interrupt sources, startup order,
  overrun/watchdog transitions, safe-state observables, stack/heap bounds,
  target primitive identity, or evidence acceptance.
- The G5 preamble requires accepted G2 Replay, G3 Resource/Native, and G4
  restricted-Lowering boundaries. Those boundaries remain incomplete; the
  execution package cannot replace their missing specifications.
- `docs/IMPLEMENTATION.md:17` excludes Native Backend, Resource/Borrow
  Checker, Task/Actor/Node/Kernel, proof tooling, and related future runtime
  capabilities from the v0.0.1 Seed target. No Critical runtime may be added
  without an Accepted authority.
- `GAP-CRITICAL-PROFILE-001` remains Open. The minimum Critical Core,
  forbidden capabilities, Node timing/Fault rules, Contract proof/runtime
  boundary, boundedness, model-checking claims, and evidence schema are not
  accepted; its next action is drafting RFC-0012 before any Critical API or
  evidence format.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open for target tiers, Target Primitive
  Packages, layout, ABI, unwinding/Fault, threads/reentry, and typed FFI.
  `GAP-KERNEL-DEVICE-001` remains Open for backend capability discovery,
  determinism, Placement, and device boundaries. These are prerequisites for
  any target package rather than implementation details.
- `PROTO-ABI` and `PROTO-EVIDENCE` are Future planned public protocols without
  versions, canonical schemas, identity/provenance rules, readers, writers, or
  executable verification fixtures. The support matrix records Native and
  Critical profiles as `Unavailable`, `TARGET-NATIVE-AOT` and
  `BACKEND-NATIVE` as `Unsupported`, and no committed Ling target.
- Accepted RFC-0014 and RFC-0019 cover the portable bytecode/VM route and
  Interpreter–VM differential evidence only. They do not define a static
  scheduler, Critical runtime, target primitive package, watchdog, or
  target-specific evidence; RFC-0019 explicitly leaves Native execution
  contracts future work.
- `ROADMAP-1.0.md:324-379` and the G5 exit gates are planning authority. Their
  Native/target, TCB, timing, and evidence requirements cannot be treated as
  executable runtime semantics.

## Evidence in this repository

There is no Critical runtime, static scheduler, bounded-stack/heap checker,
startup or watchdog state machine, target primitive registry, target package,
target-specific runtime backend, or Critical evidence verifier under `crates/`,
`tests/`, or `schemas/`. `crates/ling-vm` provides the accepted library VM and
host capability/limit behavior, not a deterministic Critical scheduler or
target contract. No CLI, LSP request, diagnostic, public protocol, or support
entry claims CBK-5903.

## Required authority before implementation

An accepted Critical runtime/target RFC or replacement must define, at minimum:

1. The Critical profile and runtime lifecycle: permitted Core/effects,
   startup, initialization, shutdown, reset, cancellation, Fault, overrun,
   watchdog, safe-state, recovery, and fail-closed behavior.
2. Static scheduling and time semantics: task/node admission, priorities and
   tie-breaking, tick/clock sources, deadlines, WCET assumptions, interrupt
   and I/O treatment, queue/recursion/stack/allocation bounds, and deterministic
   behavior under every declared target condition.
3. Memory and resource rules: no-general-heap meaning, stack/frame layout,
   Resource/Drop/cleanup on normal, Error, Fault, cancellation, and reset
   paths, ownership/aliasing, address spaces, and the boundary to host services.
4. Versioned Target Primitive Package and ABI contracts: target identity,
   device/clock/watchdog primitives, capabilities, calling convention,
   layout, FFI/thread/reentry rules, toolchain identity, licensing, and
   reproducible/offline build and artifact identity.
5. Target-specific evidence and trust boundaries: timing/memory measurements
   versus bounds, assumptions, TCB, independent verifier, provenance,
   migration, allowed nondeterminism, and the exact claims a package may make.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and schemas for
   profile violations, schedule/deadline/stack/heap failure, unavailable or
   mismatched primitives, undeclared host service, watchdog/safe-state failure,
   target/ABI mismatch, and evidence rejection, plus offline positive,
   negative, malformed, fault-injection, boundedness, reset, target, repeated-
   build, Unicode 17.0.0, BOM/CRLF, and cross-target fixtures.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, or Semantic ID rule. It preserves the accepted `ling` CLI and `.ling`
source extension, checked Typed Core boundary, original UTF-8 spans, Unicode
17.0.0, deterministic identity rules, and the existing interpreter/VM route.

It deliberately adds no scheduler, runtime, target package, target primitive,
ABI/FFI dependency, watchdog, safe-state API, evidence schema, diagnostic, CLI
command, public protocol, or placeholder crate, and introduces no stale `zero`
names. CBK-5903 remains deferred until the Critical Profile, Node, boundedness,
Contract/Proof, evidence, Native/ABI, ownership, kernel/device, and compiler
route authorities are Accepted with executable target fixtures and a truthful
support-matrix update.
