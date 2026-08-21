# GPU-4601 Authority Audit — Backend Spike and Selection

Status: BlockedSpec

Date: 2026-08-22

## Outcome

GPU-4601 proposes an isolated evaluation of possible device backend paths,
including SPIR-V/Vulkan, WGSL/WebGPU, CUDA/PTX, an MLIR-based bridge, and
vendor SDKs. The plan lists platform coverage, compiler/runtime API,
source/debug support, numeric control, CI hardware availability, and license
as comparison criteria.

No backend spike, dependency, target package, toolchain probe, benchmark,
adapter, or selection claim can be added yet. The Kernel and Device semantic
contract, Device IR boundary, target capability model, numeric/determinism
rules, runtime/ABI contract, and support-matrix policy are not Accepted. A
technology choice before those contracts exist would make an implementation
detail an accidental Ling protocol.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:328-349` is a
  non-normative implementation plan. It names candidate backend technologies
  and evaluation dimensions but does not define a required backend, the
  artifact it consumes, capability discovery, target identity, ABI, numeric
  behavior, or selection semantics.
- `docs/ROADMAP-1.0.md:381-431` makes GPU lowering dependent on the G4 Kernel
  gates, a backend-neutral Device IR (or an explicitly reused layer), one
  supported backend, transfer/launch/synchronization/Fault semantics,
  differential evidence, and an explicit support matrix. It does not select a
  technology or authorize a backend spike as a public implementation surface.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 1347-1381` reserve Kernel/device lowering while
  excluding Kernel, GPU, and Native behavior from the v0.0.1 Seed subset.
  Existing Interpreter/VM semantics cannot serve as a GPU backend contract.
- `GAP-KERNEL-DEVICE-001` is Open in `docs/governance/gap-register.toml` and
  leaves Kernel effects, buffers, synchronization, numeric determinism,
  Placement, and backend capability discovery unresolved. The candidate RFC is
  RFC-0013, which is not Accepted for this task.
- `GAP-NATIVE-BACKEND-ABI-001` is also Open and leaves target packages, data
  layout, ABI/FFI, Fault/unwinding, and cross-target support unresolved.
  `docs/governance/support-matrix.toml` marks `BACKEND-GPU` Unsupported and
  Experimental, implemented false, and blocked by `GAP-KERNEL-DEVICE-001`.
- No `RFC-H404` or other Accepted GPU/backend-selection authority exists. The
  plan therefore cannot override the higher-authority Seed boundary or create
  a public protocol.

## Current implementation evidence

- The repository has no GPU backend crate, Device IR consumer, target package,
  capability-discovery API, backend-selection pass, driver/toolchain probe,
  hardware matrix, benchmark harness, or GPU fixture under `crates` or
  `tests`.
- No accepted rule fixes the backend input artifact, lowering boundary,
  target/feature identity, numeric mode, floating-point tolerance,
  synchronization and transfer behavior, device Fault mapping, resource
  limits, or fallback/rejection semantics.
- No accepted policy defines how a spike result becomes an Experimental or
  Supported backend, how host/toolchain/driver versions affect cache identity,
  or how license and CI-hardware evidence is recorded without becoming Ling
  semantics.
- No GPU diagnostic allocation, protocol, dependency, CLI command, target,
  support claim, or public API is required or changed by this audit. The
  public CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel and Device IR contract with a checked Typed Core or
   verified IR boundary, supported operations/types/effects, source spans and
   Semantic IDs, and explicit host/device isolation.
2. A target and capability model defining device identity, feature discovery,
   numeric/determinism classes, layout, workgroup/grid limits, synchronization,
   transfer effects, Faults, cancellation, and permitted fallback or rejection.
3. A backend-selection and support-matrix lifecycle defining Experimental,
   Preview, Supported, and Unsupported states, required platform/driver/
   toolchain evidence, license obligations, reproducible offline evaluation,
   and no overclaim from an isolated spike.
4. A stable ABI/runtime boundary for launch, buffers, synchronization, errors,
   resource cleanup, and target/toolchain versioning; backend-specific facts
   must not leak into source semantics or unchecked AST execution.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unsupported targets/features, capability mismatch, transfer/launch/Fault
   failures, numeric mismatch, unavailable hardware, and toolchain errors.
6. Positive and negative CPU/GPU differential, determinism, malformed-input,
   source-map/Unicode, migration, and cross-target fixtures that run offline
   and justify every support-matrix claim.

## Evidence and compatibility impact

The eventual spike must be an evidence-only engineering experiment around a
versioned, verified artifact. It must not choose a backend by benchmark output
alone, and it must not expose driver paths, timestamps, host addresses,
allocation order, debug text, or unversioned toolchain details as Ling
identity. Any selected backend must remain isolated behind the accepted
Device IR and runtime contracts, with deterministic differential evidence and
an explicit support status.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

GPU-4601 implementation and technology selection, backend dependencies and
probes, target packages, capability discovery, benchmark/hardware matrices,
GPU lowering, launch/runtime adapters, differential fixtures, editor support,
and public protocol claims remain deferred until the Kernel/Device IR and
Native/backend authorities are Accepted and executable evidence exists.
