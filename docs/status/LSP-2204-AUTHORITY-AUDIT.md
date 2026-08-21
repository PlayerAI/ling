# LSP-2204 Authority Audit: Root-cause and error-storm control

## Outcome

`LSP-2204` is correctly recorded as `BlockedSpec`. The execution plan requests
root-cause prioritization and error-storm control for editor diagnostics. Ling
already defines root-cause-first ordering and bounded structured diagnostics at
the compiler boundary, but no accepted LSP policy defines suppression,
deduplication, per-snapshot caps, related-file grouping, or recovery.

No LSP diagnostic cap, suppression rule, deduplication key, truncation marker,
grouping field, or placeholder adapter was added. Existing compiler/CLI
diagnostics remain complete and unchanged.

## Normative traceability

- `docs/SEMANTICS.md` §26 requires root-cause-first diagnostics, stable codes,
  original spans, bounded Facts/repairs, and bilingual rendering; it does not
  define editor error-storm policy or lossy suppression.
- `docs/ERROR-CODES.md` defines the single registered code source and stable
  meanings; it does not authorize LSP-specific truncation or deduplication.
- `PROTO-DIAGNOSTIC-JSON` is a Preview diagnostic writer, not an LSP publication
  policy.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave snapshot/result lifecycle,
  versioning, and public editor fields open. LSP-2201 through LSP-2203 remain
  blocked on adapter and publication contracts.

## Current interface evidence

The current repository confirms the missing boundary:

- Compiler diagnostics are sorted and bounded by root cause/source order, but
  no LSP-specific maximum, aggregation, or suppression metadata exists.
- No code defines whether duplicate diagnostics are equal by code/span/facts,
  whether related-file errors are grouped, or how a later successful snapshot
  clears a suppressed error.
- No fixture covers pathological syntax/error cascades, caps, truncation,
  resumption, or deterministic suppression across process/hash seeds.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. root-cause and dependent-diagnostic ordering, deduplication identity, and
   grouping/related-file rules;
2. per-file/workspace/snapshot limits, truncation representation, severity
   policy, and whether suppressed facts remain queryable;
3. recovery and resumption when the source becomes valid, stale result and
   cancellation behavior, and interaction with push/pull diagnostics;
4. deterministic ordering, localization, negotiated positions, protocol field
   lifecycle, and compatibility/migration policy; and
5. positive, negative, cascade, cap/truncation, recovery, cross-file,
   Unicode/CRLF, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, suppressing or reshaping
diagnostics in an LSP adapter could hide root causes, produce unstable editor
behavior, or diverge from the registered compiler diagnostic contract.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-diagnostics`, and the LSP-2201/2202/2203 audit boundaries.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2204` can begin after the LSP diagnostic adapter/publication contracts and
error-storm decision are Accepted. The implementation must preserve compiler
root causes, make any truncation explicit and deterministic, and prove
recovery/clear behavior with fixtures.
