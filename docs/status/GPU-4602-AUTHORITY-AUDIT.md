# GPU-4602 Authority Audit — Backend Adapter

Status: BlockedSpec

Date: 2026-08-22

## Outcome

GPU-4602 proposes a narrow adapter around a selected device backend:
`compile(DeviceIR, TargetSpec) -> DeviceBinary`, allocation, transfer,
launch, synchronization, capability queries, Fault mapping, and stable
diagnostics. It also requires vendor-specific logic to stay outside the
front-end core.

No adapter trait, DeviceIR type, DeviceBinary schema, runtime handle, target
package, capability API, or vendor dependency can be added yet. The Device IR,
target/capability identity, buffer and synchronization semantics, binary/cache
contract, runtime ABI, and Fault/diagnostic mapping are not Accepted. Adding
the proposed signatures now would make a non-normative plan fragment a public
protocol and would force choices that later GPU and Kernel RFCs must govern.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:351-363` is a
  non-normative implementation plan. It sketches adapter operations and
  front-end isolation but does not define the Device IR schema, target spec,
  binary format, ownership, queue ordering, capability vocabulary, ABI,
  versioning, Fault classes, or diagnostic protocol.
- `docs/ROADMAP-1.0.md:381-431` places GPU lowering after Kernel/Device gates
  and requires a backend-neutral IR, one supported backend, transfers,
  launch, synchronization, Fault mapping, differential evidence, and an
  explicit support matrix. It does not authorize an adapter API or select a
  backend implementation.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 946-975, 1347-1381` reserve Kernel/device
  lowering and placement while excluding Kernel, GPU, and Native behavior
  from the v0.0.1 Seed subset. Existing scalar Interpreter/VM APIs are not a
  device runtime ABI.
- `DIR-4501`, `DIR-4502`, and `DIR-4503` are `BlockedSpec`; they do not define
  an accepted Device IR, lowering boundary, canonical identity, or target
  specialization. `GAP-KERNEL-DEVICE-001` leaves buffers, synchronization,
  numeric determinism, Placement, and backend capability unresolved.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves target packages, layout,
  ABI/FFI, Fault/unwinding, and cross-target support unresolved. The support
  matrix marks `BACKEND-GPU` Unsupported and Experimental, implemented false,
  and blocked by `GAP-KERNEL-DEVICE-001`.
- No `RFC-H404` or Accepted GPU/backend adapter authority exists. RFC-0013 is
  only a candidate topic in Draft RFC-0001 and cannot authorize a public
  adapter surface.

## Current implementation evidence

- The repository has no Device IR model, backend adapter trait, DeviceBinary
  schema, target package, buffer/queue handle, capability discovery, launch
  path, synchronization implementation, Fault mapper, or adapter fixture
  under `crates` or `tests`.
- No accepted rule fixes the input trust boundary, binary ownership and
  lifetime, transfer visibility, queue/stream ordering, launch dimensions,
  synchronization scope, device-loss behavior, cancellation, resource limits,
  or cleanup guarantees.
- No accepted rule defines target and feature normalization, compiler/runtime/
  driver identity, binary cache invalidation, ABI compatibility, opaque handle
  encoding, source-map preservation, numeric mode, or exact versus tolerance
  based results.
- No adapter diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Device IR and lowering boundary that accepts checked Typed Core
   or a verified derivative only, preserves required UTF-8 spans and Semantic
   IDs, and rejects invalid IR before adapter entry.
2. A target specification and capability model covering device identity,
   features, numeric/determinism class, layout, workgroup/grid limits,
   transfer effects, synchronization, cancellation, resource budgets, and
   explicit fallback or rejection.
3. A backend-neutral adapter ABI for compilation, binary ownership, buffer
   allocation and transfer, launch, synchronization, cleanup, and device-loss
   handling, with version negotiation and host/backend isolation.
4. A versioned DeviceBinary and cache contract defining canonical bytes,
   target/compiler/toolchain inputs, migration and corruption rejection, and
   exclusion of paths, addresses, timestamps, debug output, and unstable
   driver text from Ling identity.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unsupported capabilities, target/ABI mismatch, allocation/transfer/
   launch/synchronization failures, Faults, cancellation, resource limits,
   and toolchain errors.
6. Offline positive and negative adapter fixtures, malformed binary and
   migration cases, source-map/Unicode cases, deterministic lifecycle tests,
   and CPU-reference/device differential evidence for every claimed operation.

## Evidence and compatibility impact

The eventual adapter must be a thin consumer of a verified Device IR and must
not become a second type checker or semantic interpreter. Vendor-specific
details remain behind the adapter boundary; any selected backend is explicitly
Experimental or Supported only according to the accepted support matrix.
Allocation order, host addresses, driver paths, timestamps, debug text, and
unversioned toolchain behavior must not become Ling semantics or cache identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

GPU-4602 implementation, adapter traits and handles, DeviceBinary and cache
schemas, target packages, capability discovery, allocation/transfer/launch/
sync paths, Fault mapping, vendor dependencies, editor support, and public
protocol claims remain deferred until DIR-4501/4502/4503 and the Kernel/device
and Native/backend authorities are Accepted and executable evidence exists.
