# IDE-2309 Authority Audit: Code Actions

## Outcome

`IDE-2309` is correctly recorded as `BlockedSpec`. The execution plan proposes
structured `FixPlan` actions for importing a missing symbol, renaming a
confusable, making a binding/field mutable, adding match cases, replacing stale
syntax, and applying the formatter. Each action is required to carry a kind,
diagnostic code, and snapshot/version precondition, without parsing diagnostic
message text. No accepted FixPlan or code-action mutation contract exists.

No code-action handler, FixPlan schema, diagnostic-to-action registry,
Workspace Edit, formatter action adapter, diagnostic allocation, or placeholder
editor surface was added.

The bounded child `IDE-2309-REPAIR-INDEX` only copies existing diagnostic
codes/spans and structured `Repair` payloads into an internal read-only index.
It does not derive action IDs, applicability, edits, or protocol responses.

## Normative traceability

- The execution package is non-normative; its action list does not authorize
  source mutation or a public quick-fix protocol.
- DEC-0001 fixes the bilingual stable diagnostic registry and structured
  diagnostic policy, but it does not define applicability, fix identity,
  action safety, or editor edit responses.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  action ranges, versions, overlap, or stale-edit handling.
- DEC-0012 fixes Semantic IDs/canonical bytes; a code action must not invent
  identity-preserving rewrites or serialize Experimental graph fields.
- DEC-0015 and DEC-0023 constrain Audit/Author Source and preservation-oriented
  formatting. `GAP-FORMATTER-AUTHOR-SOURCE-001` and
  `GAP-FORMATTER-CLI-PROTOCOL-001` leave broader formatter/editor integration
  and CLI/report behavior open.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` explicitly blocks IDE-2309 and leaves
  snapshot/version and Stable versus Experimental edit fields open.
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves Semantic Graph/Transaction
  stability and migration open. Alias/localization gaps affect confusable and
  insertion fixes. RFC-0005/DEC-0027 provide no public Trait action surface.

## Current interface evidence

- Parser, resolver, type/effect checks, and `ling-cli` produce structured
  diagnostics with registered codes, but there is no FixPlan/action kind,
  applicability predicate, or diagnostic-to-edit mapping.
- `ling-diagnostics::DiagnosticRepairIndex` provides deterministic structured
  repair facts without inspecting `message_zh` or `message_en`; it has no
  FixPlan, mutation, version, or action policy.
- `ling-source` and formatter code preserve source spans/revisions, yet no
  atomic multi-file Workspace Edit, snapshot/version precondition, overlap
  policy, rollback, or stale-action response is implemented.
- No accepted semantic service describes safe import insertion, confusable
  rename, mutability, match-case generation, stale-syntax replacement, or
  formatter application; no action fixture exists.
- There is no public code-action protocol entry in the inventory and no test
  that proves actions are derived from structured facts rather than diagnostic
  message text.

## Required authority before implementation

An Accepted decision or RFC must define, at minimum:

1. action kinds, stable IDs, diagnostic-code association, applicability,
   preferred/suppressed behavior, capability requirements, and deterministic
   ordering;
2. structured FixPlan payloads and multi-file edit schema: source ranges,
   URI/path normalization, UTF-8/UTF-16 conversion, overlap/ordering,
   atomicity, rollback/conflict behavior, open-document versions, cancellation,
   limits, and stale-result rejection;
3. semantic safety for missing imports, confusable rename, mutable binding or
   field, exhaustive match cases, stale syntax replacement, and formatter use,
   including name-resolution, visibility, effects/capabilities, coherence,
   generated/dependency read-only, comments, and localized source rules;
4. diagnostic registry integration, bilingual/localized rendering, protocol
   inventory, Semantic ID/provenance, Stable versus Experimental fields, and
   migration; and
5. executable positive/negative fixtures for every action, multi-file and
   overlapping edits, Unicode/CRLF/BOM, stale versions, dependency/generated
   targets, formatter preservation, deterministic ordering, rollback, and
   message-text independence.

Until these decisions are Accepted, a code action could apply a stale or
mispositioned edit, create an invalid import/match/mutability change, rewrite
Author Source unexpectedly, or turn diagnostic wording into an unstable API.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0001, DEC-0002, DEC-0012, DEC-0015, DEC-0023,
RFC-0005, `docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the parser, resolver, type/effect, source, formatter, and CLI crates.

No compiler, interpreter, VM, bytecode, diagnostic code, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`IDE-2309` can begin after FixPlan/action, LSP transaction, diagnostic adapter,
formatter, and Semantic Graph lifecycle decisions are Accepted. The future
implementation must consume structured checked facts, preserve source-span and
identity truth, apply edits atomically against explicit versions, avoid parsing
message text, and label experimental fields.
