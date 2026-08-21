# GPU-4604 Authority Audit — Differential and Hardware Matrix

Status: BlockedSpec

Date: 2026-08-22

## Outcome

GPU-4604 proposes recording each stable device combination as an OS, GPU,
architecture, runtime/driver, backend compiler, numeric mode, and known
limitations tuple, comparing it with the CPU reference, and marking uncovered
combinations Experimental rather than assuming support.

No hardware matrix, differential harness, stable-combination record, numeric
comparison rule, or support claim can be added yet. The CPU Kernel reference,
Device IR, GPU runtime, numeric/determinism classes, Fault/transfer semantics,
and support-matrix lifecycle are not Accepted. A matrix without those
contracts would turn local machine observations into compatibility promises.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:378-392` is a
  non-normative implementation plan. It names matrix fields and CPU/GPU
  comparison but does not define canonical identities, numeric equivalence,
  tolerance, driver/toolchain policy, evidence retention, or stability gates.
- `docs/ROADMAP-1.0.md:381-431` requires CPU reference/device differential
  tests and a support matrix that lists verified hardware/software combinations
  and Experimental backends. It does not authorize a matrix schema or claim
  any GPU combination as stable.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:883-890, 1347-1381` reserve Kernel/device lowering while
  excluding Kernel and GPU behavior from v0.0.1. No Seed conformance result is
  a GPU differential baseline.
- `DIR-4501`, `DIR-4502`, `DIR-4503`, `GPU-4601`, `GPU-4602`, and `GPU-4603`
  are `BlockedSpec`; there is no accepted Device IR, target specialization,
  adapter, binary, launch, or runtime contract to compare.
- `GAP-KERNEL-DEVICE-001` is Open for Kernel/device operations, synchronization,
  numeric determinism, Placement, and capability discovery. The support matrix
  marks `BACKEND-KERNEL-CPU` and `BACKEND-GPU` Unsupported and Experimental,
  implemented false. `GAP-NATIVE-BACKEND-ABI-001` remains Open for target and
  ABI evidence.
- No `RFC-H404` or Accepted GPU differential/matrix authority exists. The
  candidate RFC topics in Draft RFC-0001 cannot authorize stable combinations.

## Current implementation evidence

- The repository has no Kernel CPU reference, GPU differential harness,
  hardware/software matrix schema, combination identity, numeric comparator,
  tolerance registry, backend evidence bundle, or matrix fixtures under
  `crates` or `tests`.
- No accepted rule fixes input corpus and seed, work-item/reduction ordering,
  floating-point modes, exact versus tolerance-based equality, NaN/signed-zero
  handling, overflow/Fault equivalence, resource limits, or allowed differences.
- No accepted policy fixes which OS/GPU/architecture/runtime/driver/compiler
  fields are identity inputs, how versions are normalized, how unsupported or
  unavailable hardware is recorded, how hardware evidence expires, or how a
  local run is prevented from becoming a public support claim.
- No matrix diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
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
   feature set, layout, limits, versions, known limitations, provenance, and
   expiry without exposing paths, addresses, timestamps, or unstable text as
   Ling identity.
4. Support-matrix lifecycle and gates for Unsupported, Experimental, Preview,
   and Stable, including required CI hardware/software evidence, reproduction,
   migration/corruption handling, license and availability constraints, and
   explicit fallback/rejection semantics.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics plus positive/negative,
   determinism, malformed-input, source-map/Unicode, CPU/GPU differential,
   cross-target, and resource/Fault fixtures executable offline where claimed.

## Evidence and compatibility impact

The eventual matrix must describe verified evidence rather than infer support
from a passing local run. Differential comparisons must use the accepted
numeric/determinism rules and preserve provenance, while unsupported or
uncovered combinations remain explicit. Hardware and toolchain metadata may be
evidence inputs but must not become source semantics or unversioned cache
identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

GPU-4604 implementation, CPU/GPU differential harnesses, numeric comparator and
tolerance registry, hardware/software matrix schema, stable-combination claims,
evidence bundles, editor support, and public protocol claims remain deferred
until the Kernel/Device IR, runtime, numeric, and support-matrix authorities
are Accepted and executable evidence exists.
