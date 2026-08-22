# IDE-2305-IDENTIFIER-OBSERVATION implementation report

## Outcome

The bounded internal child `IDE-2305-IDENTIFIER-OBSERVATION` is implemented.
It records Unicode facts for a raw possible rename candidate through
`ling-db::observe_rename_identifier`.

The public `IDE-2305` prepare-rename task remains `BlockedSpec`: no target,
range, keyword policy, placeholder, diagnostic, snapshot/version check, edit,
or protocol response was added.

## Normative clauses covered

- `DEC-0077` §§Decision 1–5 authorizes the read-only observation, delegation to
  the existing Unicode implementation, NFC/raw-spelling retention,
  deterministic security facts, and the non-protocol boundary.
- `docs/SEMANTICS.md` §§3.3–3.7 and `docs/LANGUAGE.md` §§5.4–5.6 keep XID,
  NFC, forbidden-character, Script, and confusable behavior authoritative.
- `DEC-0002` remains the source-span authority; this child intentionally does
  not fabricate a source span or editor range.
- `DEC-0019` remains the in-process query boundary; no cache, persistence, or
  invalidation contract is introduced.

No Draft or lower-authority execution-plan text is used as semantic authority.

## Implementation

- `crates/ling-db/src/rename_identifier.rs` defines the owned observation,
  status projection, and pure helper.
- The helper delegates to `ling_unicode::inspect_identifier`; invalid input
  returns the existing `IdentifierError` without a new diagnostic allocation.
- Original spelling, NFC name, UTS #39 skeleton, sorted Script and
  Identifier_Type labels, Identifier_Status, and mixed-script facts are copied
  into an immutable value.
- `ling-db` depends directly on `ling-unicode` so the observation does not
  duplicate Unicode tables or keyword logic.

## Verification

Executed successfully:

- `cargo fmt --all`
- `cargo check -p ling-db --all-targets --offline`
- `cargo test -p ling-db --all-targets --offline` (33 passed)

Focused tests cover decomposed/NFC names, confusable mixed-script facts,
invalid XID input, repeated deterministic observations, and exact raw spelling
retention.

## Compatibility and determinism

- No language semantics, diagnostics, schemas, Semantic IDs, CLI behavior,
  protocol inventory, runtime, bytecode, VM, ABI, or Unicode 17.0.0 tables
  changed.
- The observation is not a rename validator and does not classify keywords,
  select targets, inspect aliases/collisions, evaluate visibility/coherence, or
  bind versions, positions, snapshots, edits, cancellation, or stale results.

## Intentionally deferred

`IDE-2305` still requires Accepted prepare-rename and transaction authority
before editor-facing work. `IDE-2305-IDENTIFIER-OBSERVATION` must not be
presented as name acceptance, prepare rename, rename support, or Stable 1.0 IDE
functionality.

