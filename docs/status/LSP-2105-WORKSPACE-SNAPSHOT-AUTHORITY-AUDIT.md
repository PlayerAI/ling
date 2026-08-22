# LSP-2105-WORKSPACE-SNAPSHOT Authority Audit

## Outcome

`LSP-2105-WORKSPACE-SNAPSHOT` is a bounded child of the blocked `LSP-2105`
target, authorized by Accepted `DEC-0071` and the internal revision boundary
in `DEC-0019`. It captures compiler-owned workspace state without accepting or
implementing a public workspace reload, watcher, or LSP notification contract.

## Normative traceability

- Accepted `DEC-0019` authorizes immutable in-process query inputs, session
  revisions, deterministic invalidation identity, and clean/incremental
  equivalence without a public cache or protocol.
- Accepted `RFC-0004` defines the existing LSP lifecycle boundary but does not
  define watcher ownership, project reload events, or stale-result policy.
- `GAP-INCREMENTAL-CACHE-001`, `GAP-PROJECT-CLI-INTERFACE-001`, and
  `GAP-LSP-TRANSACTION-PROTOCOL-001` keep public reload identity, event
  sources, publication, and version semantics open.
- Accepted `DEC-0071` authorizes only the immutable source/input/revision
  capture and explicitly preserves the blocked parent.

## Current interface evidence

`ling-source::VirtualFileSystem` already owns visible disk/overlay layers and
workspace inputs. The child adds `WorkspaceStateSnapshot` and
`CompilerDb::workspace_snapshot`, which copy those values into canonical owned
collections. No host filesystem, watcher, URI, document version, or LSP wire
surface is introduced.

## Evidence and compatibility

Focused tests cover canonical file/input ordering, overlay visibility,
per-input and session revisions, exact bytes, lookup, post-capture mutation,
and the compiler accessor. No diagnostic allocation, schema, Semantic ID,
language behavior, runtime, bytecode, VM, ABI, or Unicode table changed.

## Intentionally deferred

Workspace roots, watcher/event sources, symlink/path policy, coalescing and
debounce, dependency graph reload scope, failure-atomic publication, stale
request results, cancellation, diagnostics refresh, URI/version fields, JSON-
RPC methods, and Stable 1.0 claims remain in the blocked `LSP-2105` parent.
