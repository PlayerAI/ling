# LSP-2204 implementation report

> Status: Done
> Task: `LSP-2204`
> Authority: Accepted `RFC-0034`, `RFC-0033`, `RFC-0032`, `RFC-0031`,
> `RFC-0004`, `DEC-0001`, `DEC-0019`, `DEC-0034`, `DEC-0071`, and `DEC-0072`

## Scope

This milestone adds one stateless post-adapter diagnostic-control layer shared
by push and pull. It suppresses repeated exact roots, applies immutable
configurable document/workspace caps, and emits registered bilingual omission
summaries without mutating the complete compiler result.

## Normative clauses covered

- RFC-0034 §§1–3: defaults and bounded initialization options, exact discovery,
  accepted adapter order, code/range/Semantic-ID/Facts root identity,
  first-wins retention, and document-then-workspace caps.
- RFC-0034 §4: registered `L-LSP-0001` summaries with exact range, message,
  severity, version, scope, omitted, deduplicated, capped, and maximum values.
- RFC-0034 §§5–7: resource-root preservation, stateless recovery, shared
  push/pull sets, failure atomicity, and push/pull 0.2 migration.

## Implementation

- `diagnostic_control.rs` owns immutable limits, request parsing, root-key
  validation, checked suppression counters, deterministic caps, and summaries.
- `publication.rs` retains the unchanged compiler ticket and adapter output,
  then invokes control before both publication-ledger comparison and pull
  report/result-ID construction.
- Initialization records exact limits once and advertises
  `ling.lsp.diagnostic-control/0.1`; push and pull advertise 0.2 markers.
- `L-LSP-0001` is allocated in the single registry and canonical Rust code set;
  the generated compatibility lock records its immutable identity and Facts.

## Tests and evidence

- Module tests cover first-wins root identity, independent range/Facts roots,
  simultaneous document/workspace caps, summary counts, resource-shaped
  roots, and malformed internal input failure.
- Integration tests cover default/custom discovery, all limit boundaries and
  malformed types, unknown fields, exact summary range/data/severity, URI-
  ordered workspace selection, push/pull byte parity, and recovery clearance.
- Existing adapter tests retain Unicode/CRLF/order evidence; push/pull suites
  retain stale, result-ID, removal, temporary, response ordering, and oversized
  failure-atomicity evidence under configured upper limits.
- Focused LSP, diagnostic registry, and governance checks pass. The complete
  repository gate set passed against implementation commit
  `b70308c1e215fd2f4a4736aa56d7372c368af599` before status binding.

## Compatibility, determinism, and Unicode impact

- Adds Preview `ling.lsp.diagnostic-control/0.1` and `L-LSP-0001`; advances
  publish and pull protocols to 0.2. Adapter 0.2 and pull-result 0.1 formats do
  not change.
- Existing compiler diagnostic values are retained byte-for-byte when selected.
  No code other than the new omission code changes meaning or shape.
- No Ling syntax/semantics, Typed Core, parser recovery, production Trait
  solver, runtime, bytecode, VM, ABI, Semantic ID, filesystem/network/cache,
  or Unicode 17.0.0 behavior changes.

## Intentionally deferred

Compiler/parser recovery improvements and production Trait solver integration
remain compiler tasks. Dynamic settings, severity/code filtering, merge rules,
telemetry, persistence, cancellation, progress, fixes, Workspace Edits,
Semantic Transactions, and Stable lifecycle remain future work.
