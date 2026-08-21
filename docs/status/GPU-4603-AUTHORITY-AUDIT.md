# GPU-4603 Authority Audit — Launch and Runtime

Status: BlockedSpec

Date: 2026-08-22

## Outcome

GPU-4603 proposes the runtime sequence for a device backend: device
discovery, capability matching, module loading, buffer binding, launch
dimensions, queue submission, synchronization, device-loss handling, resource
cleanup, and metrics/explain output.

No device runtime, scheduler, queue, module loader, buffer binder, discovery
API, launch command, cleanup path, metrics schema, or device-loss protocol can
be added yet. The accepted language subset excludes GPU behavior, while the
Kernel/Device IR, target/capability, ownership, synchronization, Fault,
placement, and runtime/ABI contracts are unresolved. Implementing this list
would silently choose observable semantics and resource guarantees.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:365-376` is a
  non-normative implementation plan. It enumerates runtime phases but does not
  define discovery identity, capability matching, module format, buffer
  ownership, launch/queue ordering, synchronization, device loss, cleanup,
  metrics, or explain protocol.
- `docs/ROADMAP-1.0.md:381-431` requires a supported GPU lowering path with
  transfer, launch, synchronization, Fault mapping, differential evidence,
  fallback/rejection, and an explicit support matrix. It does not authorize a
  runtime API or define scheduling/resource semantics.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 946-975, 1347-1381` reserve Kernel/device
  lowering and Placement while excluding Kernel, GPU, and Native behavior
  from v0.0.1. Scalar Interpreter/VM execution cannot be treated as a device
  runtime oracle.
- `DIR-4501`, `DIR-4502`, `DIR-4503`, `GPU-4601`, and `GPU-4602` are
  `BlockedSpec`. They provide no accepted Device IR, target specialization,
  adapter ABI, binary identity, or runtime handle contract.
- `GAP-KERNEL-DEVICE-001` is Open and leaves buffers, synchronization, numeric
  determinism, Placement, and backend capability discovery unresolved.
  `GAP-NATIVE-BACKEND-ABI-001` is Open for target packages, layout, ABI/FFI,
  Fault/unwinding, and cross-target support. `BACKEND-GPU` remains Unsupported,
  Experimental, and unimplemented in the support matrix.
- No `RFC-H404` or Accepted GPU runtime authority exists; RFC-0013 is only a
  candidate topic and cannot authorize a public runtime or metrics protocol.

## Current implementation evidence

- The repository has no device discovery, capability matcher, module loader,
  buffer binder, launch scheduler, queue or synchronization runtime,
  device-loss handler, cleanup manager, metrics/explain schema, or GPU runtime
  fixture under `crates` or `tests`.
- No accepted rule fixes device enumeration and stable identity, feature and
  capability negotiation, module/binary versioning, buffer layout and
  ownership, launch dimensions, queue ordering, synchronization visibility,
  cancellation, device loss, cleanup on Fault, or resource budgets.
- No accepted policy defines which metrics are diagnostic-only, how placement
  decisions are explained or replayed, how host/driver/toolchain facts affect
  cache identity, or how unsupported hardware is rejected without changing
  source semantics.
- No runtime diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A verified Device IR and binary boundary, with versioned module loading,
   source-map/Semantic-ID treatment, malformed-input rejection, and no
   unchecked AST or unverified IR execution.
2. Stable device and target identity plus capability discovery/matching,
   including feature versions, numeric/determinism class, layouts,
   workgroup/grid limits, queue families, synchronization support, and
   explicit fallback or rejection.
3. Buffer/resource ownership and lifetime rules, transfer visibility,
   launch-dimension validation, queue/stream ordering, synchronization,
   cancellation, device-loss behavior, cleanup on success/Error/Fault, and
   bounded resource budgets.
4. A runtime/ABI and Fault model separating host, backend, and device errors,
   with stable version negotiation and deterministic evidence; vendor and
   driver details must not leak into Ling identity.
5. A metrics/explain contract defining stable versus diagnostic-only fields,
   replayability, privacy, provenance, and exclusion of addresses, paths,
   timestamps, allocation order, and debug output from semantics.
6. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and offline positive/negative
   fixtures for discovery, capability mismatch, module corruption, binding,
   launch, synchronization, device loss, cleanup, cancellation, resource
   exhaustion, Unicode/source maps, migration, and CPU/device differential
   behavior.

## Evidence and compatibility impact

The eventual runtime must consume only a verified, versioned artifact and
must keep the frontend independent of vendor APIs. Discovery and placement
must be explicit and reproducible; unsupported capability or device loss must
produce declared rejection/Fault behavior rather than a silent fallback.
Metrics and explain output must remain diagnostic evidence, not new language
semantics or cache identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

GPU-4603 implementation, discovery and capability APIs, module loading,
buffer binding, launch/queue/synchronization runtime, device-loss and cleanup
paths, metrics/explain schemas, editor support, and public protocol claims
remain deferred until the Device IR, adapter, Kernel/device, and Native/backend
authorities are Accepted and executable evidence exists.
