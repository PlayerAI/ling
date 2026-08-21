# LSP-2504 Authority Audit: Memory and Resource Limits

## Outcome

`LSP-2504` is correctly recorded as `BlockedSpec`. The execution plan names
limits for open-document bytes, pending requests, completion results,
diagnostic count, and solver work, and requires a stable tool diagnostic
instead of OOM. No accepted LSP contract defines resource units, accounting
scope, limit precedence, failure response, retry behavior, or compatibility.

No LSP limit constants, quota manager, tool diagnostic, request rejection
response, protocol schema, diagnostic allocation, or placeholder server was
added.

## Normative traceability

- The execution package is non-normative; its resource list does not authorize
  public quotas, defaults, diagnostics, or host-memory behavior.
- Accepted RFC-0002 resource limits apply to project manifests and dependency
  graphs. Accepted bytecode limits apply to bytecode envelopes/tables. Their
  domain-specific diagnostics and units cannot be reused as LSP quotas.
- DEC-0019 bounds internal query inputs and requires oversized or malformed
  snapshots to fail without publishing partial checked results, but it does
  not define an LSP resource protocol or tool diagnostic.
- `PROTO-DIAGNOSTIC-JSON` is a Preview compiler/CLI diagnostic writer, not an
  LSP resource-limit response. The single diagnostic registry permits only
  registered `L-<DOMAIN>-<NUMBER>` meanings.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave request, result, lifecycle,
  stability, and migration fields open. LSP-2501 through LSP-2503 remain
  `BlockedSpec`, so quota enforcement cannot safely classify or publish their
  outcomes.

## Current interface evidence

- `ling-project`, `ling-bytecode`, and parser boundaries have explicit local
  limits and registered diagnostics for their own inputs; none governs open
  editor bytes, pending LSP work, completion lists, diagnostic reports, or
  solver steps.
- No LSP transport, request scheduler, snapshot association, queue accounting,
  result-size policy, or process-memory guard exists. Host allocation failure,
  cancellation, stale revision, and semantic errors have no LSP precedence.
- No tool diagnostic code, bilingual message/facts schema, capability flag,
  configuration version, retry/backoff rule, or client-visible limit response
  is inventoried for LSP.
- No fixture covers exact-boundary/over-limit cases, aggregate versus
  per-request accounting, concurrent requests, nested dependencies, partial
  result suppression, Unicode/CRLF/BOM bytes, cancellation races, or
  deterministic behavior without OOM.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. each resource's unit and accounting scope: bytes versus scalar/code units,
   per document/request/workspace/process, pending work versus retained
   results, solver steps, and dependency/generated-file inclusion;
2. default, minimum, maximum, configuration, capability negotiation, and
   versioning rules, including whether limits are hard, soft, or adaptive and
   how changes affect existing snapshots and queues;
3. failure precedence and response schemas for limit exceeded, cancellation,
   stale, invalid, internal, and host-memory failures, including bilingual
   stable tool diagnostics, facts, spans/URI, retry/backoff, and the guarantee
   that no partial checked result, token, diagnostic, completion list, cache, or
   Workspace Edit is published;
4. interaction with immutable snapshots, scheduling priorities, cancellation,
   fairness, dependency isolation, cleanup, process safety, and deterministic
   behavior independent of host allocator, CPU, or memory size; and
5. protocol inventory, Stable versus Experimental fields, migration rules, and
   executable positive/negative/boundary/concurrency/Unicode/CRLF/BOM,
   cancellation, stale, deterministic, and no-OOM fixtures.

Until these decisions are Accepted, a quota could measure the wrong unit,
reject a newer request instead of an older one, leak partial results, or turn
host memory behavior into an accidental LSP compatibility contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/ERROR-CODES.md`, `docs/ROADMAP-1.0.md`, RFC-0002, RFC-0020, DEC-0019,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-project`, `crates/ling-bytecode`, `crates/ling-diagnostics`, and
the current LSP-2501/LSP-2502/LSP-2503 authority boundaries.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2504` can begin after LSP snapshot/version, cancellation, scheduling,
diagnostics, and Semantic Transaction lifecycle decisions are Accepted. The
future implementation must use explicit versioned units and quotas, fail
before partial publication, return registered bilingual tool diagnostics, and
remain bounded and deterministic under adversarial input.
