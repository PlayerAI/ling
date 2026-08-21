# DBUF-4403 Authority Audit — Transfer Effect

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DBUF-4403 proposes an explicit transfer operation and requires Semantic Graph or
Audit evidence for byte count, source and destination address spaces,
synchronization, possible Faults, and a DeviceTransfer capability. It also says
large implicit transfers must not appear free.

This effect cannot be added yet. The proposed syntax, effect-row extension,
capability identity, ownership transition, address-space model, synchronization
completion, Fault behavior, and cost/evidence representation are not Accepted.
Adding a transfer operation now would make unresolved device and resource
semantics part of the language.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:252-270 sketches a
  transfer expression and required graph/audit facts, but does not define
  source syntax, type result, effect-row grammar, byte-count units, address-space
  identity, synchronization state, failure/cancellation behavior, capability
  provenance, or cost observability.
- docs/ROADMAP-1.0.md:381-429 requires explicit transfer effects, Device Buffer
  lifecycle, synchronization, capability discovery, and non-silent unsupported
  behavior for G4. It is a roadmap gate, not an Accepted transfer protocol.
- docs/SEMANTICS.md reserves Device transfer and DeviceFault concepts for future
  Kernel behavior; the v0.0.1 Seed effect and capability slices do not authorize
  DeviceTransfer or device execution. docs/LANGUAGE.md excludes Kernel and
  Device Buffer from the first implementation.
- GAP-KERNEL-DEVICE-001 leaves Device Buffer address spaces, synchronization,
  capability discovery, numeric determinism, and backend behavior unresolved.
  GAP-OWNERSHIP-MODEL-001 leaves resource transitions, aliases, regions, and
  drop behavior unresolved. DBUF-4401 and DBUF-4402 are BlockedSpec on those
  contracts.
- Existing accepted Seed/VM effect and Fault decisions do not define device
  transfer, asynchronous completion, cross-address-space ownership, or a
  Native/device execution oracle.

## Current implementation evidence

- No transfer expression, Transfer Effect, DeviceTransfer capability, address
  space type, byte-count checker, synchronization token, transfer Fault mapper,
  or Semantic Graph/Audit transfer record exists under crates or tests.
- No accepted rule fixes whether transfer is a move, borrow, copy, or view
  transition; how source and destination layouts are validated; whether bytes
  are logical or physical; or how overlapping and zero-length transfers behave.
- No lifecycle contract defines asynchronous completion, visibility, cancellation,
  timeout, device loss, cleanup, committed effects, resource limits, or the
  diagnostic source span for transfer failures.
- No rule defines whether cost is semantic, evidence-only, or a diagnostic hint,
  nor how a large transfer is reported without exposing timing, bandwidth,
  allocation addresses, driver text, or host paths as Ling semantics.
- No transfer protocol, schema, diagnostic allocation, dependency,
  target/toolchain selection, CLI command, or public support claim is required
  or changed by this audit. The public CLI and source extension remain ling
  and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A typed transfer operation and Effect Row/Capability contract with source and
   destination types, address spaces, shape/layout, byte-count semantics,
   ownership transition, result/error type, and stable Semantic-ID/source-span
   witnesses.
2. Device capability and backend discovery rules for DeviceTransfer, supported
   directions, limits, alignment, visibility/coherence, feature versions,
   unsupported-target behavior, and deterministic capability identity.
3. Transfer lifecycle semantics for synchronous and asynchronous completion,
   TransferToken and Fence/Event interaction, cancellation, timeout, device
   loss, Faults, drop, resource limits, and committed effect ordering.
4. Ownership and aliasing rules for source/destination buffers, views, subviews,
   concurrent reads/writes, actor/task crossing, and recovery after failure.
5. Canonical Semantic Graph/Audit fields and protocol lifecycle for bytes,
   address spaces, synchronization, Fault, capability, cost/evidence status,
   corruption, migration, redaction, and deterministic ordering.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism,
   capability, bounds, ownership, cancellation, and unsupported-target
   fixtures executable offline.

## Evidence and compatibility impact

The future implementation must consume checked Typed Core or a verified
derivative only and must never interpret unchecked AST nodes. Transfer facts
must preserve original UTF-8 byte spans, Semantic IDs, Unicode 17.0.0 behavior,
declared effects, ownership transitions, deterministic ordering, and explicit
Fault provenance. Hardware addresses, driver output, timing, bandwidth, and
debug text must remain outside Ling semantics.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DBUF-4403 implementation, transfer syntax and effect typing, capability and
address-space rules, lifecycle and cancellation runtime, Semantic Graph/Audit
transfer fields, transfer corpus, Unicode/source-map cases, editor integration,
and public protocol claims remain deferred until DBUF-4401/4402 and the
Kernel/device and ownership gaps are resolved by Accepted authority and
executable evidence exists.
