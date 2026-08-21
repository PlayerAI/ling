# ACC-4701 Authority Audit — Accelerator Plugin Interface

Status: BlockedSpec

Date: 2026-08-22

## Outcome

ACC-4701 proposes a plugin boundary for TPU, NPU, and other accelerators.
Plugins would consume only verified Device IR or Kernel Core and declare
supported operations/types, shape/layout constraints, numeric modes, device
capabilities, target identity, cache identity, fallback policy, and a
diagnostic mapper.

No plugin trait, registry, loader, capability manifest, cache protocol,
dependency, or accelerator adapter can be added yet. Device IR/Kernel Core,
target and capability identity, numeric/determinism rules, cache identity,
fallback, Fault/diagnostic, plugin trust, and support-matrix lifecycle are not
Accepted. A public extension point now would freeze vendor and supply-chain
semantics before the core contracts exist.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:410-423` is a
  non-normative implementation plan. It lists plugin declarations and the
  verified-input boundary but does not define the Device IR schema, plugin ABI,
  capability vocabulary, target/cache identity, trust or loading policy,
  versioning, fallback, diagnostics, or compatibility lifecycle.
- `docs/ROADMAP-1.0.md:381-431` allows accelerator extensions through a narrow
  interface that reuses Kernel verification, shape/layout, and Placement, and
  says only support-matrix backends are 1.0 gates; it does not authorize a
  plugin API or permit unreviewed vendor semantics in the compiler.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 1347-1381` reserve Kernel/device behavior while
  excluding GPU, Native, and accelerator execution from v0.0.1. The current
  Seed pipeline is not a plugin ABI.
- `GAP-KERNEL-DEVICE-001` is Open for Kernel/device semantics, numeric
  determinism, Placement, and capability discovery. `GAP-NATIVE-BACKEND-ABI-001`
  is Open for target packages, layout, ABI/FFI, Fault/unwinding, and
  cross-target support. The support matrix marks accelerator backends
  Unsupported and Experimental, implemented false.
- No `RFC-H404`/H405 or Accepted accelerator/plugin authority exists. RFC-0013
  is only a candidate topic, and `ACC-4702` is explicitly an Experimental
  follow-on rather than authorization for ACC-4701.

## Current implementation evidence

- The repository has no accelerator plugin trait or registry, dynamic/static
  loader, manifest schema, capability declaration, target/cache identity,
  sandbox/trust boundary, plugin diagnostic mapper, or accelerator fixtures
  under `crates` or `tests`.
- No accepted rule fixes plugin input validation, verified artifact trust,
  supported ops/types, shape/layout and numeric constraints, resource/ownership
  behavior, capability negotiation, fallback/rejection, version compatibility,
  cache invalidation, or plugin failure provenance.
- No accepted policy defines whether plugins are built in, dynamically loaded,
  or external; how dependencies and licenses are audited; or how a plugin may
  access host paths, devices, toolchains, and network resources without
  violating deterministic offline builds.
- No plugin diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Device IR/Kernel Core verification boundary, source-map and
   Semantic-ID treatment, operation/type/shape/layout rules, and rejection of
   unchecked or unverified plugin input.
2. A narrow plugin ABI and lifecycle defining declaration, loading, isolation,
   version negotiation, capability/target identity, resource ownership,
   cleanup, cancellation, Fault propagation, and host/backend boundaries.
3. Numeric/determinism, cache, and fallback contracts covering precision,
   layouts, limits, compiler/toolchain identity, invalidation, unsupported
   features, and explicit fallback or rejection without semantic drift.
4. A trust and supply-chain policy for dependencies, signatures/provenance,
   sandboxing, license evidence, offline hermetic builds, and removal or
   revocation of Experimental plugins.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   malformed declarations, unsupported ops/types, shape/layout or numeric
   mismatch, capability/target mismatch, load/ABI failure, resource/Fault,
   cancellation, and plugin unavailability.
6. Offline positive/negative, malformed, migration, source-map/Unicode,
   determinism, cache, capability, fallback, security, and CPU/device
   differential fixtures for each claimed plugin stage.

## Evidence and compatibility impact

The eventual interface must be a small, auditable consumer of verified
artifacts, not a path for vendor graph semantics or unchecked execution into
the front-end. Plugin details, addresses, paths, timestamps, driver text, and
debug output must not become Ling identity or cache keys. Support status must
be explicit and evidence-backed; Experimental adapters cannot imply 1.0
coverage.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

ACC-4701 implementation, plugin ABI/registry/loader, capability manifests,
cache and fallback schemas, trust/supply-chain controls, diagnostics,
accelerator adapters, editor support, and public protocol claims remain
deferred until Kernel/Device IR, runtime, numeric, Native/backend, and
support-matrix authorities are Accepted with executable evidence.
