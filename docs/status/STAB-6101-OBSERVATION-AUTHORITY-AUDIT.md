# STAB-6101-OBSERVATION Authority Audit — Support-Matrix Item Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0216` permits only test-local support-matrix-item-audit
vocabulary. It does not define candidate Stable items, promotion/demotion,
compatibility, public audit results, diagnostics, release binding, protocols,
or support claims.

## Traceability

- `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:32-49` is a
  non-normative per-item template.
- `docs/status/STAB-6101-AUTHORITY-AUDIT.md` records missing candidate,
  authority, traceability, compatibility, evidence, release, and fixture
  boundaries.
- The current support matrix is `1.0-draft`; no feature/profile/target row is
  authorized as a 1.0 Stable commitment.
- Incomplete G1-G5 exits and open gaps prevent a complete Stable surface; the
  existing support verifier validates draft consistency only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit item identity,
authority, compiler/execution, conformance/editor, compatibility/evidence,
traceability, support-state, failure, and fixture boundaries. It sorts by
explicit local rank, rejects duplicates, compares opaque bytes for forward/
reverse input order, and keeps every support state and fail-closed category
distinct. `cargo xtask support verify` continues to validate the unchanged
draft matrix. No row, promotion, diagnostic, protocol, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define the candidate Stable inventory, identity,
promotion/demotion, per-row normative clauses and executable evidence,
bidirectional traceability, compatibility/migration, release binding,
independent review, fail-closed results, bilingual diagnostics, and offline
fixtures. Seed behavior, current support states, Semantic IDs, original UTF-8
spans, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

STAB-6101 Stable-surface audit, candidate registry, row promotion/demotion,
compatibility promises, release evidence, diagnostics, protocols, and public
support remain deferred until Accepted G6 authority and complete per-item
evidence exist. No placeholder Stable matrix API is created.
