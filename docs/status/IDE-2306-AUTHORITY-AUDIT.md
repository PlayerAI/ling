# IDE-2306 Authority Audit: Rename

## Outcome

`IDE-2306` is correctly recorded as `BlockedSpec`. The execution plan requires
identity-based rename, Unicode/NFC/confusable validation, reference and import
alias collection, temporary-snapshot checking, behavior/identity preservation,
versioned Workspace Edits, and stale-open-document rejection. None of those
public mutation and compatibility contracts is Accepted.

No rename handler, Workspace Edit schema, temporary-snapshot mutation API,
name validator, diagnostic allocation, or placeholder editor surface was added.
The bounded child `IDE-2306-REFERENCE-SPANS` only projects resolver-backed HIR
identifier spans into an internal read-only value; it does not choose targets
or apply edits.

## Normative traceability

- The execution package is non-normative; its seven-step rename flow cannot
  authorize a public mutation protocol.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  edit ordering, document versions, or stale-edit responses.
- DEC-0012 defines `DefinitionId` from normalized module/name identity and says
  that renaming a definition changes its DefinitionId. The plan's
  identity/behavior-preservation gate therefore needs an accepted migration and
  identity policy; it cannot silently promise stable IDs.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` explicitly blocks IDE-2306 and leaves
  snapshot/version preconditions and Stable versus Experimental Workspace Edit
  fields open. `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves Semantic
  Graph/Transaction stability, stale rejection, and schema migration open.
- `GAP-UNICODE-ALIAS-SYNTAX-001` leaves alias identity, collision, localized
  display, and rename consequences open. `GAP-AUTHOR-SOURCE-LOCALIZATION-001`
  leaves localized source equivalence and migration open.
- RFC-0005 and DEC-0027 keep Trait witnesses internal and explicitly provide no
  CLI/LSP integration; a rename operation cannot invent Trait/coherence edits.

## Current interface evidence

- `ling-resolve` and `ling-semantic` provide internal identity and reference
  data, but no complete reference/import-alias index, rename target service, or
  public identity migration model.
- `ling-db::resolved_reference_span_index` now pairs resolver reference IDs
  with exact original UTF-8 Name, projection-field, and mutable-place-root
  spans. It omits HIR IDs absent from resolver identity and has no editor range,
  edit, version, or mutation policy.
- `ling-source` has session-local VFS revisions and source maps, but no atomic
  multi-document edit, open-document version precondition, conflict response,
  or rollback contract.
- `ling-project` discovers source/dependency files, but no accepted policy
  authorizes edits to dependency or generated documents.
- The protocol inventory has no accepted Workspace Edit or Semantic Transaction
  schema, and no executable fixture covers rename across modules/packages,
  aliases, Unicode/security cases, comments/formatting, or stale versions.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. target identity and exact source ranges, including definitions, references,
   import aliases, constructors, local bindings, generated/builtin/primitive
   targets, and dependency read-only behavior;
2. new-name grammar, Unicode 17.0.0/XID/NFC/confusable rules, keyword and alias
   handling, localized diagnostics, and collision/error stability;
3. complete multi-file edit semantics: URI/path normalization, position
   encoding, ordering/overlap rules, open-document versions, atomicity,
   rollback/conflict behavior, and generated/source preservation;
4. temporary-snapshot application and re-check semantics for resolution,
   types, effects, capabilities, visibility, coherence, Semantic IDs, and
   behavior, including the deliberate DefinitionId migration consequence;
5. Workspace Edit versus Semantic Transaction versioning, Stable/Experimental
   fields, cancellation, resource limits, stale-result rejection, and protocol
   migration; and
6. executable positive, negative, cross-package, alias, Unicode/CRLF/BOM,
   comments/formatting, overlapping-edit, ambiguity, visibility/coherence,
   stale-version, deterministic, rollback, and migration fixtures.

Until these decisions are Accepted, rename could apply stale or mispositioned
edits, alter Semantic IDs without an explicit migration, or mutate read-only
dependency/generated content.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, RFC-0005,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-source`, `ling-resolve`, `ling-semantic`, and `ling-project`
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2306` can begin after the LSP transaction, Semantic Graph lifecycle,
Unicode alias/localization, and identity-migration decisions are Accepted. The
future implementation must use checked resolution and references, preserve
source-span truth, apply edits atomically against explicit versions, make
DefinitionId changes visible, and label experimental fields.
