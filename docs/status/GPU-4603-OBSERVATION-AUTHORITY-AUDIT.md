# GPU-4603-OBSERVATION Authority Audit — Launch and Runtime Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0167` authorizes test-local vocabulary only. No device runtime,
scheduler, discovery API, module loader, buffer binder, queue handle,
dependency, target package, metrics schema, diagnostic allocation, public
protocol, or support claim is added. GPU-4603 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:365-376` enumerates
  runtime phases but is non-normative and does not define discovery identity,
  capability matching, module format, buffer ownership, launch/queue ordering,
  synchronization, device loss, cleanup, metrics, or explain behavior.
- `docs/ROADMAP-1.0.md:381-431` requires a supported GPU lowering path with
  transfer, launch, synchronization, Fault mapping, differential evidence,
  fallback/rejection, and an explicit support matrix. It does not authorize a
  runtime API or scheduling semantics.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` exclude Kernel, GPU, and Native
  behavior from the v0.0.1 Seed subset; scalar VM execution is not a device
  runtime oracle.
- DIR-4501 through DIR-4503 and GPU-4601/4602 remain `BlockedSpec`; RFC-0013
  and RFC-H404 are not Accepted. `GAP-KERNEL-DEVICE-001` plus
  `GAP-NATIVE-BACKEND-ABI-001` remain open, and `BACKEND-GPU` is
  Unsupported/Experimental in the support matrix.

## Current implementation evidence

- No device discovery, capability matcher, module loader, buffer binder,
  launch scheduler, queue/synchronization runtime, device-loss handler,
  cleanup manager, metrics/explain schema, or GPU runtime fixture exists
  under `crates` or `tests`.
- No accepted rule fixes device enumeration and stable identity, feature and
  capability negotiation, module/binary versioning, buffer layout and
  ownership, launch dimensions, queue ordering, synchronization visibility,
  cancellation, device loss, cleanup on Fault, or resource budgets.
- No accepted policy defines which metrics are diagnostic-only, how placement
  decisions are explained or replayed, how host/driver/toolchain facts affect
  cache identity, or how unsupported hardware is rejected without changing
  source semantics.
- No runtime diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this evidence. The public
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

The eventual runtime must consume only a verified, versioned artifact and keep
the frontend independent of vendor APIs. Discovery and placement must be
explicit and reproducible; unsupported capability or device loss must produce
declared rejection/Fault behavior rather than a silent fallback. Metrics and
explain output must remain diagnostic evidence, not new language semantics or
cache identity.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

Discovery and capability APIs, module loading, buffer binding,
launch/queue/synchronization runtime, device-loss and cleanup paths,
metrics/explain schemas, editor support, and public protocol claims remain
deferred until the Device IR, adapter, Kernel/device, and Native/backend
authorities are Accepted and executable evidence exists.
