# DEC-0030: LSP request snapshot internal capture boundary / LSP 请求快照内部边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0019`, `RFC-0004`, `RFC-0023`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision fixes only an internal, immutable capture operation for the
existing `ling-lsp` server. It does not define a JSON-RPC request, a public
analysis protocol, a `CompilerHost`, a Semantic Transaction, or a stale-result
publication rule.

## Question

Future LSP analysis needs one stable view of the visible VFS layer while the
server continues to accept later document updates. DEC-0019 already defines
session-local immutable VFS revisions and RFC-0023 defines full-text overlay
versions, but neither freezes how an in-process analysis task captures those
values without exposing host paths or conflating revisions with client
versions.

## Decision

1. `ling_lsp::LspServer::capture_request_snapshot` captures all currently
   tracked visible documents in canonical URI order and returns an owned
   `RequestSnapshot`. The capture is a read-only operation and never publishes
   a JSON-RPC response.
2. Each `RequestDocument` stores only path-free URI/logical-name text,
   original visible UTF-8 bytes, disk/overlay origin, open state, optional
   client document version, and the session-local VFS `Revision`. It does not
   expose `SourceId`, host paths, allocation identities, or map order.
3. The snapshot also stores the negotiated position encoding, lifecycle state,
   and the current session-local VFS revision. The VFS revision is an internal
   monotonic observation only; it is not a client version, Semantic ID,
   serialized cache key, or cross-process identity.
4. Snapshot values own immutable byte storage. A later `didChange`, `didClose`,
   or disk publication cannot mutate an already captured snapshot, and an
   analysis task need not hold a VFS write borrow while consuming it.
5. An open document reports `Some(client_version)`; a disk-only or closed
   document reports `None`. Client versions and VFS revisions remain distinct
   even when their numeric values happen to match.
6. A missing VFS entry during capture is an internal invariant error and the
   operation returns no partial snapshot. This decision adds no diagnostic
   allocation and no wire error mapping.

## Conformance plan

- Capture disk and overlay documents, then mutate and close them; the first
  snapshot must retain exact original bytes, origin, open state, and versions.
- Verify deterministic URI ordering, distinct client-version/VFS-revision
  fields, negotiated encoding/state retention, and path-free accessors.
- Verify a failed invariant capture cannot publish a partial value and that
  temporary-document close/removal is reflected only in later snapshots.
- Repeat captures across identical event sequences and compare all exposed
  values without relying on `SourceId` or host paths.

## Compatibility impact

- Adds an internal `ling-source` VFS revision accessor and an internal
  `ling-lsp` Rust snapshot value; no language syntax, semantics, diagnostics,
  JSON schema, LSP method, CLI command, Semantic ID, bytecode, VM, or Unicode
  behavior changes.
- Preserves DEC-0002 original UTF-8 bytes and RFC-0023 overlay/version rules.
- The snapshot API is not a public wire protocol and is not added to the
  protocol inventory or support matrix.

## Unresolved alternatives

JSON-RPC request IDs, analysis/query inputs beyond visible documents, project
and profile identity, dependency/config snapshots, cancellation/deadlines,
stale-result handling, memory limits, Workspace Edits, Semantic Transactions,
and Stable versus Experimental wire fields require later accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
