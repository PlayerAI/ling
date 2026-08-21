# KCHK-4104 Authority Audit — Alias and Parallel-Write Conflicts

Status: BlockedSpec

Date: 2026-08-22

## Outcome

KCHK-4104 proposes checking overlapping buffer views, read/write sets,
per-index ownership, reduction and atomic operations, and race-free conditions
for Kernel code. Programs that cannot be proven safe would be rejected or
required to use an explicitly accepted safety primitive.

The task cannot be implemented as an alias or race checker today. No
ownership judgment, buffer-view identity, read/write-set IR, reduction/atomic
semantics, race checker, safety primitive, diagnostic, Device Buffer API,
backend hook, or command is added. KCHK-4101 through KCHK-4103 and the
ownership/device authorities remain prerequisites.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:113-123 is a
  non-normative plan sketch. It names analyses but does not define alias
  identity, per-index ownership, atomics, reduction order, barrier
  interaction, accepted primitives, or compatibility behavior.
- docs/ROADMAP-1.0.md:330-371 places Copy/Move, Borrow, Region, Drop, aliasing,
  public lifetimes, Native ABI, and FFI behind the v0.3 gates. Its G4 section
  at 381-429 requires Kernel verification and CPU/device differential
  evidence, but supplies no accepted race or ownership calculus.
- docs/SEMANTICS.md:984-1005 sketches Borrow, overlap, region, and the future
  requirement that a Kernel writable slice prove non-aliasing. The same
  document states that v0.0.1 Seed does not infer BorrowShared,
  BorrowMutable, or Move and reserves Kernel in the non-implemented schema.
- docs/SEMANTICS.md:1429-1480 sketches Kernel non-overlap, Device Buffers,
  reductions, determinism, and lowering. These are future design statements,
  not an accepted alias, atomic, or race protocol.
- GAP-KERNEL-DEVICE-001 and GAP-OWNERSHIP-MODEL-001 are Open in
  docs/governance/gap-register.toml. They leave Kernel buffer ownership,
  address spaces, synchronization, determinism, aliasing, Copy/Move,
  borrow exclusivity, region escape, and drop order unresolved.
- RFC-0007 and RFC-0013 are only candidate/future authorities; no accepted
  ownership or Kernel RFC file resolves these contracts. RFC-0001 remains
  Draft under docs/decisions/0018-rfc-0001-lifecycle.md, and no Kernel race
  protocol is registered.

## Current implementation evidence

- The repository has no Kernel, Device Buffer, ownership, borrow, alias,
  read/write-set, reduction, atomic, or race checker implementation under
  crates or tests. The Seed compiler has no Kernel entry point.
- No accepted rule fixes whether buffers and views have nominal or
  content-derived identity, how subviews overlap, how index-dependent
  ownership is represented, or how alias facts survive calls, closures,
  transfers, barriers, and device lowering.
- No accepted rule fixes reduction associativity/order, atomic memory order,
  per-element versus whole-buffer exclusivity, barrier scope, race
  classification, synchronization Faults, or the meaning and safety proof
  of an explicit primitive. Host/device backends cannot define source
  semantics by their own diagnostics.
- Kernel CPU, GPU, and accelerator backends are Unsupported and Experimental
  in docs/governance/support-matrix.toml behind GAP-KERNEL-DEVICE-001. No
  diagnostic allocation, schema, Semantic ID rule, dependency, target/
  toolchain, or CLI command is required or changed by this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned ownership and Kernel RFC defining Value, Resource, Managed,
   Borrow, Region, buffer/view identity, address spaces, transfer ownership,
   and the boundaries visible to Typed Core and Device IR.
2. A deterministic alias judgment for view overlap, read/write sets,
   per-index ownership, calls and higher-order functions, loops, reductions,
   atomics, barriers, and subviews. It must define when a proof is required,
   what bounds/resource limits apply, and which fallback is permitted.
3. Normative reduction and atomic semantics: supported operations, memory
   ordering, synchronization scope, floating-point determinism class,
   failure behavior, and whether a primitive is safe, capability-gated, and
   auditable rather than an escape hatch.
4. A verifier boundary that consumes checked Typed Core or a versioned
   verified derivative, rejects unresolved alias/race facts before lowering,
   preserves original UTF-8 byte spans and Semantic IDs, and never evaluates
   unchecked AST nodes.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics with structured facts for
   overlapping views, conflicting read/write sets, missing per-index proof,
   unsupported atomic/reduction, barrier misuse, and target/profile mismatch.
6. Positive and negative fixtures for disjoint and overlapping views,
   per-index ownership, loops and calls, reductions/atomics/barriers,
   cancellation and Fault paths, CPU-reference equivalence, migration,
   Unicode/source-map positions, resource limits, and deterministic evidence.

## Evidence and compatibility impact

The eventual checker must provide a versioned alias/race schema, canonical
proof ordering and migration tests, and Graph/Audit projections that preserve
semantic identities and source provenance without exposing pointers, host
addresses, allocation order, thread timing, or driver logs. CPU reference and
device paths need differential evidence for exact or declared tolerance
behavior, and unsupported or unproven operations must be rejected visibly.
Any public safety-primitive or race-report protocol needs an inventory entry,
Accepted authority, reader/writer fixtures, and explicit Preview or Stable
status.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, effect or capability checker,
Device Buffer, scheduler, diagnostics, schema, Semantic IDs, source spans,
CLI, dependency lock, target/toolchain, support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

KCHK-4104 implementation, ownership and alias judgments, buffer views,
read/write sets, reductions, atomics, barriers, race diagnostics, safety
primitives, Graph/Audit fields, CPU reference, Device IR, device backends,
editor support, and public protocol claims remain deferred until
GAP-KERNEL-DEVICE-001 and GAP-OWNERSHIP-MODEL-001 are resolved by Accepted
authorities and the required executable evidence exists.
