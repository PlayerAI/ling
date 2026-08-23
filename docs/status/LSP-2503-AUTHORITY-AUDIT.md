# LSP-2503 Authority Audit: Debounce and Priority Scheduling

## Outcome

LSP-2503 is authorized by Accepted RFC-0050. The RFC closes the previously
missing observable scheduling boundary without introducing host-time-dependent
behavior: request/state execution stays in wire order, derived compiler work
has three logical classes, priority bursts have fixed fairness limits,
diagnostic debounce stays at the RFC-0032 message boundary, and newer accepted
source state cancels an older diagnostic ticket.

## Authority and dependency resolution

- RFC-0004 owns lifecycle, framing, ordered mutable execution, and response
  behavior.
- RFC-0023/RFC-0029/RFC-0030 own source overlays, revisions, atomic changes,
  and complete request snapshots.
- RFC-0032 owns message-boundary diagnostic debounce, immutable tickets,
  current-snapshot validation, replacement/clearance, and atomic publication.
- RFC-0045 makes `workspace/symbol` a bounded snapshot-indexed low-priority
  work family without proactive filesystem indexing.
- RFC-0049 owns live request cancellation, compiler propagation, and atomic
  request publication.
- DEC-0032 supplies the deterministic FIFO/priority primitive; RFC-0050 adds
  fixed fairness bounds, class mapping, wire-order precedence, diagnostic
  supersession, and exact Preview discovery.
- All declared LSP-2503 dependencies are Done. The remaining general LSP
  transaction gap covers broader mutation, quotas, Semantic Transactions, and
  Stable lifecycle rather than this bounded scheduling revision.

## Resolved contract

RFC-0050 fixes:

1. exact `ling.lsp.scheduling/0.1` discovery;
2. Interactive, Analysis, and Background class membership;
3. wire-order request/state barriers and no response reordering;
4. Analysis-before-Background service at a common executor boundary;
5. message-boundary debounce with no timer or latency promise;
6. cancellation of superseded diagnostic tickets plus stale-result rejection;
7. an Analysis admission after at most 8 Interactive selections and a
   Background admission after at most 16 non-Background selections; and
8. non-serialization of queue sequence, counters, tokens, timing, revisions,
   paths, source bytes, and compiler identities.

## Implementation evidence reviewed

- `crates/ling-lsp/src/scheduler.rs` implements canonical strict ordering,
  bounded-fair selection, explicit method classification, and fixed limits.
- `crates/ling-lsp/src/publication.rs` rotates diagnostic cancellation tokens
  on successful mutations, checks cancellation through source/compiler/
  adaptation stages, rejects stale results, and mutates the publication ledger
  only after a final checkpoint.
- `crates/ling-db/src/lib.rs` exposes cancellable complete workspace
  diagnostics and routes semantic checking through the existing typed
  cancellation path without returning a partial collection.
- `crates/ling-lsp/src/lib.rs` advertises exact discovery, retains a single
  wire-order mutable executor, and services pending Analysis before Background
  requests.
- Scheduling, push-diagnostic, cancellation, exact transcript, and compiler DB
  tests cover discovery, class mapping, fairness, coalescing, supersession,
  stale completion, atomic publication, Unicode position behavior, and repeat
  determinism without sleeps.

## Compatibility assessment

The change adds one Preview discovery marker and cancellable diagnostic-query
entry point. It adds no JSON-RPC method, Ling diagnostic, standalone schema,
Semantic ID, Definition ID, source-span rule, language behavior, Typed Core
evaluation, interpreter/runtime/bytecode/VM/ABI behavior, package or host I/O,
or Unicode 17.0.0 change.

## Intentionally deferred

Wall-clock/configurable debounce, deadlines, dynamic priorities, host-load
adaptation, worker pools, parallel mutable requests, response reordering,
progress, partial results, quotas, persistent queues/indexes, Stable editor
compatibility, and Semantic Transactions require later accepted authority.
