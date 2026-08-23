# EVD-5804-OBSERVATION Authority Audit — AI Provenance Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0212` permits only test-local AI-provenance vocabulary. It does
not authorize provenance records, agent/tool/reviewer identity, conversation
capture, privacy/redaction/retention policy, human approval, correctness or
proof claims, bundle linkage, diagnostics, protocols, or support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:566-581` is a
  non-normative traceability checklist with an explicit non-correctness claim.
- `docs/status/EVD-5804-AUTHORITY-AUDIT.md` records missing schema, identity,
  privacy, approval, trust, retention, migration, and fixture authority.
- RFC-K507 is absent; `PROTO-EVIDENCE` is Future;
  `GAP-CRITICAL-PROFILE-001` remains open.
- `PROTO-SEMANTIC-GRAPH-JSON` is Experimental and authorizes no provenance or
  approval behavior; `DEC-0211` provides only test-local build vocabulary.

## Current implementation evidence

The observation adds one isolated test with sixty explicit actor, semantic-
linkage, change/verification, human-review, non-claim, privacy, trust,
failure, diagnostic, and fixture boundaries. It sorts by explicit local rank,
rejects duplicates, compares opaque bytes for forward/reverse input order, and
keeps human approval, traceability-only/non-proof rules, and private/sensitive-
content exclusions distinct. No provenance, privacy, approval, diagnostic,
protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define canonical provenance and identity bytes;
semantic/artifact/evidence linkage; automated/human action and approval
semantics; privacy, redaction, retention, deletion, access, and disclosure;
trust/tamper/TCB rules; fail-closed results, diagnostics, and exits; stable
Semantic IDs and original UTF-8 spans; and synthetic offline privacy/approval
fixtures. Seed behavior, Semantic IDs, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

EVD-5804 implementation, provenance storage and linkage, identity registries,
privacy/retention/access policy, human approval, diagnostics, protocols, and
public support remain deferred until Accepted authority and executable offline
evidence exist. No placeholder AI-provenance API is created.
