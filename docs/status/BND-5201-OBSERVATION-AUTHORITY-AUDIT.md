# BND-5201-OBSERVATION Authority Audit — Bound Types/Expressions Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0181` permits only test-local Bound vocabulary. It does not
authorize Bound syntax, types, a constraint solver, proof/resource semantics,
diagnostics, CLI options, or support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:138-155` is a
  non-normative Bound proposal.
- `docs/ROADMAP-1.0.md:118` does not authorize a Bound language feature.
- `docs/status/BND-5201-AUTHORITY-AUDIT.md` records missing RFC-K504 and
  dependent Critical/resource authorities.
- Existing implementation guards remain internal safety limits and are not
  source-level bound behavior.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Bound/type/
expression boundaries, deterministic local ordering, duplicate rejection, and
an opaque observation tag. No production AST/HIR/Typed-Core node, solver,
profile parameter, diagnostic, CLI/LSP option, runtime, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define Bound grammar and checked representation,
units/domains/arithmetic, unknown and symbolic values, resource soundness,
proof/runtime states, profile/target limits, ownership/effect/concurrency/
Device/Fault relations, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
offline fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

BND-5201 implementation, Bound syntax/types, expressions and solver,
termination/resource/effect integration, diagnostics, CLI/LSP/editor support,
and public protocol claims remain deferred until RFC-K504 (or an Accepted
replacement), the dependent Critical/resource authorities, and executable
offline evidence exist.
