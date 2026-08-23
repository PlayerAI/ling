# LSP-2203 Authority Audit: Pull diagnostics Preview

## Outcome

`LSP-2203` is implementation-authorized by Accepted RFC-0033. That RFC closes
the earlier request/result, result-ID, workspace, snapshot, bound, lifecycle,
and push-parity gaps for a deliberately synchronous LSP 3.17 subset. It does
not authorize dynamic registration, observable cancellation, progress,
partial results, refresh, related-document maps, or Stable compatibility.

## Normative traceability

- RFC-0033 §§1–2 define conditional capability negotiation, method
  availability, exact identifiers, request validation, forward-compatible
  ignored fields, and the 1024-entry workspace bound.
- RFC-0033 §§3–4 require one current immutable RFC-0032 analysis ticket, exact
  adapter-value parity, no push-state mutation, and stateless domain-separated
  BLAKE3 result identities.
- RFC-0033 §§5–7 define document and workspace full/unchanged reports,
  open/closed/removed version behavior, URI ordering, removal clearance,
  response-size failure, and fixed JSON-RPC error classes.
- RFC-0032/RFC-0031 retain authority over compilation, syntax precedence,
  temporary-source isolation, diagnostic fields, order, and source-span
  projection. RFC-0004 and RFC-0023/RFC-0029/RFC-0030 retain lifecycle,
  overlay, revision, and workspace authority.

## Accepted boundary

The implementation may advertise `diagnosticProvider` only when the client
declares object-valued `capabilities.textDocument.diagnostic`, then serve
`textDocument/diagnostic` and `workspace/diagnostic` from current tracked
bytes. Result IDs depend only on the exact URI and ordered diagnostic JSON
array; no cache, document version, host path, time, or process state enters the
identity. Pull must neither consume pending push work nor update its ledger.

The current single-message server cannot observe a later cancellation message
while synchronously handling a request. RFC-0033 therefore makes cancellation,
progress, and partial results non-advertised future work instead of claiming a
non-functional surface.

## Required evidence

- exact provider presence/absence and unnegotiated MethodNotFound behavior;
- full, empty, unchanged, version-independent, and diagnostic-changing
  document results;
- byte-identical push/pull diagnostic arrays and retained pending push work;
- URI-sorted workspace mixtures, open/closed versions, empty workspace, and
  previous-only removal clearance;
- result-ID domain/format/separation evidence, temporary syntax isolation,
  Unicode position projection through the shared adapter, validation and
  previous-result bounds, and oversized-response failure atomicity;
- locked-offline focused and repository-wide quality/governance gates.

## Compatibility impact

This authority adds Preview `ling.lsp.pull-diagnostics/0.1` and
`ling.lsp.pull-result/0.1:blake3:`. It changes neither
`ling.lsp.diagnostic/0.2` nor `ling.lsp.publish-diagnostics/0.1`. No diagnostic
code, severity, message, Facts, repair, Semantic ID, Ling semantics, Typed
Core, runtime, bytecode, VM, ABI, filesystem/network behavior, or Unicode
17.0.0 data changes.

## Intentionally deferred

Dynamic registration, request cancellation, work-done/partial progress,
refresh, related-document maps, notebooks, result persistence, background
workers, root-cause grouping, deduplication, caps, suppression, fixes,
Workspace Edits, Semantic Transactions, and Stable lifecycle remain outside
LSP-2203.
