# IDE-2305 implementation report

> Status: Implemented; status-ledger binding pending the implementation commit
> Task: `IDE-2305`
> Authority: Accepted `RFC-0040`, `RFC-0039`, `RFC-0038`, `RFC-0004`,
> `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0012`,
> `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0077`, `DEC-0078`
> Protocol: `ling.lsp.prepare-rename/0.1` (Preview)

## Delivered vertical slice

- `ling-db` checked reference grouping now includes source-backed definitions
  and bindings without references and exposes a borrowed exact selection view.
- `ling-lsp` validates the standard rename client capability, advertises a
  static prepare provider plus exact Preview discovery, and handles
  `textDocument/prepareRename` only in Ready state.
- Eligible writable source-backed declaration/reference selections return the
  exact negotiated range and original selected spelling. Non-target,
  source-less builtin/Prelude, and read-only dependency selections return
  `null`.
- The handler uses one complete checked immutable snapshot, verifies selected
  and declared NFC identity through the existing Unicode 17.0.0 observation,
  and rechecks freshness before every success.

## Specification drift resolved

Standard `prepareRename` has no `newName`. RFC-0040 therefore assigns
new-name legality, confusable/collision checks, visibility/coherence
simulation, edits, and DefinitionId migration to IDE-2306, where that input and
mutation actually exist. IDE-2305 makes no false validation or identity claim.

## Tests and verification

- `cargo test -p ling-db reference_search_index --locked --offline` passes.
- `cargo test -p ling-lsp --test prepare_rename --locked --offline` passes.
- Focused formatting and compilation pass.
- Exact transcript, workspace, governance, release, and strict Clippy gates are
  recorded only after the generated evidence and implementation commit are
  complete.

## Compatibility, determinism, and Unicode impact

- Adds Preview `ling.lsp.prepare-rename/0.1` with no predecessor.
- Adds no diagnostic code, Ling syntax/semantics, Typed Core evaluation,
  Semantic/resolver ID schema, runtime, bytecode, VM, ABI, package, or
  filesystem/network behavior.
- Original UTF-8 spans remain authoritative; output uses only the negotiated
  shared projection. Ordering and selection use existing deterministic checked
  index rules. Unicode tables remain 17.0.0.

## Intentionally deferred

New-name validation, aliases, confusable/collision policy,
visibility/coherence simulation, generated/virtual documents, multi-file
edits, stale edit application, DefinitionId migration, asynchronous
cancellation, Workspace Edits, Semantic Transactions, and Stable lifecycle
remain IDE-2306 or later work.
