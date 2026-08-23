# IDE-2306 Authority Audit: Rename

## Outcome

`IDE-2306` is implementation-ready and implemented under Accepted `RFC-0041`.
Its dependencies `IDE-2305`, `LSP-2102`, and `LSP-2104` are Done. RFC-0041
closes the earlier missing identity, new-name, snapshot simulation, versioned
Workspace Edit, and lifecycle decisions for one bounded Preview protocol.

## Normative traceability

- RFC-0041 fixes transactional capability negotiation, immutable checked
  selection, Unicode 17.0.0 new-name policy, complete occurrence ownership,
  simulation, identity/topology checks, deterministic versioned edits, nulls,
  bounds, freshness, failures, and migration.
- RFC-0039 and RFC-0040 supply checked reference grouping and exact target/span
  selection. DEC-0077 and DEC-0078 supply identifier and original reference
  span observations without textual inference.
- DEC-0002 and DEC-0029 keep original UTF-8 `SourceId + Span` authoritative and
  project only through the negotiated LSP encoding. DEC-0012 requires a renamed
  definition to receive a different DefinitionId rather than pretending its
  name-derived identity is stable.
- RFC-0005 continues to govern checked Trait/coherence behavior. Rename accepts
  only a simulated workspace that completes ordinary resolution, type, and
  Effect checks while retaining exact occurrence relations.

## Gap boundaries

RFC-0041 removes IDE-2306 from the open transaction, language-alias, and
localization gap blocker lists without resolving those gaps generally. The
server returns a standard proposed Workspace Edit and performs no mutation;
Semantic Transactions, language Alias declarations, localized source views,
generated/dependency mutation, module/file rename, and Stable lifecycle remain
later work.

## Current implementation evidence

- `ling-db::CheckedRenameAliasIndex` derives explicit import-alias declaration
  and qualified-use occurrences structurally from checked HIR/resolver data.
- `ling-lsp::rename` requires negotiated transactional `documentChanges`, one
  complete fresh checked snapshot, exact writable ownership, Unicode/name
  legality, non-overlapping copied-byte edits, and a fresh simulation.
- Definition migration, binding/import-target stability, exact translated
  occurrence topology, relation preservation, document versions, URI/edit
  ordering, response bounds, and stale completion are checked before output.
- Executable tests cover capability and lifecycle behavior, definitions,
  references, locals, aliases, multiple documents, dependencies, builtins,
  temporary sources, Unicode/security and collisions, coherence rejection,
  UTF-8/16/32, BOM/CRLF, versions, notifications, and checked failure.

## Evidence and compatibility

The implementation adds Preview `ling.lsp.rename/0.1` and one internal checked
alias index. It adds no diagnostic allocation, Ling language behavior, Typed
Core evaluation, Semantic ID schema, runtime, bytecode, VM, ABI, package,
filesystem/network mutation, or Unicode 17.0.0 table change.

## Intentionally deferred

General Semantic Transactions, language Alias syntax, localized Author Source,
generated/virtual or dependency mutation, module/file rename, type-only
identities, cancellation, progress, annotations, and Stable compatibility are
intentionally deferred.
