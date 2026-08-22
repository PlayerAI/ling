# ACC-4701-OBSERVATION Authority Audit — Accelerator Plugin Interface Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0170` authorizes test-local vocabulary only. No plugin trait,
registry, loader, manifest, dependency, target package, cache API, diagnostic
allocation, public protocol, or support claim is added. ACC-4701 remains
`BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:412-423` lists
  declarations and the verified-input boundary but is non-normative and does
  not define Device IR, plugin ABI, capability/target/cache identity, trust,
  loading, versioning, fallback, diagnostics, or compatibility lifecycle.
- `docs/ROADMAP-1.0.md:381-431` allows accelerator extensions through a narrow
  interface that reuses Kernel verification, shape/layout, and Placement; it
  does not authorize a plugin API or unreviewed vendor semantics.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` exclude GPU, Native, and
  accelerator execution from the v0.0.1 Seed subset.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4605 remain
  `BlockedSpec`; RFC-H404/H405 and RFC-0013 are not Accepted. The
  Kernel/device and Native/backend gaps remain open, and accelerator support
  entries are not implemented.

## Current implementation evidence

- No accelerator plugin trait or registry, dynamic/static loader, manifest
  schema, capability declaration, target/cache identity, sandbox/trust
  boundary, diagnostic mapper, or accelerator fixture exists under `crates`
  or `tests`.
- No accepted rule fixes plugin input validation, verified-artifact trust,
  supported ops/types, shape/layout and numeric constraints,
  capability negotiation, fallback/rejection, version compatibility, cache
  invalidation, or plugin failure provenance.
- No accepted policy defines whether plugins are built in, dynamically loaded,
  or external; how dependencies and licenses are audited; or how a plugin may
  access host paths, devices, toolchains, and network resources without
  violating deterministic offline builds.
- No plugin diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this evidence. The public
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
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for malformed declarations, unsupported ops/types, shape/layout or numeric
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

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

Plugin ABI/registry/loader, capability manifests, cache and fallback schemas,
trust/supply-chain controls, diagnostics, accelerator adapters, editor support,
and public protocol claims remain deferred until Kernel/Device IR, runtime,
numeric, Native/backend, and support-matrix authorities are Accepted with
executable evidence.
