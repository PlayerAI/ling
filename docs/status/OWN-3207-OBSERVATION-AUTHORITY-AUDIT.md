# OWN-3207-OBSERVATION Authority Audit

## Outcome

The bounded child `OWN-3207-OBSERVATION` is authorized by Accepted
`DEC-0124`. It records only a test-local inventory of proposed ownership
negative-corpus and property-test boundaries. Public `OWN-3207` remains
`BlockedSpec`: no legal/illegal oracle, generator, shrinking algorithm, fuzz
target, expected diagnostic, property invariant, or ownership behavior is
defined.

## Normative traceability

- The G3 plan is non-normative and cannot authorize ownership outcomes,
  generators, diagnostics, or fuzz contracts.
- `DEC-0123` keeps ownership-diagnostic vocabulary test-only and `DEC-0122`
  keeps Drop/cleanup vocabulary test-only.
- `DEC-0009` governs Seed mutable places and excludes future ownership
  judgments.
- `DEC-0124` authorizes this child only; `GAP-OWNERSHIP-MODEL-001` and
  `GAP-ACTOR-AWAIT-REENTRY-001` remain open.

## Current implementation boundary

`ownership_corpus_evidence.rs` defines thirty-six test-local boundaries, sorts
them by local rank, rejects duplicates, and compares forward/reverse insertion
order. Its evidence tag is test-only and is not an oracle, generated case,
legal/illegal result, expected diagnostic, fuzz target, property invariant, or
ownership contract.

No ownership corpus, generator, shrinking algorithm, fuzz target, expected
diagnostic, error code, public protocol, or property semantics was added.
Accepted Seed tests remain unchanged.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines legal/illegal outcomes, generators,
shrinking, interleavings, diagnostics, migration, and interpreter/VM/Native
evidence.
