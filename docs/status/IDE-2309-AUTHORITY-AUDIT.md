# IDE-2309 Authority Audit: Code Actions

## Outcome

`IDE-2309` was correctly recorded as `BlockedSpec` before RFC-0044. The execution plan proposes
structured `FixPlan` actions for importing a missing symbol, renaming a
confusable, making a binding/field mutable, adding match cases, replacing stale
syntax, and applying the formatter. Each action is required to carry a kind,
diagnostic code, and snapshot/version precondition, without parsing diagnostic
message text. No accepted FixPlan or code-action mutation contract existed at
the time of the original audit.

Accepted RFC-0044 now authorizes one bounded Seed action:
`source.fixAll.ling.format`, derived solely from the accepted compiler-CST
formatter and returned as one versioned transactional Workspace Edit. It does
not authorize diagnostic-to-edit conversion or the other speculative actions.

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
- `GAP-LSP-TRANSACTION-PROTOCOL-001` originally blocked IDE-2309. RFC-0044 now
  closes only the bounded formatter action by composing the accepted snapshot,
  version, and transactional Workspace Edit rules; the general gap remains.
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves Semantic Graph/Transaction
  stability and migration open. Alias/localization gaps affect confusable and
  insertion fixes. RFC-0005/DEC-0027 provide no public Trait action surface.

## Current interface evidence

- Parser, resolver, type/effect checks, and `ling-cli` produce structured
  diagnostics with registered codes, but there remains no diagnostic-to-edit
  mapping. RFC-0044 defines one independent formatter plan and action kind.
- `ling-diagnostics::DiagnosticRepairIndex` provides deterministic structured
  repair facts without inspecting `message_zh` or `message_en`; it has no
  FixPlan, mutation, version, or action policy.
- `ling-source` and formatter code preserve source spans/revisions. RFC-0044
  composes them into one versioned single-document edit with complete freshness;
  multi-file overlap, rollback, and general transaction policy remain absent.
- No accepted semantic service describes safe import insertion, confusable
  rename, mutability, match-case generation, or stale-syntax replacement.
- `PROTO-LSP-CODE-ACTION` and the RFC-0044 integration fixtures now prove the
  formatter action derives only from `FormatEdit`, never diagnostic text/data.

## Authority supplied by RFC-0044

RFC-0044 defines the implemented formatter action's:

1. exact action kind, preferred state, capability requirements, filtering, and
   one-action cardinality;
2. structured single-document formatter plan, original-byte range, URI,
   version, transactional versioned Workspace Edit, freshness, and limits;
3. formatter safety through RFC-0026 and DEC-0057, with diagnostics opaque and
   dependencies/generated documents read-only;
4. bilingual title/failures, protocol lifecycle/inventory, and the absence of
   diagnostic or Semantic ID changes; and
5. executable positive, negative, Unicode/CRLF/BOM, version, determinism,
   read-only, filtering, and message/data-independence fixtures.

The other proposed actions remain unavailable until separate Accepted checked
edit producers define their semantic safety and fixtures.

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

RFC-0044 does not authorize missing-import, confusable-rename, mutability,
match-case, stale-syntax, diagnostic quick-fix, multi-document, resolve,
command, generated/dependency mutation, general Semantic Transaction, or Stable
lifecycle behavior. Those remain deferred rather than represented by placeholder
actions.
