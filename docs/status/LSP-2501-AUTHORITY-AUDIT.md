# LSP-2501 Authority Audit: Request Snapshots

## Outcome

`LSP-2501` is correctly recorded as `BlockedSpec`. The execution plan requires
each request to capture an immutable `AnalysisSnapshot` and document version,
and requires long-running work not to hold a host write lock. The repository
has an accepted internal VFS/revision boundary, but no accepted LSP request
snapshot identity, document-version contract, publication rule, or
`CompilerHost`/`AnalysisSnapshot` API.

No LSP snapshot type, document-version validator, request handler, stale-result
policy, diagnostic allocation, protocol schema, or placeholder compiler host
was added.

## Normative traceability

- The execution package and its `AnalysisSnapshot` pseudocode are
  non-normative; they do not authorize a public LSP request or version field.
- Accepted DEC-0019 defines internal immutable source/project revisions,
  overlay boundaries, query dependencies, invalidation, and cancellation. It
  explicitly does not define an LSP request, document version, JSON-RPC field,
  or editor lifecycle.
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
  values, overlay/disk precedence, and deterministic session-local revisions.
  These are internal VFS inputs, not client document versions or a request
  snapshot protocol.
- `ling-db` and the checked compiler pipeline can retain internal query/source
  revisions, but there is no public `CompilerHost`, `AnalysisSnapshot`, URI to
  `FileId` binding, request context, or snapshot publication boundary.
- The current CLI compiles a path or in-memory source per invocation; no LSP
  server pins a request to source, project, profile, toolchain, Unicode, or
  dependency inputs while another revision is published.
- No fixture covers edits during analysis, stale-result rejection, duplicate or
  non-monotonic document versions, overlay/disk races, dependency snapshots,
  concurrent readers/writers, cancellation, limits, or deterministic snapshot
  identity.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. request snapshot identity and contents, including URI/logical `FileId`,
   project/package scope, visible overlay, compiler profile, target,
   toolchain, language and Unicode versions, and dependency/config inputs;
2. the distinction between client document version, VFS revision, query
   revision, and semantic snapshot identity, including monotonicity,
   duplicate/unknown versions, open/closed state, and disk races;
3. immutable capture and lifetime rules, ownership/read-lock behavior, writer
   publication, cancellation/deadline interaction, memory/retention limits,
   and the guarantee that long work cannot observe a partially published
   source or hold a host write lock;
4. response association and stale handling: required version/snapshot fields,
   reject/ignore/replace behavior, errors, diagnostics, full/delta bases,
   Workspace Edits, and cross-file/dependency consistency;
5. JSON-RPC/request schemas, capability negotiation, protocol inventory,
   Stable versus Experimental fields, migration/versioning, and executable
   positive/negative fixtures for races, stale/duplicate versions, Unicode,
   CRLF/BOM, deterministic identities, cancellation, limits, and isolation.

Until these decisions are Accepted, a request could publish analysis for the
wrong editor revision, conflate an internal revision with a client version, or
expose host locking and cache behavior as an accidental public contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0002, DEC-0019,
`docs/ling_execution_plan/01-REPOSITORY-AND-COMPILER-ARCHITECTURE.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-source`, `crates/ling-db`, and the current compiler/CLI tests.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2501` can begin after LSP URI/open-document and incremental-change
decisions, the compiler-host/analysis-snapshot boundary, and LSP/Semantic
Transaction lifecycle rules are Accepted. The future implementation must
capture one immutable, explicitly versioned input set per request, publish
only results associated with that set, and keep internal revisions separate
from public client versions.
