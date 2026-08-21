# DBUF-4402 Authority Audit — Buffer Ownership

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DBUF-4402 proposes host/device ownership, exclusive writes, shared reads,
subviews, mapping, pinning, asynchronous transfer lifetime, drop waiting or
cancellation, and actor/task crossing rules.

These rules cannot be implemented yet. They depend on an Accepted ownership and
region calculus, Device Buffer address-space and transfer semantics, and
concurrency lifecycle contracts. A partial borrow or drop implementation would
bind aliasing, cleanup, scheduling, and hardware behavior without a normative
authority.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:240-250 lists the
  ownership topics but does not define source syntax, Checked Core operations,
  ownership state transitions, view identity, alias proofs, pinning guarantees,
  async completion, cancellation, drop ordering, or task/actor transfer.
- docs/ROADMAP-1.0.md:381-429 requires Device Buffer ownership, address-space,
  synchronization, and transfer effects for G4. It is a roadmap gate and not an
  Accepted ownership or concurrency contract.
- docs/SEMANTICS.md reserves DeviceRegion, Buffer<Device, T>, transfer, and
  DeviceFault concepts for future behavior; v0.0.1 excludes Kernel, Device
  Buffer, and ownership implementation. docs/LANGUAGE.md also reserves these
  capabilities for later profiles.
- GAP-OWNERSHIP-MODEL-001 leaves Copy/Move, borrow exclusivity, aliasing,
  region escape, drop order, Managed roots, and Profile boundaries unresolved.
  GAP-KERNEL-DEVICE-001 leaves buffer ownership/address spaces,
  synchronization, and backend capability unresolved. Their candidate RFCs are
  not Accepted authorities.
- DBUF-4401 is BlockedSpec because Device, Buffer, capability, and address-space
  identity are absent. DBUF-4402 cannot define ownership over missing semantic
  resource types.

## Current implementation evidence

- No Device Buffer ownership checker, state machine, borrow/view type,
  subview proof, mapping or pinning operation, async transfer lifetime model,
  drop/cancel implementation, or actor/task crossing test exists under crates
  or tests.
- No accepted rule fixes whether host/device ownership is affine, linear, or
  otherwise restricted; whether shared reads can overlap transfers; how
  exclusive writes are proven; or how subviews preserve bounds, layout, and
  aliases.
- No contract defines whether mapping or pinning is blocking, effectful, or
  cancelable; how drop waits for or cancels pending work; how cancellation
  commits effects; how device loss maps to Fault; or how resources cross task
  and actor boundaries.
- No ownership protocol, schema, diagnostic allocation, dependency,
  target/toolchain selection, CLI command, or public support claim is required
  or changed by this audit. The public CLI and source extension remain ling
  and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A Value/Managed/Resource/Buffer ownership calculus and Checked Core witness
   for Copy, Move, borrow, exclusive write, shared read, aliasing, region
   escape, subview bounds/layout, drop order, and Profile restrictions.
2. Device address-space and transfer semantics that identify host/device
   ownership, mapping, pinning, visibility, coherence, asynchronous completion,
   and TransferToken identity without exposing raw pointers.
3. A lifecycle state machine for pending transfers and kernel use, including
   normal return, Error, Fault, cancellation, timeout, device loss, and drop;
   every path must have deterministic cleanup and explicit committed effects.
4. Actor/task crossing rules for affine resources, views, capabilities, and
   cancellation; define whether transfer is move, borrow, share, or prohibited
   and preserve stable Semantic IDs and original UTF-8 source spans.
5. Bilingual L-<DOMAIN>-<NUMBER> diagnostics and structured facts for
   use-after-move, conflicting borrow, aliasing, out-of-bounds view, invalid
   mapping/pinning, pending-drop, cancellation, unsupported crossing, and
   device/resource Faults.
6. Positive, negative, property, corruption, migration, Unicode/source-map,
   determinism, drop-order, transfer, cancellation, resource-limit, and
   actor/task crossing fixtures executable offline.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core or a verified
derivative only and must never interpret unchecked AST nodes. Ownership facts,
view identities, and cleanup events must preserve Semantic IDs, original UTF-8
byte spans, Unicode 17.0.0 behavior, deterministic ordering, and declared
effects. Host pointers, allocation addresses, driver details, timing, and debug
output must remain outside Ling semantics.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DBUF-4402 implementation, ownership and borrow checks, view/subview APIs,
mapping/pinning, transfer lifetime and cancellation, deterministic drop logic,
actor/task crossing, Unicode/source-map cases, editor integration, and public
protocol claims remain deferred until DBUF-4401, GAP-OWNERSHIP-MODEL-001, and
GAP-KERNEL-DEVICE-001 are resolved by Accepted authority and executable
evidence exists.
