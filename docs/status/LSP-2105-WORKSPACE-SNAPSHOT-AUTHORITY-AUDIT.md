# LSP-2105-WORKSPACE-SNAPSHOT Authority Audit

## Outcome

`LSP-2105-WORKSPACE-SNAPSHOT` is a bounded child authorized by Accepted
`DEC-0071` and the internal revision boundary in `DEC-0019`. Accepted RFC-0030
now consumes this compiler-owned capture in the public `LSP-2105` reload
boundary without changing the child's original scope or evidence.

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
  capture; Accepted `RFC-0030` separately authorizes its reload consumer.

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

Host watcher/event sources, file URI mapping, symlink/path policy, debounce,
compiler-result staleness, cancellation, diagnostics refresh, Workspace Edits,
Semantic Transactions, and Stable claims remain deferred. RFC-0030 now owns
bounded logical reload publication and failure atomicity.
