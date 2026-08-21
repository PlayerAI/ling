# LSP-2202 Authority Audit: Push diagnostics v0

## Outcome

`LSP-2202` is correctly recorded as `BlockedSpec`. The execution plan proposes
`publishDiagnostics` after open-file edits, debounce, fast syntax results,
workspace type results, document-version tagging, and replacement of older
results. These behaviors depend on unaccepted LSP lifecycle, overlay/version,
position, and diagnostic-adapter contracts.

No push-diagnostics publisher, debounce scheduler, version tag, clear-result
policy, or placeholder LSP handler was added. Existing compiler/CLI diagnostic
behavior remains unchanged.

## Normative traceability

- `docs/SEMANTICS.md` and `docs/ERROR-CODES.md` define deterministic,
  root-cause-first, bilingual diagnostic facts and byte spans, but not an LSP
  publication lifecycle or incremental-result policy.
- `PROTO-DIAGNOSTIC-JSON` is a Preview writer and does not define
  `publishDiagnostics`, document versions, or stale-result replacement.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves document/snapshot/version and
  Workspace Edit behavior open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves
  public semantic lifecycle fields open.
- LSP-2101 through LSP-2104 and LSP-2201 remain blocked on lifecycle, position,
  overlay, change, and adapter authorities. A lower-level plan cannot publish
  diagnostics without those dependencies.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-diagnostics` produces deterministic human/JSON diagnostics from checked
  compiler results; no `publishDiagnostics` event or document-version field is
  emitted.
- `ling-db` exposes query revisions and traces internally, but no LSP debounce,
  cancellation, result replacement, or snapshot association exists.
- No fixture defines syntax-only versus workspace-type timing, stale result
  suppression, empty-diagnostics clearing, or related-document publication.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. trigger and debounce policy for `didOpen`/`didChange`, syntax versus
   workspace diagnostics, scheduling/cancellation, and resource limits;
2. document/project snapshot and version association, stale-result rejection,
   replacement/clear semantics, ordering, and related-document scope;
3. diagnostic adapter field mappings, negotiated positions, stable/experimental
   data, localization, root-cause/error-storm rules, and URI identity;
4. transport/lifecycle behavior, failure responses, offline/project policy,
   and Stable versus Experimental protocol fields; and
5. positive, negative, edit-burst, stale-version, cancellation, clear/replace,
   related-file, Unicode/CRLF, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, pushing results could publish
stale diagnostics, clear a newer snapshot, or freeze an unsupported debounce
and ordering contract.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-diagnostics`, `crates/ling-db`, and the LSP-2101/2104/2201 audit
boundaries.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2202` can begin after lifecycle, overlay/version, position, and diagnostic
adapter decisions are Accepted. The implementation must publish only
snapshot-associated results, replace/clear atomically, and keep debounce and
timing choices out of Ling semantics.
