# PRJ-1107-LOAD implementation report

## Outcome

`ling-project` now provides a read-only `LockedProject` snapshot boundary. It
owns the validated manifest, package graph, and canonical lock projection after
RFC-0002 locked validation. The parent `PRJ-1107` remains `BlockedSpec`: this
slice does not implement semantic compilation, workspace selection,
run/test/build, artifact policy, or a new CLI/protocol surface.

## Normative traceability

- Accepted `DEC-0058` §§1–4 authorizes `LockedProject` and
  `load_locked_project` with explicit-root, `LockMode::Locked`, no-partial-
  snapshot, and no-mutation boundaries.
- Accepted RFC-0002 continues to govern manifest, module, package-graph, and
  lock identity/validation behavior.
- Accepted RFC-0024 continues to govern the existing explicit locked project
  check Preview command; this child does not widen that command.

## Implementation

- Added `crates/ling-project/src/workspace.rs` with `LockedProject` and
  `load_locked_project`.
- Re-exported the library boundary from `crates/ling-project/src/lib.rs`.
- Added deterministic fixture tests for repeated equality, graph identity,
  canonical lock bytes, and lock non-mutation.

## Evidence

Executed locally against the implementation:

- `cargo fmt --all -- --check`;
- `cargo test -p ling-project --all-features --locked --offline`;
- `cargo clippy -p ling-project --all-targets --all-features --locked --offline -- -D warnings`;
- `git diff --check`.

## Compatibility, determinism, and Unicode

No language syntax, Checked Core, runtime, bytecode, diagnostics, schemas,
Semantic IDs, CLI behavior, protocol inventory, dependency, or Unicode
17.0.0 data changed. The snapshot stores no path or host state; graph and lock
ordering come from the existing deterministic RFC-0002 library boundary.

## Intentionally deferred

Compiler-host source loading, workspace/member selection, semantic checking,
incremental revisions, project run/test/build, artifact generation, registry
and network access, package publication, and Stable project CLI behavior remain
deferred to the blocked PRJ-1107 parent.

