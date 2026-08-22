# CTR-5402-OBSERVATION Authority Audit — Contract Status-Model Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0193` permits only test-local Contract status-model
vocabulary. It does not authorize a status enum, lifecycle transition
table, aggregation policy, Graph/Audit/Evidence field, schema, renderer,
diagnostic, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:318-328` is a
  non-normative status checklist.
- `docs/status/CTR-5402-AUTHORITY-AUDIT.md` records the unresolved plan versus
  Draft `SEMANTICS.md` vocabulary and the missing lifecycle, identity,
  provenance, trust, and evidence contracts.
- `GAP-CRITICAL-PROFILE-001` remains open; RFC-K503 and a Contract
  proof/runtime/evidence replacement are not Accepted.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Contract
status, identity, evidence, lifecycle, projection, diagnostic, and fixture
boundaries. It sorts by explicit local rank, rejects duplicates, compares
canonical opaque bytes for forward/reverse input order, and uses an
observation-only tag. No status enum, transition table, evidence schema,
Graph/Audit field, diagnostic, CLI/LSP action, dependency, or support claim
is introduced.

## Required authority and compatibility

Accepted authority must define a versioned status vocabulary resolving the
plan/Draft conflict; meanings and legal transitions; composition,
precedence, aggregation, invalidation, stale/corrupt/revoked evidence and
migration; stable obligation/Contract/Graph/Audit/Evidence identities and
spans; provenance and trust; fail-closed behavior; UI text/accessibility;
stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics; and offline fixtures.
Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

CTR-5402 implementation, status lifecycle/enum, propagation fields and
schema, evidence/Graph/Audit projections, renderer/UI, diagnostics,
CLI/LSP/protocols, and support claims remain deferred until accepted
authority and executable offline evidence exist. No placeholder status API
is created.
