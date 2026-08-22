# LSP-2103-OVERLAY Implementation Report

## Status and scope

**Status: Done (bounded full-text Preview slice).**

This child task implements the document-overlay boundary authorized by Accepted
RFC-0023. The parent `LSP-2103` remains `BlockedSpec` for incremental ranges,
compiler query snapshots, diagnostics, Workspace Edits, and Semantic
Transactions.

## Normative clauses covered

- RFC-0023 §§1–3: the `ling.lsp.overlay/0.1` marker, request/notification
  methods, and restricted path-free URI forms.
- RFC-0023 §§4–7: full-text open/change/close, monotonic versions, VFS
  precedence, read-only dependency behavior, disk publication, and temporary
  document removal.
- RFC-0023 §§8–9: bilingual protocol errors, no diagnostic allocation, exact
  UTF-8 preservation, and no leakage of SourceId/host paths/revisions.
- RFC-0004, DEC-0019, DEC-0002, and DEC-0029: lifecycle gating, immutable VFS
  snapshots, source-byte authority, and deferred position/range conversion.

## Implemented slice

- `LspServer` owns a session-local `VirtualFileSystem`, URI/document records,
  monotonic version history, deterministic document views, and a host-facing
  disk snapshot publication method.
- `textDocument/didOpen`, `didChange`, and `didClose` accept both valid
  notifications and request-form conformance probes. Successful requests
  return JSON `null`; notifications remain response-free.
- Workspace, dependency, and untitled URI classes are validated without host
  path resolution. Dependency documents can be opened but cannot be changed.
- Only one full-text replacement is accepted for `didChange`; ranged edits,
  stale versions, duplicate opens, closed documents, invalid URIs, oversized
  text, and invalid state are rejected before VFS mutation.
- Disk updates remain hidden behind open overlays; closing reveals the latest
  disk layer, and closing an untitled document removes its temporary VFS file.
- `ling-source` exposes the shared logical-name validator and a non-reusing
  temporary-file removal operation.

## Evidence

- `cargo test -p ling-lsp --locked --offline`: 3 unit tests, 6 lifecycle tests,
  and 4 overlay tests passed.
- Overlay tests cover workspace disk/overlay races, dependency read-only
  rejection, stale versions, temporary-file removal, URI rejection, ranged
  change rejection, response suppression, and deterministic document views.
- Full workspace, governance, status, CI-contract, formatting, Clippy, and diff
  gates are run for the milestone commit before completion evidence is filled.

## Compatibility and deferrals

No language syntax, compiler semantics, diagnostics registry, Semantic Graph,
Semantic IDs, bytecode, VM, CLI command set, package identity, or Unicode
17.0.0 data changed. `ling.lsp.overlay/0.1` is Experimental and current-writer
only. Incremental range edits, file URI/root mapping, compiler snapshots,
stale-result handling, diagnostics, Workspace Edits, cancellation, and
Semantic Transactions remain parent-task work.
