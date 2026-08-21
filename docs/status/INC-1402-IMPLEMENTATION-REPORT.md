# INC-1402 Implementation Report: VFS and Revision Boundary

## Outcome

INC-1402 is complete. `ling-source` now provides a host-independent,
session-local virtual file system for immutable source snapshots and the
workspace inputs that will participate in the internal query graph. The
implementation is deliberately in-memory and keeps exact caller-provided
bytes; it does not read the host filesystem or publish a cache format.

## Normative traceability

- Accepted `DEC-0019` Decision §§1–3 authorize immutable query inputs, exact
  retained UTF-8 source bytes, canonical logical names, source/project
  revisions, and deterministic cache-key inputs.
- Accepted `DEC-0019` Decision §§4–8 keep the first implementation
  single-threaded, internal, non-persistent, and free of public protocol or
  diagnostic compatibility claims.
- Existing source-position rules remain authoritative: `SourceFile` still
  owns BOM/line-ending normalization and all original UTF-8 byte spans; the
  VFS stores the bytes without rewriting them.

## Implemented boundary

- `VirtualFileSystem` allocates deterministic session-local `SourceId` and
  `Revision` values and retains disk layers as immutable `Arc<[u8]>` values.
- `FileSnapshot` exposes the visible bytes, canonical logical name, source
  ID, revision, and whether the visible layer is `Disk` or `Overlay`.
- Editor overlays can be opened, replaced, and closed. Disk changes received
  while an overlay is open update the hidden disk layer without invalidating
  the visible overlay; closing the overlay reveals the latest disk bytes.
- `WorkspaceInput` covers package manifest, config, profile, and target
  revisions through the same exact-byte and duplicate-update rules.
- `ChangeEvent` and `InputChange` distinguish additions, visible changes, and
  deduplicated updates. Snapshot enumeration uses canonical logical-name order
  rather than map or host filesystem order.
- Logical names reject empty segments, traversal, backslashes, NUL bytes, and
  drive-style colons. No host path or persistent cache key is exposed.

## Evidence

- `crates/ling-source/src/vfs.rs` contains ten focused unit tests together
  with the existing source-map tests: immutable duplicate disk updates,
  overlay hiding/reveal, workspace-input deduplication, canonical ordering,
  and invalid-name rejection.
- The execution backlog marks INC-1402 Done, and the machine status registry
  records the implementation commit and acceptance evidence.
- The open incremental-cache gap remains limited to persistent schema and
  migration, corruption recovery, parallel scheduling, and later query
  invalidation decisions.

## Compatibility and deferred work

- No language syntax or semantics, diagnostic code, schema, Semantic ID,
  canonical semantic bytes, CLI/LSP field, public protocol, or Unicode table
  changed.
- Host filesystem watching, persistent cache serialization, query evaluation,
  line-index/query nodes, cancellation of later query work, and LSP document
  version adapters remain separate targets.

## Validation

The focused test, clippy, formatting, and diff checks passed. The completion
commit and machine-readable status evidence are recorded in
`docs/status/implementation-status.toml`.
