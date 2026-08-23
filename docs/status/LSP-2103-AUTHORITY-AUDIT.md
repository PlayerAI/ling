# LSP-2103 Authority Audit: Open-document overlay

## Outcome

`LSP-2103` is implementation-ready and complete under Accepted `DEC-0259`,
which recognizes the already accepted and implemented RFC-0023 full-text
overlay as the exact parent contract. Its dependencies, `LSP-2101` and
`LSP-2102`, are Done.

The former audit over-scoped this task with incremental edits, compiler query
transactions, diagnostics, and Workspace Edits. The execution plan instead
defines a bounded URI-to-file open-document state with overlay precedence,
monotonic versions, deterministic close behavior, and read-only dependencies.
RFC-0023 implements each of those rules.

## Normative traceability

- Accepted `DEC-0002` keeps exact original UTF-8 bytes and spans authoritative.
- Accepted `DEC-0019` defines immutable VFS snapshots, overlay precedence,
  revisions, canonical logical names, and deterministic invalidation.
- Accepted `RFC-0004` supplies lifecycle gates and framed stdio transport.
- Accepted `RFC-0023` defines restricted path-free URIs, full-text open/change/
  close, monotonic client versions, read-only dependencies, disk reveal,
  temporary removal, failure atomicity, and bilingual protocol errors.
- Accepted `DEC-0259` composes that implementation as complete LSP-2103 while
  keeping later method and transaction contracts independent.
- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` requires exactly the
  implemented URI/file state and four overlay rules; it does not require
  incremental ranges or compiler query publication.

## Current implementation evidence

- `LspServer` owns a session-local `VirtualFileSystem`, URI records, monotonic
  version history, deterministic URI-ordered document views, and explicit
  host-provided disk snapshot publication without filesystem access.
- `didOpen` overlays exact editor bytes, `didChange` validates one full-text
  replacement and a strictly newer version, and `didClose` reveals the latest
  disk layer or removes an untitled file.
- Restricted workspace, dependency, and untitled URIs map to canonical logical
  names without exposing `SourceId`, VFS revisions, or host paths. Dependency
  changes and ranged edits are rejected before mutation.
- Tests cover overlay/disk races, version regressions, duplicate opens,
  dependency read-only behavior, temporary removal, invalid URI/params/ranges,
  size limits, response suppression, lifecycle gating, deterministic views,
  and failure atomicity.

## Plan/repository drift resolved

The previous `BlockedSpec` state treated the open
`GAP-LSP-TRANSACTION-PROTOCOL-001` as if every transaction concern belonged to
LSP-2103. DEC-0259 corrects that drift without closing the gap: incremental
ranges, compiler snapshots, stale analysis, diagnostics, cancellation, and
Workspace Edits remain downstream work.

No duplicate overlay, host-path resolver, incremental edit API, or placeholder
transaction surface is authorized by this parent closure.

## Compatibility and determinism

This closure changes no executable behavior, protocol bytes or version,
diagnostic allocation, schema, Semantic ID, source span, runtime, bytecode, VM,
ABI, filesystem/network behavior, or Unicode 17.0.0 data. URI validation,
document ordering, version checks, and VFS publication are deterministic and
independent of host paths, locale, environment, allocation, and hash order.

## Intentionally deferred

Host `file://` resolution, project-root discovery, generated-source policy,
incremental changes, compiler snapshots, stale results, diagnostics
publication, navigation, cancellation, Workspace Edits, Semantic Transactions,
and Stable editor compatibility remain assigned to later tasks.
