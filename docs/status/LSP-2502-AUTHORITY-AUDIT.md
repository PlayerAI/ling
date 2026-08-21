# LSP-2502 Authority Audit: Request Cancellation

## Outcome

`LSP-2502` is correctly recorded as `BlockedSpec`. The execution plan requires
`$/cancelRequest`, periodic cancellation checks in solver/index/rename/
completion, suppression of partial Workspace Edits, and a compiler query that
can return `Cancelled` without caching a partial result. The repository has
only separate internal query and VM cancellation boundaries; no accepted
compiler-facing or LSP cancellation protocol defines request IDs, propagation,
publication, or observable failure behavior.

No LSP cancellation handler, request token, compiler cancellation API,
partial-result suppression mechanism, diagnostic allocation, protocol schema,
or placeholder server was added.

## Normative traceability

- The execution package is non-normative; its `$/cancelRequest` and checkpoint
  bullets do not authorize a JSON-RPC method or compiler-facing API.
- Accepted DEC-0019 permits only an internal cooperative query cancellation
  point. It may stop work before publishing a new query result, must not
  publish partial checked results, and explicitly says compiler-facing
  cancellation and LSP request cancellation require separate decisions.
- Accepted RFC-0020 defines cancellation for the VM host-control API and the
  `execution.cancelled` Runtime Fault. It is not a compiler or LSP request
  protocol and cannot be generalized silently.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version, request, and
  editor mutation fields open, while `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`
  leaves public result lifecycle, stale handling, and migration open.
- LSP-2501 request snapshots are `BlockedSpec`; without an accepted snapshot
  identity, cancellation cannot prove which revision is being suppressed or
  published.

## Current interface evidence

- The internal query boundary has no public cancellation token, JSON-RPC
  request ID, compiler-service cancellation result, or LSP transport adapter.
- The VM has a distinct host cancellation token and Runtime Fault projection;
  reusing it for compiler/LSP work would conflate runtime effects with
  analysis cancellation and publish an unsupported compatibility promise.
- No solver/index/rename/completion implementation, Workspace Edit publisher,
  partial-result cache, or request scheduler exists in the current repository.
- No fixture covers cancellation before start, at query checkpoints, during
  dependency work, racing with completion, after a result is ready, unknown or
  reused request IDs, stale snapshots, partial edits, cache publication, or
  deterministic cancellation outcomes.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the JSON-RPC cancellation request/response method, request-ID type and
   lifetime, unknown/duplicate/late cancellation behavior, and capability
   negotiation;
2. propagation from transport to the immutable request snapshot, compiler
   queries, solver/index/rename/completion, and all checkpoints, including
   cleanup and the guarantee that cancellation cannot observe or publish
   partially checked data;
3. publication precedence among cancelled, completed, failed, stale, limited,
   and superseded requests; suppression/rollback of partial Workspace Edits,
   diagnostics, semantic tokens, cache entries, and progress notifications;
4. interaction with document versions, stale-result rejection, request
   deadlines, resource limits, fairness, host effects, and deterministic
   behavior independent of wall-clock timing; and
5. protocol inventory/versioning, Stable versus Experimental fields, bilingual
   diagnostic/error mapping, and executable positive/negative/race/migration
   fixtures for pre-start, mid-query, dependency, Unicode/CRLF/BOM, stale,
   duplicate/unknown IDs, late cancellation, result-ready races, no-partial-
   publication, and deterministic cleanup.

Until these decisions are Accepted, cancellation could leave partial checked
state visible, suppress the wrong document revision, or expose VM/runtime
cancellation details as an LSP compatibility contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0019, RFC-0020,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-vm`, `crates/ling-db`, `crates/ling-source`, and the current
compiler/VM tests.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2502` can begin after LSP-2501 snapshot/version, compiler-facing
cancellation, and LSP/Semantic Transaction lifecycle decisions are Accepted.
The future implementation must propagate cancellation cooperatively, publish
no partial checked result or Workspace Edit, keep VM cancellation separate, and
make all result-state transitions explicit and deterministic.
