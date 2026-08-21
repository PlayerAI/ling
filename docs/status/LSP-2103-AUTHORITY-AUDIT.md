# LSP-2103 Authority Audit: Open-document overlay

## Outcome

`LSP-2103` is correctly recorded as `BlockedSpec`. The repository already has
an internal deterministic VFS overlay and revision boundary, but the execution
plan's LSP layer additionally requires URI-to-FileId identity, client document
versions, open/closed state, read-only dependency policy, and `didOpen`/
`didClose` behavior. No accepted LSP protocol defines those fields or
transitions.

No LSP document state, URI mapper, version validator, editor overlay API,
dependency edit guard, or placeholder server was added. Internal VFS behavior
remains unchanged.

## Normative traceability

- Accepted DEC-0019 defines deterministic source/VFS snapshots, overlays,
  revisions, query invalidation, and cancellation boundaries for compiler
  services. It does not define LSP URIs, client version numbers, JSON-RPC
  notifications, or editor lifecycle semantics.
- Accepted DEC-0002 keeps original UTF-8 bytes/spans authoritative; an editor
  projection must preserve that identity and derive positions explicitly.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves document/snapshot/version and
  Workspace Edit behavior open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves
  public semantic revision/lifecycle fields open.
- `LSP-2101` and `LSP-2102` remain blocked on the missing lifecycle and position
  encoding decisions. No LSP protocol inventory entry authorizes an overlay.

## Current interface evidence

The current repository confirms the split boundary:

- `ling-source::Vfs` and `ling-db::CompilerDb` provide internal `open_overlay`,
  `close_overlay`, immutable bytes, revisions, and disk/overlay visibility.
- Those APIs identify files with internal `SourceId`; they do not map URIs,
  enforce client document versions, classify dependency/read-only files, or
  emit LSP notifications.
- No LSP server, workspace-folder root policy, open/close fixture, or stale
  notification behavior exists. Reusing the internal VFS directly would expose
  implementation identity and revision rules as an unversioned protocol.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. URI normalization and workspace-root mapping to path-free logical names and
   `FileId`, including virtual/untitled and dependency documents;
2. document version type, monotonicity/duplicate policy, open/closed state,
   overlay precedence, disk changes while open, and close/reveal behavior;
3. writable/read-only dependency policy, generated/virtual file handling,
   snapshot/revision association, and stale-result behavior;
4. notification/request schemas, position encoding dependency, errors,
   cancellation, resource limits, and Stable versus Experimental fields; and
5. positive, negative, stale/duplicate version, disk-race, dependency,
   URI/Unicode/CRLF, deterministic, and lifecycle fixtures.

Until those decisions and fixtures are Accepted, adding an LSP overlay would
make client version and URI policy an accidental compatibility contract and
could invalidate compiler snapshots or accept edits to dependencies.

## Evidence and compatibility

This audit was checked against `docs/decisions/0019-incremental-query-boundary.md`,
`docs/decisions/0002-source-position-units.md`, `docs/SEMANTICS.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source/src/vfs.rs`, and `crates/ling-db/src/lib.rs`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2103` can begin after LSP lifecycle/position decisions and a versioned
overlay contract are Accepted. The implementation should adapt the internal
VFS without exposing its Rust identities, preserve immutable snapshots, and
reject stale or read-only edits before they reach compiler queries.
