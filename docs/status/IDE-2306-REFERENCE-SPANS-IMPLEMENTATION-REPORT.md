# IDE-2306-REFERENCE-SPANS implementation report

## Outcome

The bounded internal child `IDE-2306-REFERENCE-SPANS` is implemented. It
builds `ling-db::ResolvedReferenceSpanIndex`, pairing resolver reference IDs
with exact original UTF-8 identifier spans collected from HIR.

The public `IDE-2306` rename task remains `BlockedSpec`: no target policy,
Workspace Edit, temporary snapshot, mutation, identity migration, or protocol
response was added.

## Normative clauses covered

- `DEC-0078` §§Decision 1–5 authorizes the resolver-filtered HIR span
  projection, exact Name/projection/place-root span selection, deterministic
  ordering, and the in-process boundary.
- `DEC-0002` preserves original `SourceId + Span` as the only source-position
  authority; no LSP position is fabricated.
- `DEC-0019` remains the internal query boundary; no persistence, cache, or
  invalidation protocol is introduced.
- `DEC-0012` identity values are observed only; no DefinitionId is changed or
  migrated.

No Draft or lower-authority execution-plan text is used as semantic authority.

## Implementation

- `crates/ling-db/src/reference_span_index.rs` recursively walks resolved HIR
  expressions and records Name, projection-field, and mutable-place-root spans.
- Entries are filtered against the resolver's `ReferenceKey` map, sorted by
  logical source/module/reference/span order, and exposed through immutable
  source and identity lookups.
- `CompilerDb::resolved_reference_span_index` derives the observation from the
  validated workspace resolver result and publishes no partial value after a
  resolution failure.

## Verification

Executed successfully:

- `cargo fmt --all`
- `cargo test -p ling-db --all-targets --offline` (36 passed)

Focused tests cover exact Unicode/BOM/CRLF name spans, deterministic repeated
construction, source-scoped lookup, and omission of HIR reference IDs absent
from resolver identity.

## Compatibility and determinism

- No language semantics, diagnostics, schemas, Semantic IDs, CLI behavior,
  protocol inventory, runtime, bytecode, VM, ABI, or Unicode 17.0.0 tables
  changed.
- The observation has no URI/version, position encoding, edit, snapshot,
  cancellation, stale-result, persistence, cache, relation taxonomy, or target
  eligibility semantics.

## Intentionally deferred

`IDE-2306` still requires Accepted rename identity, alias/reference collection,
temporary-snapshot, transaction, and protocol decisions before editor-facing
work. This child is only source-span evidence and must not be presented as
rename support or Stable 1.0 IDE functionality.
