# DBUF-4404 Authority Audit — Device Synchronization Model

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DBUF-4404 proposes command queues, events and fences, host await, device
barriers, cross-queue ordering, buffer hazards, cancellation, and device-lost
behavior.

No synchronization runtime or public model can be added yet. The proposal does
not define a checked representation of queue/event state, memory visibility,
hazard proofs, ordering identity, cancellation and cleanup, or device-loss
Faults. Implementing one backend-specific interpretation would create
unreviewed concurrency and memory semantics.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:272-283 lists the
  synchronization topics but does not define source syntax, queue ownership,
  event/fence identity, host/device visibility, barrier scope, cross-queue
  ordering, hazard classification, cancellation, device-loss recovery, or
  committed effects.
- docs/ROADMAP-1.0.md:381-429 requires Device Buffer synchronization, transfer
  effects, unsupported-target behavior, and deterministic evidence for G4. It
  is a roadmap gate, not an Accepted synchronization or runtime Fault
  protocol.
- docs/SEMANTICS.md reserves DeviceFault, Device Buffer, transfer, and future
  Kernel behavior; v0.0.1 excludes Kernel and Device execution. docs/LANGUAGE.md
  does not authorize queue, fence, barrier, or device-loss language behavior.
- GAP-KERNEL-DEVICE-001 leaves synchronization, buffer ownership/address spaces,
  backend capability, and determinism unresolved. GAP-OWNERSHIP-MODEL-001
  leaves resource transitions, aliases, regions, and drop behavior unresolved.
  DBUF-4401 through DBUF-4403 are BlockedSpec on those contracts.
- Accepted Seed/VM runtime decisions do not define device queues, memory
  barriers, cross-queue ordering, device-loss recovery, or a device execution
  oracle.

## Current implementation evidence

- No command queue, event/fence, host-await, barrier, hazard checker,
  cross-queue ordering graph, cancellation path, device-loss Fault mapper, or
  synchronization corpus exists under crates or tests.
- No accepted rule fixes queue and event identity, happens-before or memory
  visibility, barrier scope, hazard classes, read/write conflict detection,
  ordering across queues, or whether host await is blocking, effectful, or
  cancelable.
- No lifecycle contract defines in-flight work on normal return, Error, Fault,
  cancellation, timeout, drop, process exit, or device loss; nor does it define
  which effects are committed and how resources are cleaned.
- No synchronization protocol, schema, diagnostic allocation, dependency,
  target/toolchain selection, CLI command, or public support claim is required
  or changed by this audit. The public CLI and source extension remain ling
  and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A checked Typed Core and runtime state model for queues, events, fences,
   barriers, awaits, buffer hazards, and device-loss states with stable
   Semantic IDs and original UTF-8 source spans.
2. Memory visibility and ordering semantics for command submission, host/device
   access, queue and cross-queue happens-before, barriers, event completion,
   acquire/release behavior, and deterministic scheduling evidence.
3. Hazard and ownership rules for overlapping reads/writes, views/subviews,
   transfers, mapping/pinning, queue sharing, actor/task crossing, and
   rejection of unsafe or ambiguous ordering.
4. Cancellation, timeout, drop, Error/Fault, device loss, and process-shutdown
   lifecycle rules, including committed effects, cleanup, recovery, and
   resource limits. Device faults must map to stable diagnostics.
5. A versioned synchronization/evidence schema and protocol lifecycle for queue,
   event, fence, barrier, hazard, capability, result, Fault, source map,
   corruption, migration, redaction, and deterministic ordering.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism, hazard,
   cancellation, device-loss, and cross-target fixtures executable offline.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core or a verified
derivative only and must never interpret unchecked AST nodes. Synchronization
facts must preserve original UTF-8 byte spans, Semantic IDs, Unicode 17.0.0
behavior, declared effects, deterministic ordering, and explicit Fault
provenance. Host thread IDs, addresses, timing, driver output, and debug text
must remain outside Ling semantics.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DBUF-4404 implementation, queue/event/fence types, barriers and hazard checks,
ordering graph, host-await and cancellation runtime, device-loss handling,
synchronization evidence, Unicode/source-map cases, editor integration, and
public protocol claims remain deferred until DBUF-4401 through DBUF-4403 and the
Kernel/device and ownership gaps are resolved by Accepted authority and
executable evidence exists.
