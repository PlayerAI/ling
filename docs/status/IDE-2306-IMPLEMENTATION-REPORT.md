# IDE-2306 implementation report

> Status: Done (2026-08-23)
> Task: `IDE-2306`
> Implementation commit: `ecb6545fec5fa1f457ee9abf69c7354306ea1bb0`
> Authority: Accepted `RFC-0041`, `RFC-0040`, `RFC-0039`, `RFC-0038`,
> `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`,
> `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0077`,
> `DEC-0078`
> Protocol: `ling.lsp.rename/0.1` (Preview)

## Delivered vertical slice

- `ling-db` exposes a bounded immutable checked import-alias occurrence index
  and a borrowed checked reference target used only for identity simulation.
- `ling-lsp` negotiates standard transactional versioned Workspace Edit
  support, advertises exact Preview discovery, and handles standard
  `textDocument/rename` only in Ready state.
- Eligible writable definitions, bindings, and explicit resolved import aliases
  are renamed across their complete checked occurrence set. Read-only,
  source-less, unresolved, incomplete, or unsupported targets return `null`.
- Every candidate name passes Unicode 17.0.0 XID/NFC/security policy and a
  fresh whole-snapshot resolution/type/Effect simulation with exact identity
  migration and relation-topology checks before deterministic publication.

## Specification drift resolved

Accepted RFC-0041 chooses a bounded standard versioned `documentChanges`
response, gated by the client's exact `transactional` failure capability. It
does not invent server-side mutation or a Semantic Transaction. Definition
rename intentionally changes DefinitionId under DEC-0012; bindings and import
targets retain their existing resolver identity. Language Alias syntax and
localized Author Source remain outside this implementation.

## Tests and verification

- `cargo test -p ling-db rename_alias_index --locked --offline` passes.
- `cargo test -p ling-lsp --test rename --locked --offline` passes.
- All-target `ling-db` and `ling-lsp` tests and strict Clippy pass.
- Full locked offline workspace tests, strict Clippy, CI, governance, LSP,
  support, status, RC0, v0.0.1 traceability, formatting, and diff gates pass
  against implementation commit `ecb6545fec5fa1f457ee9abf69c7354306ea1bb0`.

## Compatibility, determinism, and Unicode impact

- Adds Preview `ling.lsp.rename/0.1` with no predecessor and no unversioned
  fallback.
- Adds no diagnostic code, Ling syntax/semantics, Typed Core evaluation,
  Semantic ID schema, runtime, bytecode, VM, ABI, package, or filesystem/network
  mutation behavior.
- Original UTF-8 spans remain authoritative. URI/document/edit ordering is
  deterministic, open versions are explicit, closed versions are `null`, and
  output uses only negotiated position projection. Unicode remains 17.0.0.

## Intentionally deferred

General Semantic Transactions, Stable edit lifecycle, language Alias syntax,
localized Author Source equivalence, generated or virtual mutation,
dependency mutation, module/file rename, type-only identities, cancellation,
progress, and change annotations remain deferred.
