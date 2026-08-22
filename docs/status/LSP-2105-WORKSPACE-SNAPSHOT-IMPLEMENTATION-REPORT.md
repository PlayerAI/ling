# LSP-2105-WORKSPACE-SNAPSHOT Implementation Report

## Outcome

The compiler now has a deterministic, immutable observation value for the
visible VFS workspace state. It captures source files, non-source workspace
inputs, and the session revision high-water mark while preserving exact bytes
and overlay precedence. This completes only the bounded internal child; the
public `LSP-2105` workspace-reload target remains `BlockedSpec`.

## Implementation

- Added `ling_source::WorkspaceStateSnapshot` with canonical file and input
  collections, revision access, and identity/kind lookups.
- Added `VirtualFileSystem::workspace_snapshot` and the forwarding
  `CompilerDb::workspace_snapshot` accessor.
- Added tests proving deterministic ordering, overlay visibility, exact input
  and source bytes, revisions, and immutability after later VFS mutations.

## Verification

Executed locally:

- `cargo fmt --all -- --check`
- `cargo test -p ling-source -p ling-db --all-targets --locked --offline`
  (25 `ling-source` tests and 21 `ling-db` tests passed)
- `cargo clippy -p ling-source -p ling-db --all-targets --locked --offline -- -D warnings`
- `cargo xtask governance check-all` (111 documents, 28 gaps, 86 lifecycle
  records, 27 protocols, and 89 diagnostic codes)

Implementation commit: `60566b4cd547c1969c0deec512f11f59069e2e7c`.

## Compatibility and determinism

No Ling syntax, semantics, diagnostics, schemas, Semantic IDs, CLI, LSP wire
method, filesystem access, bytecode, runtime, VM, ABI, or Unicode 17.0.0 data
changed. Snapshot values are ordered by canonical logical names and declared
workspace-input order; they contain owned immutable bytes and no host paths or
hash-map iteration effects.

## Intentionally deferred

Watcher/reload notifications, workspace-folder identity, event coalescing,
dependency graph publication, stale-result rejection, cancellation, URI and
document versions, JSON-RPC methods, and public compatibility remain deferred
to the blocked parent and its registered gaps.
