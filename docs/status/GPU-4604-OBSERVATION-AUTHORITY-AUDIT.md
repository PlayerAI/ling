# GPU-4604-OBSERVATION Authority Audit — Differential and Hardware-Matrix Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0168` authorizes test-local vocabulary only. No CPU/GPU
differential harness, hardware/software matrix schema, numeric comparator,
tolerance registry, hardware claim, diagnostic allocation, public protocol, or
support claim is added. GPU-4604 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:378-392` names
  matrix fields and CPU/GPU comparison but is non-normative and does not
  define canonical identities, numeric equivalence, tolerance, driver/toolchain
  policy, evidence retention, or stability gates.
- `docs/ROADMAP-1.0.md:381-431` requires CPU-reference/device differential
  tests and a support matrix listing verified combinations and Experimental
  backends. It does not authorize a matrix schema or claim any combination as
  stable.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` exclude Kernel and GPU behavior
  from the v0.0.1 Seed subset; no Seed conformance result is a GPU baseline.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4603 remain
  `BlockedSpec`; RFC-0013 and RFC-H404 are not Accepted. The Kernel/device and
  Native/backend gaps remain open, and CPU/GPU backend entries are not
  implemented support.

## Current implementation evidence

- No Kernel CPU reference, GPU differential harness, hardware/software matrix
  schema, combination identity, numeric comparator, tolerance registry,
  backend evidence bundle, or matrix fixtures exists under `crates` or `tests`.
- No accepted rule fixes input corpus and seed, work-item/reduction ordering,
  floating-point modes, exact versus tolerance-based equality, NaN/signed-zero
  handling, overflow/Fault equivalence, resource limits, or allowed
  differences.
- No accepted policy fixes which OS/GPU/architecture/runtime/driver/compiler
  fields are identity inputs, how versions are normalized, how unsupported or
  unavailable hardware is recorded, how evidence expires, or how a local run
  is prevented from becoming a public support claim.
- No matrix diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this evidence. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Versioned Kernel/Device IR and CPU-reference semantics with deterministic
   execution, checked-artifact boundaries, source spans/Semantic IDs, and an
   oracle relation for every supported GPU lowering.
2. Numeric and determinism classes covering precision, reduction/parallel
   ordering, NaN/signed-zero, overflow, rounding, tolerances, atomics/barriers,
   and exact Fault/unsupported behavior.
3. A canonical combination and evidence schema defining OS, device, vendor,
   architecture, runtime/driver, backend compiler/toolchain, numeric mode,
   feature set, layout, limits, known limitations, provenance, and expiry
   without exposing paths, addresses, timestamps, or unstable text as Ling
   identity.
4. Support-matrix lifecycle and gates for Unsupported, Experimental, Preview,
   and Stable, including required CI hardware/software evidence, reproduction,
   migration/corruption handling, license and availability constraints, and
   explicit fallback/rejection semantics.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics plus positive/negative,
   determinism, malformed-input, source-map/Unicode, CPU/GPU differential,
   cross-target, and resource/Fault fixtures executable offline where claimed.

## Evidence and compatibility impact

The eventual matrix must describe verified evidence rather than infer support
from a passing local run. Differential comparisons must use accepted
numeric/determinism rules and preserve provenance, while unsupported or
uncovered combinations remain explicit. Hardware and toolchain metadata may
be evidence inputs but must not become source semantics or unversioned cache
identity.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

CPU/GPU differential harnesses, numeric comparator and tolerance registry,
hardware/software matrix schema, stable-combination claims, evidence bundles,
editor support, and public protocol claims remain deferred until the
Kernel/Device IR, runtime, numeric, and support-matrix authorities are
Accepted and executable evidence exists.
