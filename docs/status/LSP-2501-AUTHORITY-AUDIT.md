# LSP-2501 Authority Audit: Request Snapshots

## Outcome

`LSP-2501` is authorized for composed completion. Accepted DEC-0030 defines the
owned, path-free internal `RequestSnapshot`; RFC-0023 and RFC-0029 define
versioned overlay publication; RFC-0030 adds complete workspace-input capture;
and the Accepted request RFCs through RFC-0048 define how every current
compiler-backed LSP request selects, consumes, revalidates, and publishes from
an immutable snapshot.

The earlier audit incorrectly treated the execution plan's illustrative
`CompilerHost`/`AnalysisSnapshot` names and a public snapshot identity as
required deliverables. They are not normative. No JSON-RPC snapshot method or
serialized VFS/query revision is needed: public request association uses the
standard method parameters and JSON-RPC ID, while snapshot identity remains an
internal equality/freshness boundary.

## Normative traceability

- DEC-0002 preserves original UTF-8 bytes and compiler spans; adapters project
  positions only through the negotiated SourceMap encoding.
- DEC-0019 defines immutable internal VFS/query inputs, deterministic
  invalidation, owned consumption, and cancellation boundaries without making
  revisions public semantics.
- RFC-0023 and RFC-0029 define accepted URI forms, open/closed overlays,
  strictly increasing client versions, atomic full/incremental changes, and
  separation of client versions from VFS revisions.
- DEC-0030 defines `RequestSnapshot`/`RequestDocument`, canonical URI order,
  exact owned visible bytes, origin/open/client-version fields, lifecycle and
  negotiated encoding, immutable lifetime, and failure atomicity.
- RFC-0030 adds manifest, lock, config, profile, and target snapshots in
  canonical input order and atomic workspace publication.
- RFC-0026 §3–4 authorizes synchronous formatting from the exact immutable VFS
  document snapshot and client version. The single-threaded dispatcher writes
  the response before accepting a later change, so no concurrent recapture is
  required for that bounded request.
- RFC-0032/RFC-0033 define diagnostic analysis tickets, complete-snapshot
  equality, URI/version association, stale rejection, and atomic push/pull
  publication.
- RFC-0036–RFC-0045 define complete snapshot capture and freshness for document
  symbols, hover, navigation, references, prepare rename, rename, completion,
  completion resolve, code actions, and workspace symbols.
- RFC-0048 defines the same boundary for semantic-token full/delta generation,
  client document versions, cancellation, result publication, and stale
  rejection.

The remaining semantic-transaction lifecycle gaps govern future cross-request
or cross-tool protocols; they do not invalidate the bounded Preview request
contracts already Accepted and implemented.

## Current implementation boundary

`LspServer::capture_request_snapshot` owns the complete visible document and
workspace-input state. Each `RequestDocument` carries exact bytes, URI/logical
name, origin, open/writable/temporary state, optional client version, and an
internal revision. The snapshot also carries lifecycle state, negotiated
position encoding, the workspace revision, and canonical project inputs.

Compiler-backed handlers consume only the captured value and, where their RFC
permits work that could outlive a state observation, recapture and require
complete equality before success. Diagnostic tickets compile after capture
without borrowing the mutable host. Formatting is the explicit RFC-0026
synchronous exception. Lifecycle, overlay, and workspace-reload methods mutate
or negotiate server state and therefore are not analysis requests.

The implementation exposes no host lock, filesystem path, `SourceId`, Rust
allocation, hash-map order, query key, Semantic ID, or serialized snapshot
revision. Client document versions are never inferred from internal revisions.

## Acceptance evidence

- `crates/ling-lsp/tests/request_snapshot.rs` proves owned immutability, exact
  visible bytes, deterministic URI order, disk/overlay precedence, close
  behavior, negotiated state, and client-version/VFS-revision separation.
- `crates/ling-lsp/tests/workspace_reload.rs` proves complete source and all
  five workspace-input snapshots, canonical ordering, equality across event
  order, immutable prior captures, atomic failure, and revision freshness.
- diagnostic publication tests prove capture/compile/publish tickets reject
  changed complete snapshots and associate results with exact current URIs and
  versions.
- request-specific suites for formatting, document symbols, hover, navigation,
  references, prepare rename, rename, completion/resolve, code actions,
  workspace symbols, and semantic tokens prove their Accepted snapshot and
  stale-publication contracts.
- `tests/protocols/lsp-request-snapshot/README.md` provides the reviewable
  authority-to-reader matrix without creating a new protocol.

## Specification gaps and compatibility

No unresolved decision blocks the current LSP-2501 scope. A future public
snapshot token, asynchronous scheduler, persistent/cross-session identity,
Semantic Transaction, deadline, or generalized `CompilerHost` would require
separate Accepted authority and is not implied here.

This composed completion adds no method, capability, protocol marker, public
schema, diagnostic, Semantic ID, canonical bytes, language semantics, compiler
checked fact, runtime, bytecode, VM, ABI, dependency, filesystem/network
behavior, or Unicode 17.0.0 data.
