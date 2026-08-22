# TRAIT-1309-TERMINATION Authority Audit

## Outcome

`TRAIT-1309-TERMINATION` is a bounded child authorized by Accepted RFC-0005,
DEC-0026, and DEC-0068. It records deterministic source-evidence independence
and preserves the accepted cycle/depth boundary. The `TRAIT-1309` parent remains
`BlockedSpec` for production query integration, benchmark metrics, resource
budgets, and LSP cancellation.

## Evidence boundary

The child adds one crate-private solver test. It runs the existing concrete
Trait fixture under two logical source names and compares only selected
obligation order, Trait name, implementation ordinal, receiver, and member
names. Existing cycle and depth-64 tests remain the negative termination
evidence. Source names and spans are not used for selection.

No wall-clock measurement, allocation budget, benchmark schema, public
diagnostic, cancellation API, CLI/LSP service, Semantic ID, or protocol is
added. The solver remains an internal checked-data boundary.

## Intentionally deferred

The parent still needs an accepted production HIR/Typed Core obligation graph,
deep-chain/diamond/failure/cross-package corpus, deterministic work budget,
variance/environment policy, and LSP cancellation/stale-result authority.
