# LSP-2105 authority audit: workspace reload

## Outcome

`LSP-2105` is implementation-ready and complete under Accepted `RFC-0030`.
The RFC resolves the reload boundary as bounded client/host publication of
logical source and project-input deltas against an exact session revision. It
does not require Ling to own a host filesystem watcher or expose path behavior.

## Normative traceability

- Accepted `RFC-0030` defines `ling.lsp.workspace/0.1`, exact capability and
  request/result fields, canonical decimal revision identity, source/input
  limits, canonical application order, overlay policy, errors, and atomicity.
- Accepted `DEC-0019` authorizes revision-aware source/project inputs and
  targeted lazy invalidation rather than eager full compilation.
- Accepted `DEC-0071` authorizes immutable canonical workspace-state capture.
- Accepted `RFC-0002` and `RFC-0025` define the single-root locked/offline
  project model whose manifest and lock bytes are reload inputs.
- Accepted `RFC-0004` and `RFC-0023` define lifecycle, transport, path-free URI,
  overlay precedence, and monotonic document-version behavior.

## Plan traceability

The implementation satisfies `04-LSP-IMPLEMENTATION.md` LSP-2105 as follows:

- it responds to source, dependency, manifest, lock, config, profile, and
  target changes through one explicit request;
- each changed input receives a session revision and participates in the
  immutable workspace snapshot;
- source updates invalidate their own VFS identities while project inputs join
  query/cache identity, so a single file event does not eagerly rebuild every
  source;
- the full batch is validated against `baseRevision` and published from a
  private clone only after every operation succeeds.

The host observes and coalesces filesystem events before publication. That
choice is deliberate: RFC-0030 prevents watcher timing, paths, symlinks, and OS
event ordering from becoming Ling semantics.

## Evidence and compatibility

Executable tests cover canonical array-order equivalence, exact source/input
bytes, Unicode/BOM/CRLF, all input kinds, no-op revisions, overlay-hidden disk
updates, close behavior, removals, stale/malformed/duplicate/unsupported/
oversized requests, notification suppression, and complete failure atomicity.

The new protocol is Experimental and current-writer-only. Package-lock bytes
join disposable internal query identity, but no public cache format,
cross-process revision, compiler diagnostic, public schema, Semantic ID,
language behavior, runtime, bytecode, VM, ABI, filesystem/network behavior, or
Unicode 17.0.0 data changes.

## Intentionally deferred

Host watcher adapters, `file://` mapping, path/root discovery, symlink policy,
multi-root projects, network dependencies, compiler-result cancellation and
staleness, diagnostics refresh, Workspace Edits, Semantic Transactions, and
Stable compatibility remain governed by later tasks and open gaps. They are
not prerequisites for the bounded atomic reload publication completed here.
