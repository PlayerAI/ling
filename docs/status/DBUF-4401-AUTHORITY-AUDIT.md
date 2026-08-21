# DBUF-4401 Authority Audit — Device Types and Capability

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DBUF-4401 proposes DeviceId, DeviceKind, DeviceCapability, AddressSpace,
Buffer<Device, T, Shape>, ReadView, WriteView, TransferToken, and Fence/Event.
It also prohibits exposing arbitrary raw device pointers.

These concepts cannot be implemented as public Ling types or runtime APIs yet.
The execution plan does not define their type identity, ownership, address-space,
synchronization, capability discovery, transfer, Fault, or lifecycle semantics.
Adding them before the Kernel/device and ownership authorities are Accepted
would make an unstable hardware and memory model observable.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:221-238 lists
  proposed device names and the raw-pointer boundary, but does not define
  source syntax, Typed Core representation, identity, capability versions,
  address spaces, shape/layout, ownership, synchronization, transfer errors,
  or migration.
- docs/ROADMAP-1.0.md:381-429 requires Device Buffer ownership, address space,
  synchronization, transfer effects, capability discovery, and placement
  constraints for G4. It is a roadmap gate and not an Accepted Device type
  or public protocol contract.
- docs/SEMANTICS.md reserves Buffer<Device, T>, DeviceRegion, DeviceFault, and
  transfer concepts for future Kernel behavior while v0.0.1 remains the Seed
  subset. docs/LANGUAGE.md likewise excludes Device Buffer and Kernel from the
  first implementation.
- GAP-KERNEL-DEVICE-001 leaves buffer ownership/address spaces, synchronization,
  numeric determinism, Placement fallback, and backend capability unresolved.
  GAP-OWNERSHIP-MODEL-001 leaves Copy/Move, borrow exclusivity, aliasing, region
  escape, drop order, and Profile boundaries unresolved. Their candidate RFCs
  are not Accepted authorities.
- The support matrix marks the Kernel CPU reference/SIMD backend Unsupported
  and unimplemented, blocked by GAP-KERNEL-DEVICE-001. No accepted Device
  capability or execution backend exists.

## Current implementation evidence

- No DeviceId, DeviceKind, DeviceCapability, AddressSpace, Buffer, ReadView,
  WriteView, TransferToken, Fence/Event type, verifier, capability registry,
  or Device Buffer conformance corpus exists under crates or tests.
- No accepted rule fixes whether device identity is semantic or evidence-only,
  how capabilities are named/versioned, which address spaces are safe, how
  shapes and layouts are represented, or how raw pointers are rejected at
  the Typed Core and FFI boundaries.
- No ownership or synchronization contract defines exclusive writes, shared
  reads, subviews, mapping/pinning, asynchronous transfer lifetime, drop,
  cancellation, actor/task crossing, or Fence/Event ordering and Faults.
- No Device protocol, schema, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this audit. The public CLI and source extension remain ling and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A checked Typed Core representation for Device and Buffer values, with stable
   Semantic IDs, original UTF-8 spans, supported type/shape/layout rules,
   profile restrictions, effects, capabilities, and raw-pointer prohibition.
2. A versioned DeviceCapability and backend discovery contract defining device
   identity, kind, feature sets, address spaces, limits, cache identity,
   unsupported capability behavior, and deterministic ordering.
3. A Device Buffer ownership and view model covering host/device ownership,
   exclusive write, shared read, subviews, mapping, pinning, aliasing, transfer
   tokens, async lifetime, drop/cancel behavior, and actor/task crossing.
4. Synchronization and Fault rules for Fence/Event creation, waiting, ordering,
   cancellation, device loss, bounds, resource limits, and observable committed
   effects. Device faults must map to stable source spans and diagnostics.
5. A backend-neutral protocol/schema lifecycle with canonical encoding,
   corruption and migration behavior, exact identity rules, privacy
   redaction, and explicit Internal/Preview/Stable status.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism,
   CPU-reference, capability, ownership, transfer, and unsupported-target
   fixtures executable offline.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core or a verified
derivative only and must not interpret unchecked AST nodes. Device identity,
capabilities, buffer views, and synchronization must preserve Semantic IDs,
original UTF-8 byte spans, Unicode 17.0.0 behavior, deterministic ordering, and
declared effects. Host pointers, driver text, allocation addresses, timing, and
debug output must remain outside Ling semantics.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DBUF-4401 implementation, Device and Buffer types, capability registry, view and
token APIs, Fence/Event runtime, ownership and synchronization checks, transfer
fixtures, Unicode/source-map cases, editor integration, and public protocol
claims remain deferred until the Kernel/device and ownership gaps are resolved
by Accepted authority and the required executable evidence exists.
