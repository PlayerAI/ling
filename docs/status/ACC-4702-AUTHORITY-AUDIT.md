# ACC-4702 Authority Audit — Experimental Accelerator Adapter

Status: BlockedSpec

Date: 2026-08-22

## Outcome

ACC-4702 proposes that a first TPU/NPU adapter may remain Experimental and
need not block `v0.4 Stable` unless the support matrix explicitly includes it;
vendor graph semantics must not be copied into the core compiler.

No experimental adapter, plugin package, target entry, support claim, or
vendor graph bridge can be added yet. Experimental status limits release
claims but does not supply the missing Device IR/Kernel Core, plugin ABI,
capability, numeric, resource, Fault, cache, trust, and support-matrix
contracts. Without those authorities, an adapter would still define
unreviewed language-visible behavior and a new supply-chain boundary.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:425-427` is a
  non-normative implementation plan. It permits an Experimental label and
  forbids copying vendor graph semantics into the compiler, but does not
  define the adapter input, ABI, capability/target identity, isolation,
  numeric/determinism, cache, Fault, trust, or evidence contract.
- `docs/ROADMAP-1.0.md:417-431` says accelerator extensions should reuse
  Kernel verification, shape/layout, and Placement through a narrow interface;
  only support-matrix backends are 1.0 gates. It does not authorize an
  implementation or make Experimental behavior semantically unspecified.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 1347-1381` reserve accelerator lowering and
  exclude Kernel/GPU/Native behavior from v0.0.1. Experimental adapters must
  not extend the Seed language silently.
- `ACC-4701` is `BlockedSpec`; `DIR-4501` through `DIR-4503` and the GPU
  runtime tasks are also blocked. `GAP-KERNEL-DEVICE-001` and
  `GAP-NATIVE-BACKEND-ABI-001` remain Open, and accelerator entries in the
  support matrix are Unsupported/Experimental and unimplemented.
- No `RFC-H404`/H405 or Accepted accelerator authority exists. The plan's
  Experimental wording is not an accepted protocol or waiver of governance.

## Current implementation evidence

- The repository has no TPU/NPU adapter, plugin package/manifest, target or
  capability entry, graph-to-Device-IR bridge, runtime path, cache, Fault
  mapper, sandbox/trust boundary, or Experimental fixtures under `crates` or
  `tests`.
- No accepted rule fixes supported operations/types, shape/layout constraints,
  numeric modes, device capability and target identity, fallback/rejection,
  resource ownership, versioning, cleanup, or compatibility of an
  Experimental artifact.
- No accepted policy defines how an Experimental adapter is isolated from the
  front end, dependencies and toolchains are audited, network/host access is
  controlled, evidence is reproduced offline, or an adapter is promoted,
  revoked, or removed without stale support claims.
- No adapter diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A verified Device IR/Kernel Core input boundary and narrow plugin ABI,
   preserving source spans/Semantic IDs and rejecting unchecked or unverified
   vendor graph input.
2. Capability, target, shape/layout, numeric/determinism, resource,
   synchronization, Fault, fallback, and cache contracts shared with stable
   backends; Experimental status must not change their semantics.
3. An Experimental lifecycle and support-matrix schema defining required
   evidence, explicit limitations, versioning, reproducibility, promotion,
   deprecation, revocation, and non-1.0 status.
4. Trust and supply-chain controls for plugin provenance, dependencies,
   signatures, sandboxing, licenses, hermetic/offline builds, host/device
   isolation, and no vendor graph semantics in core compiler code.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unsupported operations/types, shape/layout or numeric mismatch,
   capability/target mismatch, loading/ABI/Fault, resource, cancellation, and
   adapter unavailability.
6. Offline positive/negative, malformed, migration, capability, cache,
   security, source-map/Unicode, determinism, differential, and lifecycle
   fixtures sufficient for every Experimental claim.

## Evidence and compatibility impact

The eventual adapter must remain an isolated, evidence-backed experiment and
must consume only accepted verified artifacts. Vendor graph semantics,
addresses, paths, timestamps, driver text, and debug output must not enter
Ling identity or cache keys. Experimental status must be visible and must not
be inferred as support by the compiler, CLI, or support matrix.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

ACC-4702 implementation, TPU/NPU adapter and package, graph bridge, target and
capability entries, cache/runtime/Fault paths, supply-chain controls,
Experimental support evidence, editor support, and public protocol claims
remain deferred until ACC-4701 and the Kernel/Device IR, runtime, numeric,
Native/backend, and support-matrix authorities are Accepted.
