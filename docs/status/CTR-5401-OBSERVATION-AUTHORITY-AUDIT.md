# CTR-5401-OBSERVATION Authority Audit — Contract Syntax/Core Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0192` permits only test-local Contract syntax/Core vocabulary.
It does not authorize a Contract parser, AST/HIR/Core form, resolver,
effect restriction, proof/status schema, runtime assertion, diagnostics, or
support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:304-316` is a
  non-normative Contract proposal.
- `docs/status/CTR-5401-AUTHORITY-AUDIT.md` records the missing grammar,
  Core, proof/runtime, status, diagnostic, and evidence authority.
- `GAP-CRITICAL-PROFILE-001` and `GAP-CONTRACT-PROOF-001` remain open;
  RFC-K503 is absent.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Contract claim,
expression, purity/effect, identity, status/proof, runtime, diagnostic, and
fixture boundaries. It sorts by explicit local rank, rejects duplicates,
compares canonical opaque bytes for forward/reverse input order, and uses an
observation-only tag. No parser, Core form, checker, proof adapter,
diagnostic, CLI/LSP action, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define versioned Contract grammar and AST/HIR/Core
mapping, expression/effect/purity rules, identity/canonical bytes, one status
lifecycle, proof/runtime-check boundaries, Fault/isolation, profiles and
evidence, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

CTR-5401 implementation, Contract parser/AST/Core, resolver/effect rules,
proof/status schema, runtime checks, diagnostics, CLI/LSP/protocols, and
support claims remain deferred until RFC-K503 (or an Accepted replacement),
the related gaps, and executable offline evidence exist. No placeholder
Contract API is created.
