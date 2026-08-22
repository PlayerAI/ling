# IDE-2302-TYPED-INDEX implementation report

## Outcome

The bounded internal child `IDE-2302-TYPED-INDEX` is implemented. It joins
resolver-owned user definitions with exact facts from a successful
`CheckedProgram` in `ling-db::TypedDefinitionIndex`.

The public `IDE-2302` hover task remains `BlockedSpec`: no hover request,
presentation text, Markdown, editor range, URI/version, or publication path
was added.

## Normative clauses covered

- `DEC-0074` §§Decision 1–5 authorizes the immutable checked observation,
  user-only filtering, resolver spans and IDs, optional exact facts,
  deterministic ordering, and the in-process boundary.
- `DEC-0012` and `DEC-0019` preserve existing identity and query-boundary
  rules; no Semantic ID or cache protocol is introduced.
- `DEC-0060` supplies the existing canonical Effect-row naming used by the
  observation; it is copied without reinterpretation.

No Draft or lower-authority execution-plan text is used as semantic authority.

## Implementation

- `crates/ling-db/src/typed_definition_index.rs` defines immutable
  `TypedDefinitionIndex` and `TypedDefinitionSymbol` values.
- `CompilerDb::typed_definition_index` consumes the existing checked
  workspace query and never publishes a value when source parsing, resolving,
  type checking, or Effect checking fails.
- Records retain resolver definition ID, module/name spelling, classification,
  mutability, logical source name, original UTF-8 `Span`, optional type display,
  optional canonical Effect labels, and optional checked module capabilities.
- Sorting uses logical source name, source ID, original byte start/end,
  classification, source spelling, and definition ID. No host path, map order,
  allocation address, position encoding, or locale participates.

## Verification

Executed successfully:

- `cargo fmt --all`
- `cargo test -p ling-db --lib --locked --offline` (25 passed)
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`

The focused test corpus covers BOM, CRLF, Unicode names, exact original-byte
spans, deterministic repeated observations, canonical `Console.Write` effects,
module capability names, user-only source lookup, and invalid UTF-8 failure.

## Compatibility and determinism

- No language semantics, diagnostics, schemas, Semantic IDs, CLI behavior,
  protocol inventory, runtime, bytecode, VM, ABI, or Unicode 17.0.0 data
  changed.
- The public hover contract, expression selection, display/localization policy,
  Markdown safety, Trait witnesses, URI/version/snapshot binding, cancellation,
  stale publication, and JSON-RPC lifecycle remain deferred.

## Intentionally deferred

`IDE-2302` still requires an Accepted hover authority before any editor-facing
implementation. This child is only a checked compiler observation and must not
be presented as hover support or Stable 1.0 IDE functionality.
