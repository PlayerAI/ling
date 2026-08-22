# OWN-3203-OBSERVATION Authority Audit

## Outcome

The bounded child `OWN-3203-OBSERVATION` is authorized by Accepted
`DEC-0120`. It records only a test-local inventory of proposed region and
lifetime boundaries. Public `OWN-3203` remains `BlockedSpec`: no region
variable, lifetime rule, inference relation, escape judgment, public API
projection, diagnostic, or ownership behavior is defined.

## Normative traceability

- The G3 plan is non-normative and cannot authorize lifetime syntax, public
  ABI, escape rules, or inference.
- `DEC-0119` keeps Borrow vocabulary test-only.
- `DEC-0009` governs Seed mutable places and excludes Borrow and lifetime
  semantics.
- `DEC-0120` authorizes this child only; `GAP-OWNERSHIP-MODEL-001` and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain open.

## Current implementation boundary

`region_inference_evidence.rs` defines thirty-nine test-local boundaries,
sorts them by local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is test-only and is not a region variable,
lifetime, outlives constraint, escape result, public signature, diagnostic, or
ownership contract.

No region/lifetime Core node, inference solver, outlives graph, escape checker,
public lifetime projection, diagnostic, Semantic ID, or protocol was added.
Accepted Seed Place behavior remains unchanged.

## Evidence and deferred work

Focused tests cover the complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines region inference, public lifetime
projection, escape/suspension behavior, diagnostics, and interpreter/VM/Native
evidence.
