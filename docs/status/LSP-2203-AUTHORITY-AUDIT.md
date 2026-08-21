# LSP-2203 Authority Audit: Pull diagnostics Preview

## Outcome

`LSP-2203` is correctly recorded as `BlockedSpec`. The execution plan proposes
an LSP pull-diagnostics Preview in addition to push diagnostics. No accepted
request/result schema defines `resultId`, unchanged reports, workspace
diagnostics, partial results, snapshot pinning, or lifecycle interaction.

No pull-diagnostics method, result-id cache, workspace report, partial-result
stream, or placeholder protocol field was added. Existing diagnostic writers
and CLI behavior remain unchanged.

## Normative traceability

- `docs/SEMANTICS.md` and `docs/ERROR-CODES.md` define diagnostic meaning,
  ordering, spans, localization, and repairs, not LSP pull request/result
  schemas or cache identity.
- `PROTO-DIAGNOSTIC-JSON` is a Preview writer and cannot be treated as an LSP
  pull response. No LSP protocol entry is inventoried.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version and request/edit
  fields open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves public semantic
  result lifecycle open.
- LSP-2101 through LSP-2202 remain blocked on transport, position, overlay,
  change, and diagnostic adapter contracts; pull results cannot bypass those
  dependencies.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-diagnostics` and `ling-db` can produce checked diagnostics and semantic
  snapshots internally, but no LSP request handler or result-id cache exists.
- No code defines unchanged-result semantics, workspace-wide report limits,
  partial result tokens, cancellation, or association with a document/project
  revision.
- The protocol inventory contains no LSP pull-diagnostics version or fixture;
  adding one would publish an unregistered editor API.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. document and workspace pull request/response schemas, result IDs, unchanged
   reports, partial result/progress tokens, and limits;
2. snapshot/document version pinning, stale-result behavior, cancellation,
   cache invalidation, and push/pull interaction;
3. diagnostic field mapping, position encoding, related documents, severity,
   tags, experimental data, localization, and error responses;
4. lifecycle/capability negotiation, project/offline policy, and Stable versus
   Experimental protocol/version migration; and
5. positive, negative, unchanged, stale, cancellation, workspace-limit,
   deterministic, Unicode/CRLF, and migration fixtures.

Until those decisions and fixtures are Accepted, a pull endpoint could return
  diagnostics for the wrong revision or freeze an unsupported result-id/cache
  contract.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-diagnostics`, `crates/ling-db`, and the LSP-2201/2202 audit
boundaries.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2203` can begin after LSP lifecycle, snapshot/version, diagnostic adapter,
and pull protocol decisions are Accepted. The implementation must pin results
to immutable revisions, support deterministic unchanged handling, and keep
Preview fields out of Stable contracts.
