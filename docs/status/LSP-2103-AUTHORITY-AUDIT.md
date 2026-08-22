# LSP-2103 Authority Audit: Open-document overlay

## Outcome

`LSP-2103` remains `BlockedSpec` for the complete synchronization and
transaction surface. Accepted RFC-0023 authorizes and the child task
`LSP-2103-OVERLAY` implements a bounded full-text Preview overlay: restricted
path-free URIs, monotonic versions, open/change/close state, dependency
read-only checks, and immutable VFS precedence. Incremental ranges, filesystem
URI mapping, compiler snapshots, diagnostics, edits, and transactions remain
deferred.

No host filesystem resolver, range-edit API, diagnostic adapter, Workspace Edit,
or Semantic Transaction was added. Internal VFS behavior remains deterministic
and path-free.

## Normative traceability

- Accepted DEC-0019 defines deterministic source/VFS snapshots, overlays,
  revisions, query invalidation, and cancellation boundaries for compiler
  services. It does not define LSP URIs, client version numbers, JSON-RPC
  notifications, or editor lifecycle semantics.
- Accepted DEC-0002 keeps original UTF-8 bytes/spans authoritative; an editor
  projection must preserve that identity and derive positions explicitly.
- Accepted RFC-0004 defines the Preview lifecycle and framed `ling lsp --stdio`
  transport without document synchronization.
- Accepted RFC-0023 defines the optional `ling.lsp.overlay/0.1` full-text
  overlay, restricted URI forms, monotonic version rules, dependency read-only
  policy, and close/reveal behavior.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves document/snapshot/version and
  Workspace Edit behavior open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves
  public semantic revision/lifecycle fields open.
- `LSP-2101` and `LSP-2102` remain blocked on the missing lifecycle and position
  encoding decisions. No LSP protocol inventory entry authorizes an overlay.

## Current interface evidence

The current repository confirms the split boundary:

- `ling-source::Vfs` and `ling-db::CompilerDb` provide internal `open_overlay`,
  `close_overlay`, immutable bytes, revisions, and disk/overlay visibility.
- `ling-lsp` now adapts those APIs through the RFC-0023 URI/version boundary,
  while keeping `SourceId` and revisions out of the wire protocol.
- The overlay is full-text only and uses no host path resolution. It does not
  define range conversion, compiler query snapshots, or stale result handling.

## Required authority before implementation

The remaining parent task requires accepted authority for, at minimum:

1. file URI and workspace-root mapping, package/dependency discovery, and
   generated/virtual source policy;
2. incremental range edits tied to negotiated position encoding, batch order,
   bounds, and failure atomicity;
3. compiler snapshot identity, stale-result/cancellation behavior, and query
   invalidation across overlays;
4. diagnostics, Workspace Edits, Semantic Transactions, lifecycle labels, and
   migration/compatibility fields; and
5. positive, negative, range, stale-result, dependency, Unicode/CRLF, and
   editor/compiler differential fixtures.

Until those remaining decisions and fixtures are Accepted, adding range edits,
compiler queries, or mutation protocols would make snapshot and transaction
policy an accidental compatibility contract.

## Evidence and compatibility

This audit was checked against `docs/RFC-0023.md`,
`docs/decisions/0019-incremental-query-boundary.md`,
`docs/decisions/0002-source-position-units.md`, `docs/SEMANTICS.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source/src/vfs.rs`, `crates/ling-lsp/src/lib.rs`, and
`crates/ling-db/src/lib.rs`. The bounded Experimental overlay protocol changed
only LSP document synchronization; no diagnostic allocation, core schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 contract
changed.

## Intentionally deferred

The child full-text overlay is complete under RFC-0023. The parent
`LSP-2103` remains deferred until range edits, compiler snapshot/version
semantics, cancellation, and transaction/diagnostic authorities are Accepted.
Any follow-on implementation must continue adapting the internal VFS without
exposing Rust identities and must reject stale or read-only edits before query
publication.
