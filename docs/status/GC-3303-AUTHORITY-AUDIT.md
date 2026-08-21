# GC-3303 Authority Audit — Managed/Native/FFI Boundary

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

GC-3303 is a post-Seed boundary-design task. The execution plan lists pin/unpin,
a handle table, no raw-pointer escape, callback roots, thread attachment,
foreign ownership, collection during an FFI call, and the distinction between
deterministic cleanup and finalizers. These rules jointly constrain memory
safety, ABI, reentry, and resource lifetime; they cannot be inferred from Rust
references or from an implementation-specific collector.

No Managed handle, pinning API, raw-pointer wrapper, callback-root registry,
thread-attachment protocol, FFI ownership mode, ABI schema, diagnostic, or
placeholder Native/FFI crate is added. GC-3303 remains `BlockedSpec` until the
Managed object/collector authorities and the Native ABI/FFI decisions are
Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:271-280` is non-normative and
  lists the boundary questions; it does not define a safe handle representation
  or foreign-call contract.
- GC-3301 and GC-3302 are `BlockedSpec`. RFC-N303 is absent, so Managed
  identity, movement, roots, finalization, collection, and pinning inputs are
  not authoritative.
- RFC-N304 (Native ABI), RFC-N305 (Target Primitive/FFI), and RFC-N306 (Native
  backend support) are plan placeholders, not Accepted RFCs. RFC-0001 remains
  Draft under DEC-0018, and the governance gap register points to RFC-0011
  only as a future Native decision after RFC-0007.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves layout, calling convention,
  Fault/unwinding, thread/reentry, typed FFI, target packages, and target tiers
  unresolved. `GAP-OWNERSHIP-MODEL-001` leaves Managed/Resource identity,
  cleanup, roots, and Profile boundaries unresolved.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` sketch Managed Islands, Resource
  ownership, FFI/Native profiles, and capability boundaries, but do not specify
  pin lifetime, handle-generation safety, callback rooting, foreign ownership,
  ABI layout, or GC behavior while an FFI call is suspended.
- Accepted DEC-0009/RFC-0017 only constrain Seed mutable places and lowering;
  accepted runtime-fault and VM-host decisions do not create a Native ABI,
  FFI schema, or public raw-pointer facility. A reserved Managed/Native shape
  is not executable v0.0.1 behavior.

## Current implementation evidence

- The workspace contains no `ling-managed-runtime`, Native backend, ABI, FFI,
  handle-table, pinning, callback-root, or thread-attachment crate. The current
  evaluator/bytecode/VM paths operate on Seed checked Typed Core and host
  capabilities, not Managed pointers.
- There is no compiler representation for a pinned Managed reference, a
  generation-checked handle, a foreign ownership token, a callback root, a
  thread-attached runtime, or a GC-safe FFI call. No public ABI or FFI schema is
  registered.
- Existing JSON/host objects and Rust references are internal values. Their
  addresses, layout, destructor timing, and allocation strategy are not Ling
  semantics and cannot be used as a boundary contract.
- Existing tests cover Seed value/effect behavior, diagnostics, and
  interpreter/VM equivalence. They do not establish pinning, movement,
  reentrancy, foreign ownership, callback rooting, or cross-target behavior.

## Required authority before implementation

RFC-N303 together with the accepted Native/FFI decisions must define at least:

1. Managed identity and movement, stable handle representation and generation
   checks, pin/unpin lifetime and nesting, borrowed-view validity, and the rule
   that raw pointers never escape into Ling semantics or unchecked public APIs.
2. Handle-table ownership, allocation, stale-handle behavior, root promotion,
   callback roots, thread registration/attachment/detachment, and collection
   progress while foreign or host code is executing.
3. Foreign ownership modes for Value, Managed, Resource, and opaque handles;
   transfer/borrow/return rules; release and failure paths; finalizer versus
   deterministic Resource Drop; and bounded cleanup without Rust unwinding
   leakage.
4. ABI layout, calling convention, alignment, target/endianness policy,
   Fault/unwind and thread/reentry rules, capability and TCB boundaries,
   callback signatures, and versioned Target Primitive Package/FFI schemas.
5. Behavior when an FFI callback allocates, triggers collection, blocks,
   cancels, faults, or reenters an Actor/Task; safe-point and mailbox/turn
   invariants; and recovery/shutdown ordering.
6. Profile rules for Explore, Native Managed Islands, and Critical, including
   pinning and no-GC restrictions, security review, migration/compatibility,
   bilingual diagnostics, Semantic IDs, Typed Core/Graph/Audit Source, and
   Unicode 17.0.0 source-span preservation.

## Evidence and compatibility impact

The eventual implementation needs positive and negative handle/pin fixtures,
movement and stale-handle cases, nested pin/unpin cases, callback-root and
thread-attach cases, GC-during-FFI and reentry/cancellation/fault cases,
foreign ownership transfer and cleanup cases, raw-pointer escape rejection,
ABI/target and sanitizer evidence, and interpreter/VM/Native differential
traces. Evidence must be bounded and deterministic, exclude host addresses,
timing, allocator order, and thread scheduling from semantic comparisons, and
preserve original UTF-8 byte spans and registered diagnostic identities.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no FFI/ABI error code and introduces no
public handle, pointer, or protocol surface. Existing Seed tests and offline
build/test behavior remain unchanged.

## Intentionally deferred

Pin/unpin, handles, callback roots, thread attachment, foreign ownership,
GC-safe FFI calls, ABI layout and unwind/reentry behavior, deterministic
cleanup/finalization separation, Native Managed Islands, and all related
sanitizer, target, and differential fixtures remain deferred until the required
RFCs and governance protocols are Accepted.
