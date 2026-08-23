# ZED-6803-CURRENT-EVIDENCE Implementation Report

## Result

The thirteen-row Zed acceptance inventory now reflects the passing locked
Windows grammar/query suite and the implemented Preview LSP/position
prerequisites. Every listed public document feature and all Zed packaging,
installation, and marketplace behavior remain unsupported.

The parent `ZED-6803` remains `BlockedSpec`. No extension support is claimed.

## Implementation

- `docs/testing/ZED-EXTENSION-ACCEPTANCE.md` corrects obsolete npm, LSP,
  formatter-prerequisite, position, and lifecycle statements while retaining
  explicit feature and Zed boundaries.
- `tools/xtask/src/zed_extension.rs` composes the current Zed-matrix and
  discovery verifiers, checks ten historical/current evidence files, and adds
  three position-evidence files for negotiation, Unicode, surrogate, BOM/CRLF,
  overlay-method, malformed-metadata, and response markers.
- A focused negative test rejects missing position-evidence markers.
- The result is four covered, five partial, five unsupported, and two future
  classifications across thirteen acceptance areas.

## Executed evidence

`npm run verify --offline` passed on Windows on 2026-08-23 with 41 grammar
cases, scanner/layout integration, 18 Unicode cases, 29 precedence cases, 41
pattern/type cases, 10 recovery cases with 9 incremental edits and 64
mutations, 42 conformance programs with 84 edits and 43 stable mappings, 18
highlight captures, 4 bracket pairs, 15 indentation CST nodes, and example
parsing. Regeneration left no tracked worktree change.

Focused position, LSP, and composed governance checks plus the full repository
gates are the Rust evidence. No Zed process, install, marketplace, Linux/macOS
grammar suite, or cross-host editor integration was executed.

## Compatibility and deferrals

No Ling or editor behavior changes. Zed packaging/API versions, all public LSP
document features and edit protocols, tasks/runnables, Replay/Evidence UI,
restart behavior, workspace limits, installation/marketplace provenance,
cross-host/Zed fixtures, migration, and G6 sign-off remain deferred.
