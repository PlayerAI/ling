# OWN-3204-OBSERVATION Authority Audit

## Outcome

The bounded child `OWN-3204-OBSERVATION` is authorized by Accepted
`DEC-0121`. It records only a test-local inventory of proposed
cross-suspension and Actor-turn boundaries. Public `OWN-3204` remains
`BlockedSpec`: no `await` semantics, pinning rule, state-machine lowering,
cross-turn lifetime judgment, Actor reentry rule, message gate, diagnostic, or
ownership behavior is defined.

## Normative traceability

- The G3 plan is non-normative and cannot authorize suspension, pinning, Actor
  reentry, message sendability, or lifetime behavior.
- `DEC-0120` keeps region/lifetime vocabulary test-only.
- `DEC-0119` keeps Borrow vocabulary test-only and `DEC-0009` governs Seed
  mutable places while excluding Borrow and suspension semantics.
- `DEC-0121` authorizes this child only; `GAP-ACTOR-AWAIT-REENTRY-001` and
  `GAP-OWNERSHIP-MODEL-001` remain open.

## Current implementation boundary

`borrow_await_turn_evidence.rs` defines thirty-seven test-local boundaries,
sorts them by local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is test-only and is not an await state,
pinning decision, lifetime result, Actor transition, message schema,
diagnostic, or ownership contract.

No suspension Core, state-machine lowering, cross-turn borrow checker, Actor
reentry rule, message sendability gate, diagnostic, Semantic ID, or protocol
was added. Accepted Seed behavior remains unchanged.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines suspension, pinning, Actor reentry,
message sendability, cancellation/Drop, diagnostics, and
interpreter/VM/Native evidence.
