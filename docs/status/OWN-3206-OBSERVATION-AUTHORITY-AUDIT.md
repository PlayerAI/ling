# OWN-3206-OBSERVATION Authority Audit

## Outcome

The bounded child `OWN-3206-OBSERVATION` is authorized by Accepted
`DEC-0123`. It records only a test-local inventory of proposed ownership
diagnostic and repair boundaries. Public `OWN-3206` remains `BlockedSpec`: no
ownership diagnostic, error-code allocation, repair ranking, JSON field, LSP
code action, or ownership behavior is defined.

## Normative traceability

- The G3 plan is non-normative and cannot authorize error meanings, repair
  ranking, source projections, or LSP fields.
- Accepted DEC-0001/DEC-0002 govern existing diagnostic codes, bilingual
  messages, and UTF-8 spans only.
- `DEC-0122` keeps Drop/cleanup vocabulary test-only.
- `DEC-0123` authorizes this child only; `GAP-OWNERSHIP-MODEL-001` and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain open.

## Current implementation boundary

`ownership_diagnostic_evidence.rs` defines forty-five test-local boundaries,
sorts them by local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is test-only and is not a diagnostic, error
code, Fact, Repair, ranking, edit, code action, public schema, or ownership
contract.

No ownership diagnostic, error-code allocation, repair ranking, JSON field,
Semantic ID, LSP code action, or protocol was added. Accepted Seed diagnostics
remain unchanged.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines ownership diagnostic meanings, repair
schema/ranking, LSP mapping, migration, and interpreter/VM/Native evidence.
