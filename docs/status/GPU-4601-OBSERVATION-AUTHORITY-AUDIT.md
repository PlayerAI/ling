# GPU-4601-OBSERVATION Authority Audit — Backend Spike and Selection Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0165` authorizes test-local vocabulary only. No backend choice,
dependency, target package, toolchain probe, benchmark, capability API,
diagnostic allocation, public protocol, or support claim is added. GPU-4601
remains `BlockedSpec` for technology selection.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:328-349` names
  candidate technologies and evaluation dimensions but is non-normative and
  does not define a required backend, input artifact, target identity, ABI,
  numeric behavior, or selection semantics.
- `docs/ROADMAP-1.0.md:381-431` makes GPU lowering dependent on G4 Kernel
  gates, a backend-neutral Device IR (or explicitly reused layer), one
  supported backend, transfer/launch/synchronization/Fault semantics,
  differential evidence, and an explicit support matrix. It does not select a
  technology or authorize a public benchmark result.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` reserve Kernel/device lowering
  while excluding GPU and Native behavior from the v0.0.1 Seed subset.
- RFC-0013 is not Accepted, RFC-H404 is absent, and
  `GAP-KERNEL-DEVICE-001` plus `GAP-NATIVE-BACKEND-ABI-001` remain open.
  `BACKEND-GPU` is Unsupported/Experimental in
  `docs/governance/support-matrix.toml`.

## Current implementation evidence

- No GPU backend crate, Device IR consumer, target package,
  capability-discovery API, backend-selection pass, driver/toolchain probe,
  hardware matrix, benchmark harness, or GPU fixture exists under `crates`
  or `tests`.
- No accepted rule fixes the backend input artifact, lowering boundary,
  target/feature identity, numeric mode, floating-point tolerance,
  synchronization or transfer behavior, Device Fault mapping, resource
  limits, or fallback/rejection semantics.
- No accepted policy defines how a spike becomes Experimental, Preview, or
  Supported; how host/toolchain/driver versions affect cache identity; or how
  license and CI-hardware evidence is recorded without becoming Ling
  semantics.
- No GPU diagnostic allocation, protocol, dependency, CLI command, target,
  support claim, or public API is required or changed by this evidence. The
  public CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel and Device IR contract with a checked Typed Core or
   verified IR boundary, supported operations/types/effects, source spans and
   Semantic IDs, and explicit host/device isolation.
2. A target and capability model defining device identity, feature discovery,
   numeric/determinism classes, layout, workgroup/grid limits,
   synchronization, transfer effects, Faults, cancellation, and permitted
   fallback or rejection.
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

The eventual spike must be an evidence-only experiment around a versioned,
verified artifact. It must not choose a backend by benchmark output alone, and
it must not expose driver paths, timestamps, host addresses, allocation order,
debug text, or unversioned toolchain details as Ling identity. Any selected
backend must remain isolated behind accepted Device IR and runtime contracts,
with deterministic differential evidence and an explicit support status.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

Technology selection, backend dependencies and probes, target packages,
capability discovery, benchmark/hardware matrices, GPU lowering,
launch/runtime adapters, differential fixtures, editor support, and public
protocol claims remain deferred until the Kernel/Device IR and Native/backend
authorities are Accepted and executable evidence exists.
