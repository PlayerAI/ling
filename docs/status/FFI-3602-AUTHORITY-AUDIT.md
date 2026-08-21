# FFI-3602 Authority Audit — Minimal C ABI Interoperability

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

FFI-3602 proposes a deliberately small C interoperability surface: scalars,
fixed-layout records, pointer-plus-length spans, callbacks, opaque handles,
explicit allocator pairs, and error-code mapping, with compile-time rejection
of variadics, bitfields, and platform-dependent layouts. This is a
non-normative execution-plan proposal, not an accepted C ABI or target
contract.

No C declaration syntax, header/import model, layout calculator, linker or
compiler probe, callback trampoline, handle runtime, allocator bridge, error
adapter, target matrix, or public ABI schema is added. The v0.0.1 Seed
compiler, interpreter, bytecode, VM, and diagnostics remain unchanged until
the Native ABI, ownership, target, runtime, and protocol decisions are
Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:431-443` is non-normative.
  Its proposed C subset does not define C type widths, calling conventions,
  record layout, pointer validity, callback lifetime, allocator provenance,
  error transport, or target selection.
- `docs/SEMANTICS.md:1831-1868` requires a future FFI boundary to define ABI,
  ownership, lifetime, mutability, thread safety, reentrancy, Error/Fault,
  Capability, blocking, Target, and verification state; it does not accept a
  C ABI implementation or make a host ABI Ling semantics.
- `docs/LANGUAGE.md:1276-1287` requires Typed FFI and independent Target
  Primitive Packages. `docs/SEMANTICS.md:1872-1931` excludes Native backend
  support from the v0.0.1 formal subset, so the Seed grammar cannot be
  expanded by this task.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open and blocks FFI-3602. It records
  layout, ABI, unwinding/Fault, thread/reentry, typed FFI, Target Primitive,
  and target-tier behavior as unaccepted, with RFC-0011 deferred until
  RFC-0007 defines the memory categories exposed to Native/FFI.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain
  Open. Copy/move, borrow, region, drop, Managed/Resource, and exported
  lifetime rules cannot be inferred from C pointer or allocator conventions.
- `docs/governance/protocol-inventory.toml:495-515` registers `PROTO-ABI` as
  Planned public with no version, schema, reader/writer policy, migration
  tool, or fixtures. Its writer policy explicitly leaves layout, calling
  convention, ownership transfer, exceptions/Faults, target identity, and
  symbol versioning to accepted RFCs.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no accepted C ABI model, C header/import reader, layout
  verifier, target/linker probe, foreign compiler harness, callback or opaque
  handle runtime, allocator-pair contract, or executable FFI boundary.
  Existing Rust ABI behavior and host platform details are implementation
  facts, not evidence for a Ling C ABI.
- No v0.0.1 Seed type has a foreign layout or address semantics. Adding
  pointer, callback, handle, or allocator behavior now would expose ownership
  and unsafe lifetime choices before they are specified.
- No C toolchain, target matrix, generated header, dependency, linker, unsafe
  boundary, diagnostic allocation, or public protocol implementation is
  required for this authority audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The supported C ABI version(s), target triples and toolchain assumptions,
   scalar widths and representations, calling convention, endianness,
   alignment, record layout/packing, symbol naming/versioning, and the exact
   rejection rules for variadic, bitfield, flexible-array, union, and other
   platform-dependent forms.
2. Safe span rules for pointer-plus-length values (null, zero length,
   overflow, provenance, bounds, mutability, encoding, and thread use),
   callback calling/lifetime/reentrancy/cancellation behavior, opaque-handle
   identity and thread affinity, and cross-module allocator ownership and
   deallocation.
3. Ownership, borrow duration, Resource/Managed lifetime, drop/finalizer,
   aliasing, Error/Fault/unwinding, blocking, and Capability/Profile rules
   that connect the C boundary to checked Typed Core and runtime behavior.
4. A verified declaration-to-layout and declaration-to-link lowering boundary,
   deterministic diagnostics and source/Semantic-ID mapping, offline and
   reproducible toolchain/linker inputs, provenance/license/TCB policy, and
   cross-target compatibility and migration rules.
5. The versioned `PROTO-ABI` schema, canonical encoding, compatibility policy,
   public reader/verifier behavior, and executable C headers/fixtures before
   any binary compatibility or support claim is made.

## Evidence and compatibility impact

The eventual implementation needs positive and negative C declaration and
layout fixtures; independent compiler/header and linker conformance; scalar,
record, span, callback, handle, allocator, and error-code cases; null,
overflow, aliasing, lifetime, thread/reentry, blocking, and Fault rejection;
unsupported variadic/bitfield/platform-layout tests; cross-target and
cross-process evidence; sanitizer/fuzz coverage; schema migration and
provenance checks; and deterministic diagnostics. It must retain original
UTF-8 byte spans, stable Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>`
diagnostics, and Unicode 17.0.0 behavior without exposing C/Rust layout,
addresses, paths, hash-map order, or host ABI as Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime, or
Unicode behavior. It adds no C ABI syntax, header, layout code, target or
linker integration, dependency, toolchain, diagnostic, public protocol
implementation, or placeholder API.

## Intentionally deferred

C ABI declaration/import syntax, layout and calling-convention verification,
pointer-span and callback safety, opaque handles, allocator pairs, symbol and
error mapping, target/linker support, ownership and lifetime lowering,
sanitizer/fuzz/differential harnesses, protocol schema/fixtures, and all
Native/FFI compatibility claims remain deferred until RFC-N305 and the
dependent ownership, Native ABI, target, runtime, and `PROTO-ABI` authorities
are Accepted.
