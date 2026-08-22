# PLC-4803-OBSERVATION Authority Audit — Cost-Model Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0174` permits only test-local Cost Model vocabulary. It does
not authorize a cost estimator, units/schema, calibration corpus, benchmark
claim, policy API, profile/replay/cache field, diagnostic, or support claim.

## Traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:466-478` is a
  non-normative cost-factor sketch.
- `docs/ROADMAP-1.0.md:381-431` requires explainable Placement cost
  information but does not define a cost contract.
- `docs/status/PLC-4803-AUTHORITY-AUDIT.md` records missing units, calibration,
  uncertainty, policy, profile, replay, cache, and diagnostics authority.
- `DEC-0173` remains prerequisite selection evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production estimator, dependency, target, benchmark, cache/runtime
API, diagnostic, CLI/LSP command, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define canonical units and domains, static/dynamic
inputs, calibration/provenance/confidence/uncertainty, overflow/unknown
handling, policy and selection use, profile/replay/cache identity, estimate
versus guarantee boundaries, privacy, bilingual `L-<DOMAIN>-<NUMBER>`
diagnostics, and offline fixtures. Seed behavior, Semantic IDs, UTF-8 spans,
CLI, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

PLC-4803 implementation, estimator/model, calibration and benchmark corpus,
policy integration, profile/replay/cache fields, explain output, diagnostics,
editor support, and public Cost Model claims remain deferred until the
Placement/Device/backend authorities are Accepted.
