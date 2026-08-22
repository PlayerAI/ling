# LSP-2502 Authority Audit: Request Cancellation

## Outcome

`LSP-2502` remains correctly recorded as `BlockedSpec` for public LSP and
compiler cancellation. Accepted DEC-0031 authorizes and child
`LSP-2502-CANCELLATION` implements only a clone-shared, monotonic in-process
checkpoint token. The public `$/cancelRequest` method, request-ID lifecycle,
compiler propagation, and partial-result publication rules remain undefined.

No JSON-RPC cancellation handler, compiler query cancellation API, partial
result suppression mechanism, diagnostic allocation, protocol schema, or
placeholder scheduler was added.

## Normative traceability

- The execution package is non-normative; its `$/cancelRequest` and checkpoint
  bullets do not authorize a JSON-RPC method or compiler-facing API.
- Accepted DEC-0019 permits an internal cooperative query cancellation point
  and forbids publishing partial checked results, while explicitly leaving
  compiler-facing and LSP request cancellation to separate authority.
- Accepted DEC-0031 defines only the internal `ling-lsp::CancellationToken`
  and typed checkpoint error; it carries no request ID, document version,
  deadline, snapshot identity, or wire response.
- Accepted RFC-0020 defines cancellation for VM host control and the
  `execution.cancelled` Runtime Fault. It is deliberately not reused for
  compiler/LSP analysis.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` still leave public result lifecycle,
  stale handling, and migration open.

## Current interface evidence

- `ling-lsp` now has an internal clone-shared token whose cancellation is
  monotonic, idempotent, non-blocking, and independent of wall-clock timing.
- The focused child tests cover pre-cancel success, clone propagation,
  repeated cancellation, typed checkpoint errors, and independent tokens.
- No solver/index/rename/completion implementation, Workspace Edit publisher,
  request scheduler, compiler cancellation result, or public cancellation
  protocol exists in the repository.
- No fixture covers public request IDs, snapshot association, stale/limited
  results, partial edits/diagnostics/tokens, deadlines, fairness, unknown or
  late cancellation, or cache publication.

## Required authority before parent implementation

An Accepted RFC or decision must still define:

1. the JSON-RPC cancellation method, request-ID type and lifetime,
   unknown/duplicate/late behavior, and capability negotiation;
2. propagation from transport to request snapshots, compiler queries,
   solver/index/rename/completion, and all checkpoints;
3. publication precedence among cancelled, completed, failed, stale, limited,
   and superseded requests, including suppression of partial artifacts; and
4. interaction with document versions, deadlines, resource limits, fairness,
   deterministic behavior, diagnostics, migration, and executable race tests.

Until those decisions are Accepted, cancellation could suppress the wrong
revision or expose VM/runtime cancellation details as an LSP contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0019, DEC-0031, RFC-0020,
`ling-lsp`, `ling-db`, `ling-vm`, the execution plan, and governance records.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed. The internal token
is not a wire protocol and is absent from the protocol inventory/support
matrix.

## Intentionally deferred

The `LSP-2502-CANCELLATION` child is complete under DEC-0031. The parent
LSP-2502 remains blocked until request snapshots, compiler-facing propagation,
public JSON-RPC cancellation, result precedence, and LSP/Semantic Transaction
lifecycle rules are Accepted. Future work must keep VM cancellation separate
and publish no partial checked result or Workspace Edit.
