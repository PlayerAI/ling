# IDE-2311-SOURCE-LOOKUPS implementation report

## Outcome

The bounded internal child `IDE-2311-SOURCE-LOOKUPS` is implemented. It adds
exact module-name and definition-name lookups over the existing deterministic
resolver definition inventory. The public `IDE-2311` workspace-symbol feature
remains `BlockedSpec`.

## Normative clauses covered

- Accepted DEC-0082 §§Decision 1–3 authorizes read-only exact lookups over
  existing resolver identities, module/name facts, and original spans.
- DEC-0002 preserves original UTF-8 source spans; DEC-0012 preserves existing
  Semantic/definition identity boundaries; DEC-0019 and DEC-0073 govern the
  internal query and resolved-definition inventory boundaries.
- DEC-0082 §4 excludes search, ranking, workspace scope, limits, positions,
  cancellation, and protocol semantics from this child.

## Implementation

- `crates/ling-db/src/definition_index.rs` adds `module_symbols` and
  `name_symbols` exact lookups returning references to existing immutable
  `ResolvedDefinitionSymbol` entries.
- Lookup results retain the index's deterministic source/span/kind/name/
  identity order and return empty for missing keys; no identity or symbol
  presentation is synthesized.
- `crates/ling-db/src/lib.rs` tests cover module/name hits and missing-key
  emptiness alongside existing Unicode/BOM/CRLF span evidence.

## Verification

```text
cargo fmt --all
cargo test -p ling-db --all-targets --locked --offline
cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings
```

The focused suite covers exact lookup, missing-key behavior, deterministic
source ordering, resolver identity, and original UTF-8 span retention.

## Compatibility and determinism

This is an internal source lookup only. It introduces no workspace-symbol
request/response, prefix or fuzzy matching, taxonomy, package/dependency/
generated policy, result limit or truncation, cancellation, stale revision,
position/URI/version projection, protocol field, diagnostic, Semantic ID,
runtime, bytecode, VM, ABI, or Unicode-table change.

## Deferred work

Workspace scope and package selection, symbol taxonomy/containers, search and
ranking, incremental invalidation, resource limits, truncation, cancellation,
positions, versions, stale behavior, persistence, and protocol fixtures remain
deferred to the blocked `IDE-2311` parent and its accepted authorities.
