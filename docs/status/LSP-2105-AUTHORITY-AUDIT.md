# LSP-2105 Authority Audit: Workspace reload

## Outcome

`LSP-2105` is correctly recorded as `BlockedSpec`. The execution plan asks the
server to react to manifest/source changes, advance project graph revisions for
configuration/lock/dependency changes, and avoid treating every file event as a
full rebuild. The compiler already has internal revision and invalidation
boundaries, but no accepted LSP workspace notification or project reload
contract exposes them.

No filesystem watcher, workspace reload notification, manifest/lock watcher,
project revision API, stale-result handler, or placeholder LSP service was
added. Existing compiler/VFS revision behavior remains unchanged.

## Normative traceability

- Accepted DEC-0019 defines workspace/project inputs, revisions, dependency
  invalidation, deterministic query scheduling, and clean/incremental
  equivalence at the compiler service boundary. It does not define LSP
  workspace-folder notifications, watcher ownership, or reload responses.
- Accepted RFC-0002 and existing package/lock decisions define library graph and
  lock models, but project CLI/editor selection and reload behavior remain
  outside the accepted public protocol.
- `GAP-INCREMENTAL-CACHE-001` keeps cache keys, invalidation, persistence, and
  corruption/recovery policy open; `GAP-PROJECT-CLI-INTERFACE-001` keeps project
  selection and lock/offline command behavior open.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave snapshot/version and public
  semantic lifecycle fields unresolved.

## Current interface evidence

The current repository confirms the split boundary:

- `ling-source` and `ling-db` track immutable source/project input revisions
  and invalidate dependent queries deterministically; they do not own OS
  watchers, LSP workspace folders, or notification acknowledgements.
- `ling-project` reads explicit manifests/locks and builds deterministic graphs;
  it has no accepted editor reload service or policy for partially changed
  dependency trees.
- No protocol entry or fixture defines which file/config events trigger which
  revision, how concurrent reloads are coalesced, or how requests pinned to an
  old graph are rejected.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. workspace-folder/root identity, watcher ownership, supported file systems,
   symlink/path policy, and manifest/lock/config/dependency event sources;
2. event coalescing/debouncing, deterministic revision allocation, dependency
   graph rebuild scope, cache invalidation, and failure-atomic publication;
3. request snapshot/version association, stale-result/cancellation behavior,
   concurrent reload ordering, and resource limits;
4. LSP notification/request schemas, diagnostics/semantic snapshot refresh,
   offline/locked behavior, and Stable versus Experimental lifecycle; and
5. positive, negative, event-order, partial-failure, dependency, lock/config,
   symlink/Unicode/CRLF, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, implementing reload behavior
would choose an editor/project identity and invalidation policy that could
silently publish stale semantics or rebuild too broadly.

## Evidence and compatibility

This audit was checked against `docs/decisions/0019-incremental-query-boundary.md`,
`docs/RFC-0002.md`, `docs/SEMANTICS.md`, `docs/ROADMAP-1.0.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source`, `crates/ling-db`, and `crates/ling-project`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2105` can begin after the project reload, LSP lifecycle, and incremental
cache contracts are Accepted. The implementation should coalesce events into
explicit immutable revisions, invalidate only affected queries, and reject
stale results without exposing host watcher details as Ling semantics.
