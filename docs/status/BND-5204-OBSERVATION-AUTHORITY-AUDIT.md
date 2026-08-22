# BND-5204-OBSERVATION Authority Audit — Resource-Budget Diagnostic Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0184` permits only test-local resource-budget diagnostic
vocabulary. It does not authorize budget facts, new diagnostic codes, schema
fields, proof/provenance meanings, repairs, CLI/LSP actions, transactions, or
support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:184-195` is a
  non-normative diagnostic proposal.
- `docs/ROADMAP-1.0.md:243-249` requires reuse of the compiler model and
  versioned edit/transaction protocols for code actions.
- `docs/status/BND-5204-AUTHORITY-AUDIT.md` records missing BND-5203 facts,
  RFC-K504, code allocation, and CLI/LSP/transaction authority.
- DEC-0001/DEC-0002 and Preview `ling.diagnostic/0.1` remain the only accepted
  diagnostic compatibility boundaries; VM Runtime Faults are host-safety
  evidence, not source budget facts.

## Current implementation evidence

The observation adds one isolated test with sixty explicit diagnostic fact,
proof/provenance, schema/migration, repair/transaction, and fixture
boundaries, deterministic local ordering, duplicate rejection, and an opaque
observation tag. No production fact producer, diagnostic code, schema,
repair, CLI/LSP action, transaction, runtime, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define budget facts and field types, proof/estimate/
unknown states, provenance and ordering, stable bilingual
`L-<DOMAIN>-<NUMBER>` allocations, schema migration, and the distinction
between Repairs and semantics-changing transactions. It must also define
snapshot/version, stale-result, cancellation, consent, rollback, and
source-map/equivalence rules with offline fixtures. Seed behavior, existing
diagnostic compatibility, Semantic IDs, UTF-8 spans, dependencies, and Unicode
17.0.0 remain unchanged.

## Deferred work

BND-5204 implementation, resource-budget facts and diagnostics, new error-code
allocations, CLI/LSP routes, Repair/transaction schemas, and public support
remain deferred until BND-5203, RFC-K504 (or an Accepted replacement), the
dependent Critical/ownership/Native/Device authorities, and executable offline
evidence exist.
