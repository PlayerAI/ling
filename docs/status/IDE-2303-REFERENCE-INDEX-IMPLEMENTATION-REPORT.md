# IDE-2303-REFERENCE-INDEX implementation report

## Outcome

The bounded internal child `IDE-2303-REFERENCE-INDEX` is implemented. It
materializes resolver reference keys and their existing definition or binding
targets in `ling-db::ResolvedReferenceIndex`.

The public `IDE-2303` definition-navigation task remains `BlockedSpec`: no
request-position lookup, URI, document version, editor location, dependency
policy, or publication path was added.

## Normative clauses covered

- `DEC-0075` §§Decision 1–5 authorizes the immutable reference/target
  observation, existing resolver identities and spans, deterministic ordering,
  and the in-process boundary.
- `DEC-0002` keeps original UTF-8 `SourceId + Span` facts authoritative; no
  editor position or source-map conversion is performed.
- `DEC-0012` and `DEC-0019` preserve existing identity and query-boundary
  rules; no Semantic ID or cache protocol is introduced.

No Draft or lower-authority execution-plan text is used as semantic authority.

## Implementation

- `crates/ling-db/src/reference_index.rs` defines immutable target and entry
  values for definition and local-binding references.
- `CompilerDb::resolved_reference_index` consumes the existing validated
  resolver workspace and omits malformed entries instead of fabricating target
  metadata.
- Definition targets retain `DefinitionId` plus optional resolver name,
  logical source name, and original span. Binding targets retain module/local
  IDs, name, logical source name, and original span.
- Entries sort by logical source, module identity, reference ID, target kind,
  and existing target identity/span facts. No host path, map order, allocation
  address, position encoding, or locale participates.

## Verification

Executed successfully:

- `cargo fmt --all`
- `cargo test -p ling-db --lib --locked --offline` (27 passed)
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`

Focused tests cover Unicode, BOM, CRLF, exact target spans, repeated
observations, source-local lookup, and invalid UTF-8 failure. A user reference
to `helper` is checked against the exact original bytes of its definition.

## Compatibility and determinism

- No language semantics, diagnostics, schemas, Semantic IDs, CLI behavior,
  protocol inventory, runtime, bytecode, VM, ABI, or Unicode 17.0.0 data
  changed.
- Request positions, declaration/type-definition distinctions, aliases,
  builtins/Prelude/primitives, generated/dependency documents, URI/version/
  snapshot binding, cancellation, stale publication, and JSON-RPC lifecycle
  remain deferred.

## Intentionally deferred

`IDE-2303` still requires Accepted navigation/source-origin authority before
any editor-facing implementation. This child is only a checked resolver
observation and must not be presented as go-to-definition support or Stable
1.0 IDE functionality.
