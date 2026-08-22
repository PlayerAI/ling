# PROF-5104-OBSERVATION Authority Audit — Profile Audit/LSP Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0180` permits only test-local Profile Audit/LSP vocabulary. It
does not authorize a checker, report schema, `ling` command, diagnostic
payload, LSP method, editor integration, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:126-136` is a
  non-normative proposal and its `zero` command names are stale.
- `docs/ROADMAP-1.0.md:142-151` does not define a Profile audit or editor
  protocol.
- Accepted DEC-0002/DEC-0012 preserve source-position and Semantic ID domains
  without authorizing this public surface.
- `docs/status/PROF-5104-AUTHORITY-AUDIT.md` records open Profile, LSP, and
  Semantic Transaction gaps.

## Current implementation evidence

The observation adds one isolated test with sixty explicit audit/LSP
boundaries, deterministic local ordering, duplicate rejection, and an opaque
observation tag. No production checker, report, diagnostic, CLI/LSP route,
dependency, runtime, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define the versioned audit schema, checked-fact and
source-span provenance, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, `ling`
CLI lifecycle and profile selection, and LSP initialization, document/version,
position, cancellation, stale-result, publication, limits, and error mapping.
Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

PROF-5104 implementation, Profile checker/report, command routes, diagnostics,
LSP/Zed integration, explanation/quick-fix behavior, migration, and public
protocol claims remain deferred until accepted Profile/RFC-0012 authority,
LSP/Semantic Transaction lifecycles, and executable offline fixtures exist.
Stale `zero` names remain prohibited.
