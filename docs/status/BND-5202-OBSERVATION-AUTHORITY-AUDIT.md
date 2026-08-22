# BND-5202-OBSERVATION Authority Audit — Loop/Recursion Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0182` permits only test-local loop/recursion vocabulary. It does
not authorize a termination calculus, proof checker, runtime guard, work-queue
transformation, diagnostic, code action, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:156-167` is a
  non-normative proposal.
- `docs/status/BND-5202-AUTHORITY-AUDIT.md` records missing RFC-K504 and
  dependent Profile/resource/concurrency/transaction authorities.
- RFC-0015 VM frame/resource limits remain runtime safety only and are not
  source termination proofs.

## Current implementation evidence

The observation adds one isolated test with sixty explicit loop/recursion
boundaries, deterministic local ordering, duplicate rejection, and an opaque
observation tag. No production checker, proof state, runtime guard,
transformation, diagnostic, CLI/LSP action, dependency, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define the termination calculus, proof states,
ranking/evidence provenance, resource/concurrency relations, runtime guard
semantics, transformation equivalence and consent, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline counterexample/equivalence
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode
17.0.0 remain unchanged.

## Deferred work

BND-5202 implementation, termination/proof checker, runtime guards, work-queue
code action, diagnostics, CLI/LSP/editor support, and public protocol claims
remain deferred until RFC-K504 (or an Accepted replacement), dependent
concurrency/resource/profile/transaction authority, and executable evidence
exist.
