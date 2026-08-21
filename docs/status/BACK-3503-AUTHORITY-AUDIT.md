# BACK-3503 Authority Audit — Native Runtime ABI

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

BACK-3503 proposes freezing a first internal ABI for Value passing, ADT tags,
closure environments, Fault/Result, GC handles, Resource Drop, Task/Actor
calls, and String/Text. The plan correctly says that an internal ABI may be
versioned and is not automatically a stable public ABI, but the representation
and failure rules still bind the compiler, runtime, Native backend, Managed
collector, ownership checker, FFI, and profiles.

No runtime ABI record, calling convention, layout table, runtime library,
version marker, handle/drop shim, Task/Actor call surface, diagnostic, or
placeholder crate is added. BACK-3503 remains `BlockedSpec` until the Native
ABI, memory/ownership/Managed, concurrency, FFI, and Profile authorities are
Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:365-378` is non-normative;
  its “internal ABI” wording does not define a representation or compatibility
  promise.
- BACK-3501/BACK-3502 and NIR-3401 through NIR-3403 are `BlockedSpec`. RFC-N304,
  RFC-N306, and candidate RFC-0011 Native authority are absent or not Accepted.
- `GAP-NATIVE-BACKEND-ABI-001` leaves layout, calling convention,
  Fault/unwinding, thread/reentry, typed FFI, target packages, and target tiers
  unresolved. `GAP-OWNERSHIP-MODEL-001` leaves Value/Resource/Managed identity,
  ownership, cleanup, and Profile behavior unresolved.
- `GAP-STRUCTURED-TASK-001` and `GAP-ACTOR-AWAIT-REENTRY-001` leave Task/Actor
  calls, suspension, cancellation, reentry, Fault aggregation, and cleanup
  ordering unresolved.
- RFC-0001 remains Draft and v0.0.1 excludes Native backend, Resource/Managed,
  structured concurrency, and Actor runtime. Accepted Seed decisions define
  only checked Typed Core and the existing interpreter/VM boundary.

## Current implementation evidence

- The workspace has no Native runtime library, ABI manifest, calling-convention
  implementation, GC handle, Resource-drop shim, Task/Actor ABI, or native
  String/Text representation. Existing VM values and bytecode calls are not a
  Native ABI.
- No target machine, alignment/data-layout, unwind/cancellation, thread/reentry,
  FFI, or runtime-version contract is registered. No public ABI schema is in
  the protocol inventory.
- Existing tests exercise Seed values, effects, diagnostics, and interpreter/VM
  equivalence, not cross-compiler/runtime ABI compatibility.
- Rust struct layout, enum discriminants, allocator addresses, unwinding,
  destructor timing, and string representation are implementation details and
  cannot be frozen by copying them into an ABI table.

## Required authority before implementation

The accepted Native/memory/concurrency decisions must define:

1. Value passing and data layout for primitives, records/tuples, ADTs/tags,
   closures/environments, String/Text/Bytes, aggregates, alignment,
   endianness, ownership, and target-specific exceptions.
2. Fault/Result representation, unwinding and recovery, cancellation,
   shutdown, thread/reentry, Task/Actor call and mailbox/turn boundaries, and
   the distinction between language Faults and host/toolchain failures.
3. GC handle identity, root/barrier/pin interaction; Resource ownership and
   deterministic Drop; borrow/region and FFI transfer; callback and foreign
   thread attachment; and profile/Managed-Island legality.
4. ABI versioning, compiler/runtime compatibility, migration, feature
   negotiation, symbol/name mangling, debug/source mapping, schema ownership,
   and the explicit rule that this ABI is internal unless a separate accepted
   public protocol exists.
5. Deterministic serialization/metadata where applicable, stable bilingual
   diagnostics for unsupported/invalid ABI forms, Unicode/source-span
   identity, security/TCB, offline build inputs, and interpreter/VM/Native
   semantic-preservation evidence.

## Evidence and compatibility impact

The eventual implementation needs ABI fixtures for every value/ADT/closure,
Fault/Result, GC handle, Resource Drop, Task/Actor, and String/Text case;
version mismatch and unsupported-target cases; cross-compiler/runtime and
cross-target tests; FFI, cancellation/reentry, sanitizer, cleanup, and
interpreter/VM/Native differential traces. Outputs must be deterministic and
exclude host addresses, allocation order, platform paths, timestamps, map order,
and debug noise from Semantic IDs or public compatibility.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, diagnostic registry, schema, Semantic ID,
source span, runtime, or Unicode behavior. It freezes no layout, installs no
runtime, allocates no ABI diagnostic, and adds no public protocol.

## Intentionally deferred

Value/ADT/closure/text layout, Fault/Result and unwind ABI, GC handles,
Resource Drop, Task/Actor calls, calling conventions, versioning/migration,
debug mapping, FFI/thread/reentry behavior, runtime libraries, and all ABI
compatibility evidence remain deferred until the required RFCs and governance
protocols are Accepted.
