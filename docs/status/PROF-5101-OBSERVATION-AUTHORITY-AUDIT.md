# PROF-5101-OBSERVATION Authority Audit — Critical-Profile Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0177` permits only test-local Critical Profile vocabulary. It
does not authorize a profile format, parser, reader/writer, selection or
composition policy, proof permission, CLI option, diagnostics, or support.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:77-106` is a
  non-normative illustrative profile fragment.
- `docs/ROADMAP-1.0.md:118` requires G2/G3/G4 prerequisites and does not
  define a profile protocol.
- `docs/status/PROF-5101-AUTHORITY-AUDIT.md` records open
  `GAP-CRITICAL-PROFILE-001` and missing RFC-K501/RFC-0012 authority.
- `DEC-0176` remains prerequisite cache evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production profile, dependency, target, CLI/LSP option, diagnostic,
checker, proof API, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define schema/version/lifecycle, canonical identity,
field/default/unknown handling, profile binding and composition, project/CLI
precedence, forbidden capabilities, effect/memory/numeric/concurrency/FFI
policy, verification obligations, proof/evidence state, privacy, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures. Seed behavior,
Semantic IDs, UTF-8 spans, CLI, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

PROF-5101 implementation, profile syntax/schema, reader/writer, selection,
composition, diagnostics, CLI/editor integration, support claims, and
evidence-bundle fields remain deferred until RFC-K501/RFC-0012 (or an Accepted
replacement), `GAP-CRITICAL-PROFILE-001`, G2/G3/G4 prerequisites, and
executable fixtures are accepted.
