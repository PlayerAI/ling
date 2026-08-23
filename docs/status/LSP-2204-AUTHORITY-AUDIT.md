# LSP-2204 Authority Audit: Root-cause and error-storm control

## Outcome

`LSP-2204` is implementation-authorized by Accepted RFC-0034. It defines the
previously missing root identity, configurable bounds, explicit omission
representation, push/pull interaction, recovery, failure, and migration rules
for a stateless LSP projection. Compiler diagnostics remain complete and
unchanged before that projection.

## Normative traceability

- `SEMANTICS.md` §26 requires root-cause-first diagnostics and suppression of
  obvious cascades while retaining multiple independent errors.
- RFC-0031 and DEC-0034 define exact adapter fields and the accepted logical
  path/original-byte/code/end/tie order.
- RFC-0032 and RFC-0033 define immutable current analysis, push replacement,
  pull full/unchanged reports, result identity, versions, and clearance.
- RFC-0034 §§1–7 add exact-root first-wins suppression, immutable initialization
  limits, document/workspace caps, `L-LSP-0001` summaries, shared push/pull
  control, recovery, and the 0.2 migration.

## Accepted boundary

Root identity is exact URI plus code, projected range, Semantic ID, and Facts.
The first accepted-order value wins; independent ranges, identities, or Facts
remain visible. Document caps run before the URI-ordered workspace cap.
Summaries are additional registered warnings and do not consume either cap.

The LSP layer does not alter parser recovery or manufacture compiler failures.
An upstream registered resource-limit diagnostic remains a root and exact
duplicates collapse to one. The crate-private Trait solver is not production-
integrated, so no Trait diagnostic or public support claim is created here.

## Required evidence

- default/custom discovery and complete limit validation;
- root identity, first-wins values, independent errors, resource roots, accepted
  order, per-document/workspace caps, and exact summary counts/ranges;
- push/pull parity, protocol markers, result IDs, pending/ledger isolation,
  recovery, clearance, stale results, and oversized failure atomicity;
- temporary sources, Unicode encodings, CRLF, repeatability, URI-order
  invariance, malformed internal shapes, and generated governance locks;
- full locked-offline repository quality and governance gates.

## Compatibility impact

Adds Preview `ling.lsp.diagnostic-control/0.1` and registered warning
`L-LSP-0001`; advances push/pull diagnostic-set markers to 0.2. The compiler
adapter and pull-result algorithm markers are unchanged. No existing diagnostic
meaning, Semantic ID, language/runtime/VM/ABI, or Unicode 17.0.0 behavior
changes.

## Intentionally deferred

Parser recovery changes, production Trait solver integration, dynamic settings,
severity/code filtering, merge policies, telemetry, persistence, cancellation,
progress, fixes, Workspace Edits, Semantic Transactions, and Stable lifecycle
remain outside LSP-2204.
