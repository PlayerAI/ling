# PROF-5103-OBSERVATION Authority Audit — Profile Composition Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0179` permits only test-local Profile Composition vocabulary.
It does not authorize a profile schema, merge algebra, precedence policy,
effective-profile digest, Semantic ID change, diagnostics, CLI option, or
support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:122-124` is a
  non-normative composition proposal.
- `docs/ROADMAP-1.0.md:145-149` does not define profile identity or authorize
  a Semantic ID migration.
- Accepted DEC-0012 remains the authority for Seed Definition/Body/Program ID
  domains and canonical bytes.
- `docs/status/PROF-5103-AUTHORITY-AUDIT.md` records open Critical Profile and
  Semantic Hash lifecycle gaps and missing RFC-K501/RFC-0012 authority.

## Current implementation evidence

The observation adds one isolated test with sixty explicit composition
boundaries, deterministic local ordering, duplicate rejection, and an opaque
observation tag. No production profile model, merge operator, digest, identity
input, dependency, diagnostic, CLI/LSP option, runtime, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define the versioned profile schema and layer scope,
merge/precedence/conflict algebra, effective-profile canonical bytes, relation
to build/cache/artifact/replay identity and Semantic IDs, configuration
precedence, migration, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
offline composition/identity fixtures. Seed identity, UTF-8 spans, CLI,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

PROF-5103 composition implementation, profile schema, canonical digest,
identity integration, diagnostics, CLI/LSP/editor integration, migration, and
public protocol claims remain deferred until RFC-0012 (or an Accepted
replacement), `GAP-CRITICAL-PROFILE-001`, `GAP-SEMANTIC-HASH-LIFECYCLE-001`,
and the G2/G3/G4 prerequisites are resolved with executable fixtures.
