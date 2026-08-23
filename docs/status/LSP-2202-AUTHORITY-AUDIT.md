# LSP-2202 Authority Audit: Push diagnostics v0

## Outcome

`LSP-2202` remains `BlockedSpec`, but its foundational dependencies are now
implemented: RFC-0004/RFC-0023/RFC-0029/RFC-0030 govern lifecycle, overlays,
incremental edits, and workspace snapshots, while Accepted RFC-0031 defines
the compiler diagnostic adapter. The remaining blocker is narrower and
substantive: no Accepted authority defines push publication, trigger/debounce,
document-version association, stale-result rejection, replacement, or clear
semantics.

Accepted DEC-0035 closes only the bounded `LSP-2202-BATCH` child: an immutable
internal batch over opaque diagnostic IDs and DEC-0034 order keys. It is not a
publication contract and cannot be inferred into one.

## Normative traceability

- `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`, and RFC-0031 define deterministic
  bilingual compiler diagnostics and their exact LSP value projection.
- RFC-0004 defines lifecycle/transport and negotiated position encoding;
  RFC-0023/RFC-0029 define versioned open-document changes; RFC-0030 defines
  atomic workspace input snapshots.
- DEC-0019 and DEC-0071 define revision-aware invalidation and immutable
  observation, but not asynchronous publication ownership.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` still leaves event scheduling,
  snapshot/client-version association, stale completion, and publication
  replacement behavior open.
- `PROTO-DIAGNOSTIC-JSON` is a separate Preview compiler writer;
  `PROTO-LSP-DIAGNOSTIC` is a pure adapter, not `publishDiagnostics`.

## Current interface evidence

- `ling-lsp::adapt_diagnostics` can build exact ordered LSP diagnostic values
  from immutable source inputs, but it accepts no request snapshot or client
  version and sends no JSON-RPC notification.
- `LspServer` tracks lifecycle, overlays, versions, and workspace revisions,
  but has no compile scheduler, debounce state, published-result identity, or
  replacement/clear ledger.
- `ling-db` exposes revisioned queries and cancellation primitives internally;
  no Accepted rule chooses syntax-only versus workspace analysis timing or
  defines which completed result may still be published.
- The DEC-0035 batch remains disconnected from transport and cannot establish
  document-version, stale-result, or empty-clear behavior.

## Required authority before implementation

An implementation-ready RFC must define, at minimum:

1. exact `didOpen`/`didChange`/workspace-reload triggers, deterministic debounce
   and cancellation checkpoints, and bounded work/resource policy;
2. immutable request/workspace snapshot identity, optional LSP document
   version mapping, and stale completion rejection;
3. `publishDiagnostics` URI/version/diagnostics shape, ordering, related-file
   scope, replacement and empty-clear semantics, and failure atomicity;
4. syntax-fast-path versus workspace semantic result precedence without
   allowing older/narrower results to overwrite newer/complete results; and
5. positive, negative, edit-burst, cancellation, stale, clear/replace,
   multi-file, Unicode/CRLF, deterministic, and migration fixtures.

## Compatibility and determinism

The future publisher must reuse RFC-0031 values unchanged, expose no host
paths or scheduler timing as Ling semantics, and prevent cancelled or stale
work from clearing a newer result. Any public extension field requires an
explicit Experimental version marker and migration evidence. No diagnostic
code or core diagnostic schema should be allocated for transport bookkeeping.

## Intentionally deferred

Until the publication RFC is Accepted, no `publishDiagnostics` handler,
background compiler job, timer/debounce mechanism, version tag, result ledger,
or placeholder public API may be added. Root-cause/error-storm caps and
suppression remain LSP-2204; pull diagnostics remain LSP-2203.
