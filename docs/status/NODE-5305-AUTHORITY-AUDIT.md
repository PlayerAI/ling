# NODE-5305 Authority Audit — Native Node Runtime

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5305 proposes a Native Node runtime with static memory, no general
allocation, a fixed schedule, a target timer primitive, a watchdog, safe state
transitions, startup/shutdown behavior, and telemetry kept outside the
hard-real-time path.

No Accepted Native/ABI, ownership/region, Critical Profile, Kernel/Device,
timer/Target Primitive, or Node RFC defines these claims. The support matrix
marks `BACKEND-NATIVE` Unsupported and unimplemented. Implementing this task
would require inventing object layout, allocation and cleanup, calling
conventions, timer/watchdog behavior, Fault/startup semantics, and target
support—not merely adding a runtime adapter.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:266-275` is a
  non-normative plan fragment. It names runtime properties but defines no
  static-memory accounting, ABI/layout, timer primitive, watchdog contract,
  safe-state transition, startup/shutdown state machine, telemetry boundary,
  target qualification, or evidence schema.
- `docs/SEMANTICS.md:1560-1568` describes Native conceptually, while
  `:1914-1931` reserves Native backend and Critical enforcement and forbids
  silent placeholders. `docs/LANGUAGE.md:1348-1358` explicitly lists Native
  Backend, Ownership/Borrow, Task/Actor/Node, and related systems features as
  first-version non-goals.
- `docs/ROADMAP-1.0.md:116-118` and `:352-386` require resource/ownership,
  Native IR/ABI, Fault/FFI, and reproducible target evidence before Native;
  G5 further requires bounded memory, Node timing/Fault, and target evidence.
- `GAP-NATIVE-BACKEND-ABI-001` leaves Native IR validity, layout, ABI,
  unwinding/Fault, thread/reentry, typed FFI, Target Primitive Packages, and
  target tiers Open; its next action is RFC-0011 after ownership categories.
- `GAP-OWNERSHIP-MODEL-001` leaves Value/Managed/Resource/Borrow/Region/Drop,
  aliasing, cleanup, and Profile boundaries Open. `GAP-KERNEL-DEVICE-001`
  leaves device/backend capabilities, synchronization, placement, and
  deterministic target operations Open. `GAP-CRITICAL-PROFILE-001` leaves the
  Critical Core, Node timing/Fault, boundedness, and evidence boundary Open.
- `docs/governance/support-matrix.toml:207-216` records `BACKEND-NATIVE` as
  `Unsupported`, `implemented = false`, with Native ABI and ownership blockers.
  No timer, watchdog, Native Node runtime, or target protocol is registered.
- Accepted Seed RFCs (including RFC-0017–RFC-0020) cover mutable-place,
  Effect/Capability, interpreter–VM differential, and VM host-control slices;
  none authorizes a Native backend or Node runtime.

## Current implementation evidence

- The repository has no Native IR/backend, ABI manifest, target package,
  calling convention, timer/watchdog adapter, static-memory linker/runtime,
  safe-state machine, startup/shutdown lifecycle, or Native Node fixtures.
- `ling-eval` and `ling-vm` execute checked Seed programs on host facilities;
  VM `heap_byte_limit`, `frame_limit`, and cooperative host cancellation are
  safety controls, not static-memory proofs, target timers, watchdog behavior,
  or hard-real-time guarantees.
- There is no accepted rule for object/value layout, stack/arena/data buffers,
  ownership/drop/region cleanup, interrupt/preemption, timer drift, watchdog
  expiry, state commit atomicity, startup failure, shutdown ordering, safe
  fallback, or telemetry effects.
- No target/compiler/ABI identity, supported-target matrix, reproducible
  Native artifact, source/binary correspondence, or independent TCB evidence
  exists. Native and device backends are explicitly unsupported in the matrix.
- No stable bilingual diagnostic or schema fixes unsupported Native target,
  ABI mismatch, allocation/ownership violation, timer/watchdog failure,
  unsafe state transition, startup/shutdown Fault, telemetry violation, or
  hard-real-time evidence mismatch.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Native IR, ABI, target primitive, FFI, calling convention, data/layout,
   endianness, alignment, unwind/Fault, thread/reentry, and version/migration
   contracts with explicit target/toolchain identity.
2. Ownership/Region/Resource/Drop, static-memory categories, stack/arena/buffer
   accounting, no-general-allocation enforcement, state-cell layout, aliasing,
   cleanup, and safe-state transition semantics.
3. Node schedule/tick/clock/deadline/overrun authority, timer primitive units
   and drift, watchdog arming/expiry/recovery, interrupt/preemption
   assumptions, startup/shutdown state machine, and hard-real-time versus
   telemetry/host-service boundary.
4. Critical Profile capability restrictions, Kernel/Device/Placement rules,
   Native/Actor/Task boundary, unsupported-target and fallback behavior,
   deterministic numeric/effect rules, and bounded memory/queue semantics.
5. Target-specific evidence and reproducible build/artifact identity,
   source/binary correspondence, TCB/qualification claims, sanitizer/fuzz
   coverage, and independent verification boundaries.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for target/ABI unsupported, memory/ownership, timer/watchdog, schedule,
   startup/shutdown, safe-state, telemetry, Fault, and evidence mismatches.
7. Offline executable positive/negative, ABI/layout, ownership/drop, static
   memory, timer/watchdog, startup/shutdown, safe-state, telemetry isolation,
   target/compiler, migration, Unicode/CRLF/BOM, determinism, and
   interpreter/VM/Native differential fixtures.

## Evidence and compatibility impact

The eventual implementation must consume accepted Node Checked Core and
verified Native IR only, fail closed on missing target/ABI/memory/timing facts,
and preserve original UTF-8 spans and Semantic IDs. Target hardware behavior,
host paths, addresses, allocator implementation, interrupt timing, telemetry
format, linker/debug text, and Rust layout must not become Ling identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, scheduler, Node runtime, memory or
ownership behavior, diagnostics, schemas, Semantic IDs, source spans, CLI,
LSP, dependency lock, target/toolchain support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

NODE-5305 implementation, Native Node runtime, target timer/watchdog adapters,
static-memory/runtime package, safe-state/startup/shutdown/telemetry contracts,
diagnostics, CLI/LSP/evidence protocols, and support claims remain deferred
until RFC-K502 and RFC-0011 (or Accepted replacements),
`GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, `GAP-KERNEL-DEVICE-001`,
`GAP-CRITICAL-PROFILE-001`, and NODE-5301 through NODE-5304 are resolved with
independent target fixtures. No placeholder Native backend, target API, or
public runtime is created.
