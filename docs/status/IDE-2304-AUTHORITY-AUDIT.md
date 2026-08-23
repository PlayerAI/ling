# IDE-2304 Authority Audit: References

## Outcome

`IDE-2304` is authorized by Accepted `RFC-0039`. Its dependencies `IDE-2303`,
`LSP-2101`, and `LSP-2102` are `Done`; Accepted `DEC-0075`, `DEC-0076`, and
`DEC-0078` provide the bounded forward, reverse, and exact span observations.
RFC-0039 composes those internal facts with the accepted snapshot, URI,
position, checked-program, ordering, bound, error, and migration contracts.

The implementation boundary is a request-scoped checked index and standard
`Location[]`. It does not invent a persistent cache, expose internal identity,
or infer type/implementation occurrences from names.

## Normative traceability

- RFC-0039 fixes `ling.lsp.references/0.1`, capability/request/result shapes,
  declaration inclusion, snapshot/freshness, ordering, bounds, errors, and
  migration behavior.
- RFC-0038 fixes the shared exact resolver-target and URI/range navigation
  boundary reused by references.
- DEC-0002 and DEC-0029 preserve original UTF-8 spans and negotiated
  UTF-8/16/32 projection.
- DEC-0012 keeps resolver and Semantic identities internal; no identity field
  enters the response.
- DEC-0019 and DEC-0071 authorize request-scoped immutable compiler snapshots,
  but no persistent cache or cross-request complexity claim.
- DEC-0075, DEC-0076, and DEC-0078 authorize exact resolver target grouping
  and reference-span observations.

## Relation boundary

RFC-0039 fixes the complete vocabulary as `read`, `write`, `call`, `type`, and
`implementation`. Version 0.1 emits only resolver-owned expression
occurrences: assignment roots are `write`, application function positions are
`call`, and other resolved expression references are `read`. The exact
discovery object names both the complete vocabulary and emitted subset.

Type and implementation surfaces currently lack resolver-owned occurrence
identities. They remain explicit non-emission rather than being guessed from
spelling, displayed types, coherence candidates, or Experimental Semantic
Graph data. Emitting either kind is an incompatible marker migration.

## Current implementation evidence

- `ling-db::ReferenceSearchIndex` consumes a complete checked program, joins
  exact resolver targets/spans/declarations atomically, classifies expression
  relations, enforces bounds, and supports declaration or reference selection.
- `ling-lsp::references` validates the standard request, isolates temporary
  sources, maps logical source names to unique captured URIs, projects exact
  ranges, rejects stale completion, and returns only standard `Location[]`.
- Focused database, LSP integration, navigation regression, exact diagnostic
  transcript, and strict Clippy checks pass.

## Compatibility and deferred work

No Ling syntax/semantics, Typed Core evaluation, interpreter, runtime,
bytecode, VM, ABI, diagnostic allocation, Semantic ID/schema, persistent cache,
package format, source truth, or Unicode 17.0.0 behavior changes.

Resolver-owned type/implementation occurrence identities, generated/virtual
documents, custom relation results, persistent/incremental caches, progress,
partial results, asynchronous cancellation, Workspace Edits, Semantic
Transactions, and Stable compatibility remain deferred.
