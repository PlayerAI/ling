# ZED-6801-CURRENT-EVIDENCE Implementation Report

## Result

The Zed compatibility inventory now reflects the implemented Preview LSP
lifecycle/Experimental overlay and a passing locked Windows grammar suite. The
three package JSON files are parsed and validated structurally rather than
accepted solely through text fragments.

The parent `ZED-6801` remains `BlockedSpec`. No Zed extension or compatibility
support is claimed.

## Implementation

- `docs/testing/ZED-COMPATIBILITY-MATRIX.md` corrects obsolete LSP, OS,
  acquisition, limitation, and grammar-snapshot statements.
- `tools/xtask/src/zed_matrix.rs` retains the ten exact surface states and five
  evidence files, then structurally checks three JSON files and fifteen
  accepted metadata values. It also binds the corrected LSP claims to the
  workspace member, crate manifest, and lifecycle/overlay protocol records.
- `tools/xtask/src/main.rs` reports the structured JSON count.
- A focused negative test rejects a false public/private state and a missing
  locked Tree-sitter dependency field.
- Authority and status documents keep the grammar and LSP subset below Zed and
  Stable support boundaries.

## Executed evidence

`npm run verify --offline` passed on Windows with the locked
`tree-sitter-cli@0.26.12`, Node 24.15.0, and npm 12.0.2 after cache access was
authorized. It passed 41 grammar cases, scanner/layout integration, 18 Unicode
cases, 29 precedence cases, 41 pattern/type cases, 10 recovery cases with 9
incremental edits and 64 mutations, 42 conformance programs with 84 edits and
43 stable mappings, 18 highlight captures, 4 bracket pairs, 15 indentation CST
nodes, and example parsing. Regeneration left no tracked worktree change.

This proves only the local Windows grammar package. Linux/macOS and Zed were
not executed.

## Compatibility and deferrals

No Ling or editor behavior changes. Zed extension implementation, document
features, compatibility/version ranges, standalone acquisition, cross-host
tests, release signing/marketplace evidence, migration policy, Stable grammar
nodes, and G6 sign-off remain deferred.
