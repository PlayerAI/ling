# LSP-2503 Authority Audit: Debounce and Priority

## Outcome

`LSP-2503` is correctly recorded as `BlockedSpec`. The execution plan proposes
high-priority typing diagnostics, low-priority workspace indexing, cancellation
of older analysis after a new revision, and responsive definition/hover
requests that do not wait for unrelated workspace builds. No accepted LSP
event, scheduling, debounce, freshness, fairness, or publication contract
defines those observable choices.

No debounce timer, priority queue, scheduler policy, stale-result publisher,
cancellation integration, diagnostic allocation, protocol schema, or
placeholder LSP service was added.

## Normative traceability

- The execution package is non-normative; its priority labels and debounce
  bullets do not authorize wall-clock behavior, request ordering, or a public
  scheduling API.
- Accepted DEC-0019 defines internal query invalidation and cooperative
  cancellation at publication boundaries. Accepted DEC-0021 defines
  deterministic bounded parallelism for internal pure query jobs. Both
  explicitly keep LSP fields and public protocols out of scope.
- Existing diagnostic semantics and `PROTO-DIAGNOSTIC-JSON` define checked
  diagnostic facts, not `didOpen`/`didChange` triggers, debounce intervals,
  priority classes, or stale replacement.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave request/snapshot/version,
  publication, lifecycle, and migration fields open.
- LSP-2202 push diagnostics, LSP-2501 request snapshots, and LSP-2502
  cancellation are `BlockedSpec`; scheduling cannot safely publish or suppress
  their results before those authorities exist.

## Current interface evidence

- `ling-db` has internal revisions, query invalidation, and deterministic query
  scheduling, but no LSP event stream, debounce policy, priority classes,
  request queue, or result publication boundary.
- `ling-diagnostics` produces deterministic checked diagnostics, but no
  `publishDiagnostics` event, document-version association, stale replacement,
  or clear policy exists.
- No LSP server or editor transport defines whether typing, definition/hover,
  workspace indexing, cancellation, and dependency work share queues or
  budgets, and no implementation can prove that a new revision suppresses all
  older analysis.
- No fixture covers edit bursts, timer coalescing, priority inversion or
  starvation, new-revision cancellation, stale results, concurrent requests,
  workspace/index throttling, Unicode/CRLF/BOM, or deterministic scheduling
  independent of host timing.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. event triggers and debounce/coalescing semantics for open/change/workspace
   updates, including whether intervals are configurable, logical, or
   wall-clock and how immediate requests bypass a pending batch;
2. priority classes, queue ordering, fairness/starvation bounds, dependency
   expansion, worker/resource budgets, and interaction with DEC-0021 internal
   scheduling without exposing host CPU/timing as Ling behavior;
3. revision/request snapshot and cancellation association, supersession rules,
   stale-result rejection, and publication ordering for diagnostics, symbols,
   tokens, completion, hover, indexing, progress, and Workspace Edits;
4. diagnostics trigger/clear/replace behavior, related-file scope, generated
   and dependency policy, capability/configuration negotiation, errors,
   limits, and Stable versus Experimental lifecycle; and
5. executable positive/negative/edit-burst/stale/cancellation/priority,
   starvation, clear/replace, Unicode/CRLF/BOM, deterministic, and migration
   fixtures that do not rely on machine timing.

Until these decisions are Accepted, a scheduler could publish stale results,
starve interactive requests, clear a newer diagnostic set, or make host timer
and CPU behavior an accidental editor compatibility contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/ERROR-CODES.md`, `docs/ROADMAP-1.0.md`, DEC-0019, DEC-0021,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
the LSP-2202/LSP-2501/LSP-2502 authority boundaries, `crates/ling-db`, and
`crates/ling-diagnostics`.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2503` can begin after LSP snapshot/version, cancellation, diagnostics,
overlay/change, and Semantic Transaction lifecycle decisions are Accepted.
The future implementation must coalesce revisions explicitly, prioritize
interactive work without starvation, cancel superseded analysis, publish only
current snapshot results, and keep scheduling timing out of Ling semantics.
