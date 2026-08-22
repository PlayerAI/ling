# PLC-4805-OBSERVATION Authority Audit — Device-Binary-Cache Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0176` permits only test-local Device Binary Cache vocabulary.
It does not authorize a cache artifact, Device IR serialization, backend
identity, signing/trust API, cache namespace, migration, locking, eviction,
diagnostics, or public protocol. Accepted `DEC-0022` remains limited to its
disposable internal line-index payload.

## Traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:495-510` is a
  non-normative plan fragment.
- `docs/ROADMAP-1.0.md:421-431` requires cache correctness as a G4.6 goal but
  does not define cache behavior.
- `docs/status/PLC-4805-AUTHORITY-AUDIT.md` records missing Device IR,
  backend, identity, trust, lifecycle, and safe-recompile authority.
- `DEC-0022` is not widened; `DEC-0175` remains explain evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production cache, dependency, target, artifact, signature, migration,
diagnostic, CLI/LSP command, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define checked Device IR/backend artifacts, canonical
identity and key dimensions, validation/trust/signing, permissions/isolation,
publication/concurrency/eviction, safe miss/recompile and migration behavior,
privacy, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures.
Seed behavior, DEC-0022 line-index semantics, Semantic IDs, UTF-8 spans, CLI,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

PLC-4805 implementation, device-binary artifact/cache protocol, backend/driver
compatibility, trust, permissions, migration, locking, eviction, corruption
handling, diagnostics, editor integration, and public cache claims remain
deferred until the prerequisite authorities and executable fixtures exist.
