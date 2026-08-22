# LSP-2104 Authority Audit: Incremental text changes

## Outcome

`LSP-2104` is correctly recorded as `BlockedSpec`. The execution plan allows a
Full-sync baseline and later asks for Incremental sync with negotiated-position
range conversion, ordered batch application, LineIndex updates, version and
boundary rejection, and equivalence with full replacement. The repository has
no accepted LSP change schema or adapter contract for those operations.

Accepted `DEC-0069` now authorizes the bounded `LSP-2104-UTF8-EDITS` child. That
child implements only immutable source-layer byte-range application and its
tests; it does not close the parent public LSP, position, version, VFS, or
transaction boundary.

No incremental-change parser, range application API, LSP version validator,
partial-edit transaction, or placeholder server was added. Existing VFS full
snapshot and compiler incremental-query behavior remains unchanged.

## Normative traceability

- Accepted DEC-0019 defines immutable source snapshots, revisions, query
  invalidation, and clean/incremental equivalence at the compiler boundary. It
  does not define LSP `didChange` ranges, batch ordering, or document-version
  messages.
- Accepted DEC-0002 makes UTF-8 byte spans authoritative and requires any
  future UTF-16 position projection to be explicit; it does not define range
  conversion failure or edit normalization.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves position/snapshot/version and
  Workspace Edit behavior open. LSP-2101 through LSP-2103 remain blocked on
  lifecycle, position, and overlay contracts.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-source::Vfs` accepts complete byte snapshots and provides deterministic
  revisions/overlays; it has no LSP range decoder or client-version type.
- `ling-db` proves clean versus incremental query results equivalent after
  source replacement, but its compiler query input is already a validated
  snapshot rather than a JSON-RPC edit batch.
- `ling-source` now provides `Utf8Edit` and ordered immutable application under
  `DEC-0069`; the primitive rejects scalar/CRLF interior boundaries and
  revalidates every resulting snapshot without publishing it to the VFS.
- No public adapter defines range interpretation across UTF-8/UTF-16, client
  change ordering, stale-version response, or the required LSP fixtures.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. Full versus Incremental sync capabilities, change message schema, range
   encoding, end-position/newline rules, and batch ordering;
2. document-version monotonicity, duplicate/stale policy, atomic application,
   failure response, overlay/revision publication, and query cancellation;
3. conversion behavior for UTF-8/UTF-16, CRLF/BOM/Unicode boundaries, invalid
   ranges, limits, and source-span preservation;
4. interaction with LSP lifecycle, workspace/dependency read-only policy,
   snapshots, diagnostics, and Stable versus Experimental fields; and
5. positive, negative, multi-edit, full-equivalence, stale/duplicate version,
   invalid-boundary, Unicode/CRLF/BOM, deterministic, and migration fixtures.

Until the remaining public decisions and fixtures are Accepted, adapting the
source primitive to LSP ranges would risk partial VFS mutation, stale semantic
results, or a position unit that conflicts with DEC-0002. The bounded child is
therefore evidence only, not completion of the parent.

## Evidence and compatibility

This audit was checked against `docs/decisions/0019-incremental-query-boundary.md`,
`docs/decisions/0002-source-position-units.md`, `docs/SEMANTICS.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source/src/vfs.rs`, `crates/ling-source/src/lib.rs`, and
`crates/ling-db/src/lib.rs`.
The bounded child adds no public protocol behavior; no diagnostic allocation,
schema, Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0
claim is made. See `docs/status/LSP-2104-UTF8-EDITS-AUTHORITY-AUDIT.md` and
`docs/status/LSP-2104-UTF8-EDITS-IMPLEMENTATION-REPORT.md` for its evidence.

## Intentionally deferred

The full `LSP-2104` target can begin after the LSP position/overlay/version
contract is Accepted. The public implementation must apply each change batch
atomically, preserve full-replacement equivalence, reject stale/invalid ranges
before publication, and keep compiler byte spans authoritative. The
`LSP-2104-UTF8-EDITS` source-only child is complete under `DEC-0069`.
