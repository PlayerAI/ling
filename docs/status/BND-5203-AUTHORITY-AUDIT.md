# BND-5203 Authority Audit — Memory Budgets

Status: BlockedSpec

Date: 2026-08-22

## Outcome

BND-5203 proposes calculating and verifying static data, a stack upper bound,
arena/buffer usage, queue/mailbox capacity, task/actor state, optional device
memory, transient peaks, and error/fallback paths. It also requires the result
to be bound to a target ABI and compiler version.

No RFC-K504 or accepted Critical memory model defines the units, ownership,
allocation, object layout, lifetime, aliasing, peak calculation, queue/task
accounting, device memory, error-path accounting, target binding, or proof
status required by this task. Implementing an analyzer now would invent
language semantics and could turn host allocator behavior or backend layout
into a public Ling guarantee.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:169-182` is a
  non-normative plan fragment. It lists categories and target/compiler binding
  but defines no byte/unit model, allocation or ownership rules, lifetime and
  alias treatment, peak/path semantics, proof states, or failure behavior.
- `docs/ROADMAP-1.0.md:118` and `:433-498` place Critical bounded allocation,
  reproducible evidence, and the profile checker after G2 concurrency, G3
  resources/Native, and G4 restricted lowering. These roadmap gates do not
  authorize a memory-budget checker or a public budget protocol.
- `GAP-CRITICAL-PROFILE-001` leaves boundedness, Critical capabilities, and
  evidence boundaries Open. `GAP-OWNERSHIP-MODEL-001`,
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-NATIVE-BACKEND-ABI-001`, and
  `GAP-KERNEL-DEVICE-001` leave the allocation, concurrency, target/ABI, and
  device/resource semantics needed by this task unresolved.
- The execution plan's `RFC-K504` is not an Accepted RFC in this repository.
  RFC-0014 and RFC-0015 define bytecode artifact limits and VM host-safety
  hooks (`step_limit`, `frame_limit`, and `heap_byte_limit`), not a common
  source-level memory-budget model or a Critical proof contract.
- RFC-0014 expressly treats heap enforcement as a host-safety boundary and
  leaves a common cross-backend logical heap formula to a future profile.
  RFC-0015 likewise limits VM frames/heap and excludes a common logical heap
  formula, Native ABI, Task, Actor, and later profiles.
- No Accepted RFC defines a target/compiler identity, migration policy, cache
  or replay identity, or independent evidence schema for memory budgets.

## Current implementation evidence

- The compiler has no memory-budget checker, allocation/ownership/region
  model, stack or arena data-flow analysis, transient-peak calculation, or
  budget evidence fixtures under `crates` or `tests`.
- There is no queue/mailbox/task/actor accounting model, device-memory
  placement rule, target-ABI layout contract, compiler-version binding, or
  error/fallback budget semantics.
- `crates/ling-vm/src/execute.rs` exposes `heap_byte_limit` and `frame_limit`
  as runtime safety hooks. Their Runtime Faults protect one VM execution and
  do not prove source memory usage, cross-backend equivalence, or a Critical
  budget.
- RFC-0014 decoder/verifier limits protect malformed or untrusted bytecode
  input. They are not source-level allocation facts and must not be promoted
  into Ling semantics.
- No stable diagnostic or schema distinguishes a proven bound from an
  estimate, unknown assumption, target mismatch, runtime exhaustion, or
  fallback path; no accepted rule defines units, object layout, sharing,
  fragmentation, drop timing, or worst-case path selection.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A memory and ownership/allocator model with stable units, object/layout
   rules, regions, borrowing, sharing/aliasing, lifetime/drop behavior,
   alignment, and the boundary between Value, Managed, Resource, and Device
   memory.
2. A sound static accounting model for static data, stack, arena/buffer,
   transient peaks, queues/mailboxes, task/actor state, optional device
   memory, and error/fallback paths, including control-flow joins, recursion,
   aliasing, sharing, cancellation, and worst-case path selection.
3. Exact proof, estimate, assumption, unknown, overflow, unsupported, and
   target-mismatch states, together with profile policy and a fail-closed rule
   for claims that cannot be checked.
4. Target ABI and compiler/toolchain binding, supported-target rules,
   migration/version behavior, reproducible evidence identity, and cache or
   replay rules that exclude host paths, addresses, allocator order, timing,
   fragmentation, and debug text from Ling identity.
5. Runtime budget guards and Fault/fallback semantics, including the
   host-safety versus logical-guarantee boundary, admission failure, OOM,
   queue overflow, device allocation failure, and whether a fallback is
   forbidden, explicit, or semantically equivalent.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for exceeded, unknown, estimated, unproved, unsupported, target-mismatch,
   and fallback conditions, with original UTF-8 byte spans and stable
   Semantic IDs.
7. Offline executable positive, negative, boundary, ownership/alias,
   lifetime/drop, queue/task/device, target/compiler, migration, determinism,
   differential, and independent-evidence fixtures that bound analysis,
   diagnostic ordering, output size, and resource use.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core and distinguish a
source-level logical guarantee from VM or host allocator safety. Budget facts
must be deterministic and tied to the accepted target/toolchain identity while
preserving original UTF-8 spans and Semantic IDs. Host addresses, physical
paths, allocator implementation details, timing, fragmentation, hash order,
and debug rendering must not affect Ling identity or public evidence.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory or ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, LSP, dependency lock,
target/toolchain support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

BND-5203 implementation, memory-budget analysis, allocation/ABI model,
diagnostics, CLI/LSP/evidence protocol, and support claims remain deferred
until RFC-K504 (or an Accepted replacement),
`GAP-CRITICAL-PROFILE-001`, ownership, concurrency/mailbox, Native/ABI,
Kernel/Device, numeric/effect, and related G2/G3/G4 authorities are resolved
with independent offline fixtures. No placeholder analyzer, allocator model,
target schema, or public API is created.
