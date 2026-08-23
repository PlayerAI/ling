# DOC-6701-EVIDENCE-PATHS Implementation Report

## Result

`cargo xtask docs verify` now validates the evidence column of every formal-set
row. The current twelve manuals cite forty-six exact existing repository files
or directories; four rows remain explicitly Future/Unsupported.

The parent `DOC-6701` remains `BlockedSpec`. Path existence is not a claim that
a manual is complete, authoritative, implemented, or release-ready.

## Implementation

- `tools/xtask/src/documentation_matrix.rs` extracts recognized backticked
  repository paths, requires at least one per manual, rejects non-exact or
  unsafe spellings, and checks existence.
- The exact evidence roots are README, docs, tests, crates, tools, and editors.
- A negative test proves wildcard and missing evidence paths fail closed.
- `docs/testing/DOCUMENTATION-INVENTORY.md` replaces abbreviated decision names,
  status globs/ranges, and generic planning references with current exact paths.
- `tools/xtask/src/main.rs` reports the verified evidence-path count.

## Acceptance evidence

- Twelve manual rows retain their accepted states.
- Forty-six exact evidence paths exist in the current repository.
- Every row has at least one exact citation; wildcard, traversal, backslash,
  colon, empty-component, and missing paths are rejected.
- Four future capability groups remain Future/Unsupported; path evidence does
  not promote them.
- Focused and full offline repository gates are required before completion is
  recorded.

## Compatibility and deferrals

No Ling behavior or public contract changes. Content completeness, anchors,
external links, generated references, bilingual parity, stable manuals for
future features, publication, and G6 documentation sign-off remain deferred.
