# IDE-2308-METADATA implementation report

## Outcome

The bounded internal child `IDE-2308-METADATA` is implemented. It joins
resolver-backed user definitions and local/parameter bindings with existing
checked type, Effect Row, and module Capability observations. The public
`IDE-2308` completion-resolve feature remains `BlockedSpec`.

## Normative clauses covered

- Accepted DEC-0080 §§Decision 1–2 authorizes the immutable checked metadata
  boundary and the bounded definition/binding source set.
- DEC-0080 §3 preserves existing resolver identities, source spans, type
  displays, canonical effects, and optional capabilities without placeholders.
- DEC-0080 §4–§5 fixes deterministic ordering and read-only lookup behavior
  while excluding documentation, presentation, insertion, and protocol state.
- DEC-0002, DEC-0010, DEC-0012, and DEC-0019 remain the governing source-span,
  effect/capability, identity, and query-boundary authorities.

## Implementation

- `crates/ling-db/src/completion_metadata_index.rs` builds immutable entries
  from `CheckedProgram` definitions and bindings.
- Optional type, effect, and capability fields preserve absent checked facts;
  existing canonical names and display functions supply the observations.
- `CompilerDb::resolved_completion_metadata_index` requires the existing
  checked workspace query and publishes no value after source or checking
  failure.

## Verification

```text
cargo fmt --all
cargo test -p ling-db --all-targets --locked --offline
cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings
```

The focused suite passes with 42 tests, including checked definition and
binding metadata, optional-fact preservation, identity/source lookup,
deterministic repeated construction, and invalid-source non-publication.

## Compatibility and determinism

This is an internal compiler observation only. It introduces no completion
handle, documentation or signature renderer, capability disclosure/redaction,
insertion edit, formatter behavior, LSP position, URI/version, snapshot,
cancellation, diagnostic, Semantic ID, runtime, bytecode, VM, ABI, or
Unicode-table behavior.

## Deferred work

Completion-item identity/lifetime, documentation and localization, full
signature presentation, Effect/Capability rendering policy, redaction,
insertion text and formatter interaction, request positions and versions,
stale/cancellation/resource behavior, and protocol fixtures remain deferred to
the blocked `IDE-2308` parent and its accepted authorities.
