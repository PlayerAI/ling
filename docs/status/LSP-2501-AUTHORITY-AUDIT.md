# LSP-2501 Authority Audit: Request Snapshots

## Outcome

`LSP-2501` remains correctly recorded as `BlockedSpec` for the full public
request-snapshot and analysis contract. Accepted DEC-0030 authorizes and child
`LSP-2501-SNAPSHOT` implements only an internal immutable capture boundary:
visible documents, overlay origin, client-version separation, negotiated
encoding, and session-local VFS revisions. The repository still has no
accepted public LSP request identity, publication rule, or
`CompilerHost`/semantic `AnalysisSnapshot` API.

No JSON-RPC snapshot method, document-version validator beyond the existing
RFC-0023 overlay checks, stale-result policy, diagnostic allocation, protocol
schema, or placeholder compiler host was added.

## Normative traceability

- The execution package and its `AnalysisSnapshot` pseudocode are
  non-normative; they do not authorize a public LSP request or version field.
- Accepted DEC-0019 defines internal immutable source/project revisions,
  overlay boundaries, query dependencies, invalidation, and cancellation. It
  explicitly does not define an LSP request, document version, JSON-RPC field,
  or editor lifecycle.
- Accepted DEC-0030 defines only the in-process `ling-lsp` capture value and
  explicitly leaves public request identity, analysis/query inputs, and
  publication semantics unresolved.
- DEC-0002 keeps original UTF-8 bytes and `SourceId + Span` authoritative. A
  request adapter must derive any editor position from an explicit SourceMap;
  it must not replace compiler spans with document-version metadata.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version preconditions and
  Stable versus Experimental editor fields open. The related
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves public revision lifecycle,
  stale rejection, reader/writer compatibility, and migration open.
- LSP-2103 and LSP-2104 remain `BlockedSpec`; their URI/open-document and
  incremental-change contracts are prerequisites for associating a request
  with the correct visible source.

## Current interface evidence

- `ling-source` provides immutable `FileSnapshot` and `WorkspaceSnapshot`
  values, overlay/disk precedence, and deterministic session-local revisions;
  the `LSP-2501-SNAPSHOT` child now captures these into an owned, path-free
  `RequestSnapshot`. These remain internal VFS inputs, not a client protocol.
- `ling-db` and the checked compiler pipeline can retain internal query/source
  revisions, but there is no public `CompilerHost`, semantic
  `AnalysisSnapshot`, URI-to-`FileId` binding, request context, or result
  publication boundary.
- The current CLI compiles a path or in-memory source per invocation; no LSP
  server pins a request to project, profile, toolchain, Unicode, or dependency
  inputs while another revision is published.
- The child fixture covers immutable capture across edits/close, deterministic
  ordering, origin, and client-version/VFS-revision separation. No fixture yet
  covers public edits during analysis, stale-result rejection, duplicate or
  non-monotonic document versions, dependency snapshots, cancellation,
  limits, or public deterministic snapshot identity.

## Required authority before parent implementation

An Accepted RFC or decision must still define, at minimum:

1. public request snapshot identity and contents, including URI/logical
   `FileId`, project/package scope, compiler profile, target, toolchain,
   language/Unicode versions, and dependency/config inputs;
2. the distinction between client document version, VFS revision, query
   revision, and semantic snapshot identity, including monotonicity,
   duplicate/unknown versions, open/closed state, and disk races;
3. immutable capture lifetime, cancellation/deadline interaction,
   memory/retention limits, and publication guarantees;
4. response association and stale handling, errors, diagnostics, full/delta
   bases, Workspace Edits, and cross-file/dependency consistency; and
5. JSON-RPC/request schemas, capability negotiation, protocol lifecycle,
   migration/versioning, and executable race/isolation fixtures.

Until those decisions are Accepted, a public request could publish analysis for
the wrong editor revision or expose internal cache and locking behavior.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0002, DEC-0019, DEC-0030,
RFC-0004, RFC-0023, the execution plan, governance registries, `ling-source`,
`ling-lsp`, `ling-db`, and the child integration tests.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed. The child adds no
wire protocol or support-matrix claim.

## Intentionally deferred

The `LSP-2501-SNAPSHOT` child is complete under DEC-0030. The parent
`LSP-2501` remains blocked until LSP URI/open-document and incremental-change
decisions, the compiler-host/analysis-snapshot boundary, and LSP/Semantic
Transaction lifecycle rules are Accepted. Future work must capture one
immutable, explicitly versioned input set per request, publish only results
associated with that set, and keep internal revisions separate from public
client versions.
