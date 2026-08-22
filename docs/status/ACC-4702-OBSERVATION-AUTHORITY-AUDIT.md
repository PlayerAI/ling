# ACC-4702-OBSERVATION Authority Audit — Experimental Accelerator-Adapter Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0171` authorizes test-local vocabulary only. No TPU/NPU adapter,
plugin package, graph bridge, target or support entry, cache/runtime API,
dependency, diagnostic allocation, public protocol, or support claim is
added. Experimental wording does not authorize semantics; ACC-4702 remains
`BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:425-427` permits an
  Experimental label and forbids vendor graph semantics in the core compiler,
  but is non-normative and does not define adapter input, ABI, capabilities,
  isolation, numeric/determinism, cache, Fault, trust, or evidence.
- `docs/ROADMAP-1.0.md:417-431` says accelerator extensions should reuse
  Kernel verification, shape/layout, and Placement through a narrow interface;
  only support-matrix backends are 1.0 gates. It does not authorize an
  implementation or waive semantic governance.
- ACC-4701 remains `BlockedSpec`; DIR-4501 through DIR-4503 and GPU-4601
  through GPU-4605 remain blocked. RFC-H404/H405 and RFC-0013 are not
  Accepted, and accelerator support entries are not implemented.

## Current implementation evidence

- No TPU/NPU adapter, plugin package/manifest, target or capability entry,
  graph-to-Device-IR bridge, runtime path, cache, Fault mapper, sandbox/trust
  boundary, or Experimental fixture exists under `crates` or `tests`.
- No accepted rule fixes supported operations/types, shape/layout constraints,
  numeric modes, device capability and target identity, fallback/rejection,
  resource ownership, versioning, cleanup, or compatibility of an
  Experimental artifact.
- No accepted policy defines how an Experimental adapter is isolated from the
  frontend, dependencies and toolchains are audited, network/host access is
  controlled, evidence is reproduced offline, or an adapter is promoted,
  revoked, or removed without stale support claims.
- No adapter diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this evidence. The public
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
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for unsupported operations/types, shape/layout or numeric mismatch,
   capability/target mismatch, loading/ABI/Fault, resource, cancellation,
   and adapter unavailability.
6. Offline positive/negative, malformed, migration, capability, cache,
   security, source-map/Unicode, determinism, differential, and lifecycle
   fixtures sufficient for every Experimental claim.

## Evidence and compatibility impact

The eventual adapter must remain an isolated, evidence-backed experiment and
must consume only accepted verified artifacts. Vendor graph semantics,
addresses, paths, timestamps, driver text, and debug output must not enter
Ling identity or cache keys. Experimental status must be visible and must not
be inferred as support by the compiler, CLI, or support matrix.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

TPU/NPU adapter and package, graph bridge, target and capability entries,
cache/runtime/Fault paths, supply-chain controls, Experimental support
evidence, editor support, and public protocol claims remain deferred until
ACC-4701 and the Kernel/Device IR, runtime, numeric, Native/backend, and
support-matrix authorities are Accepted.
