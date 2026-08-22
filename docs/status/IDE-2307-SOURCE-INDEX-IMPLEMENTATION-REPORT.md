# IDE-2307-SOURCE-INDEX implementation report

## Outcome

The bounded internal child `IDE-2307-SOURCE-INDEX` is implemented. It exposes
resolver-backed definitions, local bindings, and resolved import aliases with
their existing identities and exact original UTF-8 spans. The public
`IDE-2307` completion feature remains `BlockedSpec`.

## Normative clauses covered

- Accepted DEC-0079 §§Decision 1–2 authorizes the immutable resolver-backed
  source inventory and its bounded entry categories.
- DEC-0079 §3 preserves normalized names, resolver identities, logical source
  names, and original spans without re-hashing or Semantic ID migration.
- DEC-0079 §4–§5 fixes deterministic ordering and read-only lookup behavior
  while excluding completion policy and protocol state.
- DEC-0002 and DEC-0019 continue to govern original-byte spans and the
  in-process query boundary; DEC-0012 governs identity facts without being
  extended into completion semantics.

## Implementation

- `crates/ling-db/src/completion_source_index.rs` collects user definitions,
  bindings, and resolver-confirmed import aliases from `ResolvedProgram`.
- Definition, binding, and alias entries retain their existing resolver
  identity and source/name span; non-user definitions without source spans and
  unresolved import records are omitted.
- Explicit ordering and source/module/name lookups are independent of host
  paths, allocation order, and map iteration.
- `CompilerDb::resolved_completion_source_index` publishes no value when the
  existing module graph or workspace resolver query fails.

## Verification

```text
cargo fmt --all
cargo test -p ling-db --all-targets --locked --offline
cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings
```

The focused suite passes with 38 tests, including Unicode/import/parameter
source coverage, exact original alias spans, deterministic repeated
construction, and source-scoped lookups.

## Compatibility and determinism

The inventory is an internal compiler observation only. It introduces no
completion schema, candidate ranking, visibility rule, insertion edit, LSP
position, URI/version, snapshot, cancellation, diagnostic, Semantic ID,
runtime, bytecode, VM, ABI, or Unicode-table behavior.

## Deferred work

The public completion contexts, candidate visibility, type/effect/capability
ranking, builtins/Prelude/dependency policy, insertion text, formatter
interaction, request positions and versions, stale/cancellation/resource
behavior, and protocol fixtures remain deferred to the blocked `IDE-2307`
parent and its accepted authorities.
