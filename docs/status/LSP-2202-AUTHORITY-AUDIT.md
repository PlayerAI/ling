# LSP-2202 Authority Audit: Push diagnostics v0

## Outcome

`LSP-2202` is implementation-ready under Accepted RFC-0032. The RFC closes
the previously recorded publication/scheduling portion of
`GAP-LSP-TRANSACTION-PROTOCOL-001` for this bounded Experimental writer without
claiming a general Semantic Transaction or asynchronous scheduling contract.

RFC-0004/RFC-0023/RFC-0029/RFC-0030 govern lifecycle, overlays, incremental
edits, and workspace snapshots. RFC-0031 defines the diagnostic value adapter;
RFC-0032 compatibly advances it to 0.2 for temporary sources and defines the
separate `ling.lsp.publish-diagnostics/0.1` lifecycle.

## Normative traceability

- RFC-0032 §§1–3 define exact state-change triggers, deterministic logical
  debounce, immutable complete-state tickets, full-identity freshness, syntax
  precedence, checked-workspace analysis, and internal-failure atomicity.
- RFC-0032 §§4–6 define the adapter 0.2 source-set extension, exact
  `publishDiagnostics` params, version/clearance/ledger behavior, URI order,
  failure atomicity, and initialize capability marker.
- RFC-0004 defines lifecycle/transport and negotiated position encoding;
  RFC-0023/RFC-0029 define versioned open-document changes; RFC-0030 defines
  atomic workspace reload and project-input observation.
- DEC-0019 and DEC-0071 define revision-aware invalidation and immutable
  snapshots; DEC-0034/DEC-0035 define deterministic diagnostic collection;
  DEC-0072 defines strict source-span projection.
- `PROTO-LSP-DIAGNOSTIC` and `PROTO-LSP-PUBLISH-DIAGNOSTICS` remain separate:
  the first maps immutable compiler values, while the second owns scheduling,
  freshness, replacement, clearance, and JSON-RPC delivery.

## Implemented boundary

- `ling-db::CompilerDb::workspace_diagnostics` executes deterministic
  lexical/parse precedence and complete workspace HIR, resolution, type, and
  Effect diagnostics without interpreting unchecked AST nodes.
- `ling-lsp::DiagnosticAnalysisTicket` owns exact request bytes and compiler
  inputs. Completion compares the full current snapshot and rejects stale
  results without losing pending work.
- `LspServer` marks only successful state changes pending, coalesces at an
  explicit flush boundary, publishes URI-sorted changed entries, associates
  open client versions, clears removed entries, and commits its ledger only
  after every notification is valid and within the transport bound.
- The stdio host writes a request response before any notification caused by
  that message and performs no timer-, path-, environment-, or network-based
  scheduling.

## Specification gaps encountered

The earlier audit correctly found that no Accepted authority defined push
publication. RFC-0032 supplies that missing bounded authority. The broader gap
remains open for pull diagnostics, cancellation requests, general background
scheduling, Workspace Edits, Semantic Transactions, and other editor
transactions; none is inferred into this implementation.

## Compatibility and determinism

Adapter 0.2 accepts every valid adapter 0.1 input with unchanged output and
only adds validated temporary identities. No compiler diagnostic code, core
diagnostic schema, Ling syntax or semantics, Typed Core, runtime, bytecode,
VM, ABI, or Unicode 17.0.0 table changes. Host paths, clocks, thread order,
allocation order, and debug output cannot affect published values.

## Intentionally deferred

Pull diagnostics and push/pull parity remain LSP-2203. Root-cause grouping,
deduplication, caps, and suppression remain LSP-2204. Cancellation requests,
partial results, progress, tags, code-description URLs, repair application,
Workspace Edits, Semantic Transactions, wall-clock debounce, worker pools, and
Stable compatibility require later Accepted authority.
