# PLC-4803 Authority Audit — Cost Model v0

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PLC-4803 proposes a conservative, explainable cost model using input bytes,
transfer bytes, operation count, memory footprint, launch overhead, device
occupancy hints, and deadline/energy policy metadata. It explicitly warns that
uncalibrated estimates must not be advertised as guarantees.

No cost model, estimator, units, calibration data, policy API, selection rule,
benchmark input, or cost schema can be added yet. Placement, Device IR,
capability, transfer/synchronization, numeric, runtime, profile, and support
contracts are unresolved, and no accepted authority defines whether cost is
diagnostic evidence, a selection input, a replay/cache input, or observable
program behavior. Implementing even a conservative model would freeze those
choices.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:466-478` is a
  non-normative implementation plan. It lists cost factors and a warning about
  uncalibrated estimates but does not define units, overflow, measurement
  source, calibration, uncertainty, comparability, policy precedence,
  determinism, or compatibility.
- `docs/ROADMAP-1.0.md:381-431` requires Placement cost information and
  explainable/replayable decisions, but it does not authorize a cost schema or
  promise performance, energy, deadline, occupancy, or transfer estimates.
- `docs/SEMANTICS.md:1473-1480, 1858-1931` and
  `docs/LANGUAGE.md:946-975, 1347-1381` reserve Placement/device behavior and
  exclude Kernel/GPU/Native from v0.0.1. Performance estimates cannot alter
  Seed semantics or diagnostics.
- `PLC-4801` and `PLC-4802` are `BlockedSpec`; Device IR, runtime, capability,
  numeric, and support authorities remain unresolved. `GAP-KERNEL-DEVICE-001`
  and `GAP-NATIVE-BACKEND-ABI-001` are Open.
- No `RFC-H405` or Accepted cost/profile authority exists. The plan's warning
  is not a schema or implementation authorization.

## Current implementation evidence

- The repository has no Placement cost model, estimator, cost units,
  calibration corpus, hardware model, measurement provenance, uncertainty or
  confidence representation, policy evaluator, explain field, or cost tests
  under `crates` or `tests`.
- No accepted rule fixes whether bytes/counts are static or dynamic, how
  transfer/launch/occupancy estimates interact with capabilities and buffers,
  how deadline/energy metadata is interpreted, or how missing/contradictory
  estimates are handled.
- No accepted policy defines whether cost inputs affect compile-time legality,
  runtime selection, fallback, cache identity, record/replay, Critical/Strict
  reproducibility, or only diagnostic output. Uncalibrated host timings must
  not become Ling identity.
- No cost diagnostic allocation, public protocol, dependency, CLI command,
  target, or support claim is required or changed by this audit. The public
  CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned cost schema with canonical units, domains, overflow/unknown
   handling, static/dynamic inputs, calibration/provenance, confidence and
   uncertainty, and deterministic serialization.
2. Placement and Device contracts defining which cost fields are admissible
   selection inputs, how policy/deadline/energy constraints compose, tie
   breaking, fallback, unavailable estimates, and conflict behavior.
3. Profile-specific rules for whether estimates are diagnostic-only or may
   influence runtime choice, and how Critical/Strict replay and cache identity
   remain reproducible when hardware or calibration changes.
4. Explain/replay/evidence schemas that distinguish estimates from guarantees,
   record source and version, support migration/corruption rejection, and
   exclude paths, addresses, timestamps, debug output, and unstable driver
   details from Ling identity.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unavailable/unknown estimates, invalid units, overflow, policy conflict,
   cost-limit, resource, and selection/replay mismatch.
6. Offline positive/negative, calibration, uncertainty, determinism,
   topology/capability, fallback, migration, explain/replay, source-map/
   Unicode, and CPU/device differential fixtures.

## Evidence and compatibility impact

The eventual cost model must remain an auditable estimate provider. It must not
claim performance or energy guarantees without calibrated evidence, and it
must never change language effects, ownership, numeric class, or Fault
semantics. Cost data may guide accepted policy selection and explain output
only under explicit authority; host measurements and unstable environment
facts must not become source or cache identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

PLC-4803 implementation, cost units/model/estimator, calibration and benchmark
corpus, policy integration, profile/replay/cache fields, explain output,
diagnostics, editor support, and public protocol claims remain deferred until
RFC-H405 (or an Accepted replacement), PLC-4801/4802, and the Kernel/Device
IR, runtime, numeric, Native/backend, and support-matrix authorities are
Accepted.
