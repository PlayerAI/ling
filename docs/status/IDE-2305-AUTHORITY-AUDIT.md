# IDE-2305 Authority Audit: Prepare Rename

## Outcome

`IDE-2305` is implementation-ready and implemented under Accepted `RFC-0040`.
Its plan dependencies `IDE-2304` and `LSP-2102` are Done. The earlier
`BlockedSpec` audit over-scoped this read-only standard request with the
new-name and mutation contracts owned by IDE-2306.

## Normative traceability

- RFC-0040 fixes capability validation/discovery, immutable checked snapshot
  selection, writable source-backed eligibility, exact range and placeholder,
  null behavior, bounds, freshness, failures, and migration.
- RFC-0039 supplies the complete checked declaration/reference grouping and
  canonical selector rules; RFC-0040 extends it only so declarations without
  references remain selectable.
- DEC-0077 supplies Unicode 17.0.0 identifier facts without turning security
  observations into rename acceptance policy. DEC-0002 and DEC-0029 preserve
  exact original-byte spans and negotiated projection.
- DEC-0012 remains explicit that a later rename changes DefinitionId. Prepare
  Rename returns no edit or identity-preservation promise.

## Plan/repository drift resolved

LSP `textDocument/prepareRename` contains a document and position but no
`newName`. It cannot evaluate a future spelling, collision, visibility, or
coherence result. RFC-0040 keeps all such requirements for IDE-2306 and closes
IDE-2305 as the exact standard target/range query.

The open general transaction gap remains valid for actual edits and does not
block this bounded read-only response. The Unicode Alias gap does not authorize
Alias syntax and is not consumed by this implementation.

## Current implementation evidence

- `ling-db::ReferenceSearchIndex::selection_at` selects exact declarations or
  resolver references, including unreferenced declarations, without publishing
  target identity.
- `ling-lsp::prepare_rename` requires complete resolution/type/Effect checking,
  preserves temporary isolation, rejects stale completion, and returns either
  exact standard range-with-placeholder or `null`.
- Writable truth is captured in the immutable request document instead of
  being reconstructed from URI spelling.
- Executable tests cover capability shape, declarations/references/locals,
  unreferenced targets, builtins, dependencies, temporary documents,
  UTF-8/16/32, BOM/CRLF/Unicode, invalid requests, notifications, and checked
  failure.

## Compatibility and deferrals

No diagnostic allocation, language behavior, Semantic ID, source-span,
runtime, bytecode, VM, ABI, package, filesystem/network, cache, or Unicode
17.0.0 table changes. New-name policy, aliases, collisions,
visibility/coherence, edits, DefinitionId migration, cancellation, Workspace
Edits, Semantic Transactions, and Stable compatibility remain deferred to
IDE-2306 or later Accepted authority.
