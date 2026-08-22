# GPU-4602-OBSERVATION Authority Audit — Backend Adapter Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0166` authorizes test-local vocabulary only. No adapter trait,
Device IR or DeviceBinary API, runtime handle, target package, dependency,
capability API, diagnostic allocation, public protocol, or support claim is
added. GPU-4602 remains `BlockedSpec` for the backend adapter.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:351-363` sketches
  adapter operations but is non-normative and does not define Device IR,
  target spec, binary format, ownership, queue ordering, capability
  vocabulary, ABI, versioning, Fault classes, or diagnostics.
- `docs/ROADMAP-1.0.md:381-431` places GPU lowering after Kernel/Device gates
  and requires a backend-neutral IR, one supported backend, transfers,
  launch, synchronization, Fault mapping, differential evidence, and an
  explicit support matrix. It does not authorize an adapter API.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` exclude Kernel, GPU, and Native
  behavior from the v0.0.1 Seed subset; scalar Interpreter/VM APIs are not a
  device runtime ABI.
- DIR-4501 through DIR-4503 remain `BlockedSpec`, RFC-0013 and RFC-H404 are
  not Accepted, and `GAP-KERNEL-DEVICE-001` plus
  `GAP-NATIVE-BACKEND-ABI-001` remain open. `BACKEND-GPU` is
  Unsupported/Experimental in the support matrix.

## Current implementation evidence

- No Device IR model, adapter trait, DeviceBinary schema, target package,
  buffer/queue handle, capability discovery, launch path, synchronization
  implementation, Fault mapper, or adapter fixture exists under `crates` or
  `tests`.
- No accepted rule fixes the input trust boundary, binary ownership/lifetime,
  transfer visibility, queue/stream ordering, launch dimensions,
  synchronization scope, device-loss behavior, cancellation, resource limits,
  or cleanup guarantees.
- No accepted rule defines target/feature normalization,
  compiler/runtime/driver identity, binary cache invalidation, ABI
  compatibility, opaque handle encoding, source-map preservation, numeric
  mode, or exact versus tolerance-based results.
- No adapter diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this evidence. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Device IR and lowering boundary accepting checked Typed Core or
   a verified derivative only, preserving required UTF-8 spans and Semantic
   IDs, and rejecting invalid IR before adapter entry.
2. A target specification and capability model covering device identity,
   features, numeric/determinism class, layout, workgroup/grid limits,
   transfer effects, synchronization, cancellation, resource budgets, and
   explicit fallback or rejection.
3. A backend-neutral adapter ABI for compilation, binary ownership, buffer
   allocation/transfer, launch, synchronization, cleanup, and device-loss
   handling, with version negotiation and host/backend isolation.
4. A versioned DeviceBinary and cache contract defining canonical bytes,
   target/compiler/toolchain inputs, migration and corruption rejection, and
   exclusion of paths, addresses, timestamps, debug output, and unstable
   driver text from Ling identity.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for unsupported capabilities, target/ABI mismatch, allocation/transfer/
   launch/synchronization failures, Faults, cancellation, resource limits,
   and toolchain errors.
6. Offline positive and negative adapter fixtures, malformed binary and
   migration cases, source-map/Unicode cases, deterministic lifecycle tests,
   and CPU-reference/device differential evidence for every claimed operation.

## Evidence and compatibility impact

The eventual adapter must be a thin consumer of verified Device IR and must
not become a second type checker or semantic interpreter. Vendor-specific
details remain behind the adapter boundary; a selected backend is
Experimental or Supported only according to the accepted support matrix.
Allocation order, host addresses, driver paths, timestamps, debug text, and
unversioned toolchain behavior must not become Ling semantics or cache identity.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

Adapter traits and handles, DeviceBinary and cache schemas, target packages,
capability discovery, allocation/transfer/launch/sync paths, Fault mapping,
vendor dependencies, editor support, and public protocol claims remain
deferred until DIR-4501 through DIR-4503 and the Kernel/device and
Native/backend authorities are Accepted and executable evidence exists.
