# LSP-2501-SNAPSHOT Implementation Report: Internal Request Capture

## Status

`Done` for the bounded internal child authorized by Accepted DEC-0030. The
child is now consumed by completed LSP-2501 together with the request-specific
Accepted RFCs; it still does not define a standalone JSON-RPC snapshot method
or public revision identity.

## Normative clauses covered

- DEC-0030 §1: `LspServer::capture_request_snapshot` captures the complete
  visible document set in deterministic URI order without publishing a wire
  response.
- DEC-0030 §2–§3: `RequestDocument` and `RequestSnapshot` expose only
  path-free URI/logical-name data, exact visible bytes, disk/overlay origin,
  open state, optional client version, negotiated encoding, lifecycle state,
  and session-local VFS revisions.
- DEC-0030 §4–§6: captured bytes are owned and immutable, later VFS changes do
  not mutate prior values, client versions remain distinct from VFS revisions,
  and an invariant failure returns no partial snapshot.
- DEC-0019 and RFC-0023 remain authoritative for the underlying VFS revision
  and full-text overlay behavior; DEC-0002 remains authoritative for original
  UTF-8 source bytes.

## Implementation

- `crates/ling-source/src/vfs.rs` exposes the latest session-local revision as
  an internal capture boundary.
- `crates/ling-lsp/src/lib.rs` adds `RequestDocument`, `RequestSnapshot`, the
  typed invariant error, and `LspServer::capture_request_snapshot`.
- `crates/ling-lsp/tests/request_snapshot.rs` covers deterministic ordering,
  disk/overlay precedence, immutable values across edits, close behavior,
  client-version/VFS-revision separation, and negotiated state retention.

No JSON-RPC method, protocol inventory entry, support-matrix claim,
diagnostic allocation, CompilerHost API, semantic query, Workspace Edit, or
stale-result rule was added.

## Verification

```text
cargo test -p ling-lsp --test request_snapshot --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
```

Both commands pass. Full workspace and governance gates are required before
the completion hash is recorded.

## Compatibility and determinism

The snapshot owns exact UTF-8 bytes and copies only path-free logical metadata;
it does not expose `SourceId`, host paths, allocation identity, or map order.
URI ordering and repeated event sequences are deterministic. Session-local
revision values are not Semantic IDs, cache keys, client versions, or
cross-process identities. Language, diagnostics, schemas, bytecode, VM,
Unicode 17.0.0, CLI, and LSP wire behavior are unchanged.

## Deferred work after parent integration

Public/cross-session snapshot identities, generalized CompilerHost integration,
deadlines, asynchronous scheduling, persistent result identity, Semantic
Transactions, and Stable wire compatibility remain future work outside
completed LSP-2501.
