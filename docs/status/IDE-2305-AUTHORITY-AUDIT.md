# IDE-2305 Authority Audit: Prepare Rename

## Outcome

`IDE-2305` is correctly recorded as `BlockedSpec`. The execution plan requires
an exact identifier range and placeholder, and rejection of keywords/builtins,
generated definitions, read-only dependencies, confusable or invalid names,
ambiguous snapshots, and visibility/coherence violations. There is no accepted
prepare-rename request/response, rename target policy, or version precondition
contract.

No prepare-rename handler, name validator, placeholder editor API, diagnostic
allocation, or protocol field was added.

## Normative traceability

- The execution package is non-normative; its rejection list does not authorize
  a mutation or editor protocol.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for any future LSP UTF-16 position. It does not
  define identifier-range requests, document versions, or stale responses.
- DEC-0012 defines `DefinitionId` from normalized module/name identity and
  explicitly states that renaming a definition changes its DefinitionId. The
  plan's identity-based/behavior-preserving rename therefore needs an accepted
  identity and migration policy rather than an implementation assumption.
- `docs/SEMANTICS.md` fixes Seed keyword rejection, Unicode XID/NFC handling,
  and the Seed boundary, but leaves alias syntax/display and several security
  presentation questions open. `GAP-UNICODE-ALIAS-SYNTAX-001` records the
  unresolved alias identity, collision, and confusable/rename consequences.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` explicitly blocks IDE-2305 and leaves
  snapshot/version preconditions and Stable versus Experimental edit fields
  open. `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves Semantic Graph/Transaction
  stability and migration open for the subsequent rename operation.
- RFC-0005 and DEC-0027 keep Trait witnesses internal and explicitly provide no
  CLI/LSP integration; prepare rename cannot invent Trait/coherence projections.

## Current interface evidence

- `ling-source` and the lexer preserve byte spans and implement Seed identifier
  normalization/validity boundaries, but expose no editor rename target or
  localized failure model.
- `ling-resolve` and `ling-semantic` retain internal definitions, references,
  and identities, but provide no prepare-rename service, exact identifier-range
  projection, or generated/dependency mutability policy.
- The VFS/revision implementation is internal; it does not define open-document
  version checks, ambiguity handling, stale results, or a public edit protocol.
- No executable fixture covers valid/invalid new names, keywords, Unicode
  normalization/confusables, aliases, generated/dependency targets, visibility
  or coherence failures, stale snapshots, or deterministic placeholders.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. target selection and exact identifier range, including local, module, type,
   constructor, import-alias, prelude/builtin, generated, and dependency cases;
2. new-name grammar and Unicode 17.0.0/XID/NFC/confusable policy, keyword
   handling, alias identity/collision rules, localized diagnostics, and stable
   error behavior;
3. visibility and coherence safety checks, unresolved/ambiguous target policy,
   source/dependency read-only rules, and deterministic placeholder fields;
4. snapshot identity, document versions, position encoding, cancellation,
   stale-result behavior, protocol field stability, Semantic ID consequences,
   and migration into the later Workspace Edit contract; and
5. executable positive and negative fixtures for all target/name classes,
   Unicode/CRLF/BOM positions, cross-package/generated cases, ambiguity,
   visibility/coherence, stale versions, deterministic output, and migration.

Until these decisions are Accepted, prepare rename could promise an identity
that DEC-0012 says will change, accept an alias/confusable collision, or return
an edit placeholder that cannot be safely applied to the current snapshot.

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

`IDE-2305` can begin after the LSP transaction, rename identity, Unicode alias,
and Semantic Graph lifecycle decisions are Accepted. The future implementation
must use checked resolution, preserve byte-span truth, make DefinitionId changes
explicit, reject unsafe/stale targets deterministically, and label experimental
fields.
