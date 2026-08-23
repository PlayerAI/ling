# LSP-2504 Authority Audit: Memory and Resource Limits

## Outcome

The original audit correctly recorded `LSP-2504` as `BlockedSpec`: the
execution plan named limits but no accepted LSP contract defined their units,
scope, precedence, response, cleanup, retry, or compatibility. Accepted
RFC-0051 now closes that authority gap for a bounded Preview slice.

DEC-0033 remains the checked UTF-8 arithmetic child. RFC-0051 composes it with
the already accepted frame, document, completion, diagnostic, and RFC-0005
Trait-solver bounds, and authorizes fixed aggregate open-overlay bytes, fixed
live-request admission, `L-LSP-0002`, exact discovery, and failure-atomic
cleanup. Implementation evidence is recorded in
`docs/status/LSP-2504-IMPLEMENTATION-REPORT.md`.

## Normative traceability

- The execution package is non-normative; its resource list does not authorize
  public quotas, defaults, diagnostics, or host-memory behavior.
- Accepted RFC-0002 resource limits apply to project manifests and dependency
  graphs. Accepted bytecode limits apply to bytecode envelopes/tables. Their
  domain-specific diagnostics and units cannot be reused as LSP quotas.
- DEC-0019 bounds internal query inputs and requires oversized or malformed
  snapshots to fail without publishing partial checked results, but it does
  not define an LSP resource protocol or tool diagnostic.
- Accepted DEC-0033 defines only local UTF-8-byte accounting and typed
  arithmetic failures; it does not define quotas, configuration, allocator
  observation, or a client-visible response.
- `PROTO-DIAGNOSTIC-JSON` is a Preview compiler/CLI diagnostic writer, not an
  LSP resource-limit response. The single diagnostic registry permits only
  registered `L-<DOMAIN>-<NUMBER>` meanings.
- Accepted RFC-0049 and RFC-0050 close cancellation, association, scheduling,
  fairness, and supersession dependencies. RFC-0051 deliberately leaves
  general mutation, configurable resource operations, Stable lifecycle, and
  Semantic Transactions in their existing gaps.

## Current interface evidence

- `ling-project`, `ling-bytecode`, and parser boundaries retain explicit local
  limits for their own inputs and are not reused as LSP policy. RFC-0051 now
  governs only the documented LSP-owned resources and composes existing result
  and solver bounds without changing their original domains.
- `LspServer` now owns an 8 MiB checked aggregate open-overlay byte budget;
  open/change/close reserve or release exact decoded UTF-8 bytes without
  publishing a resource-rejected overlay or version.
- The stdio registry admits at most 128 queued or executing distinct live IDs,
  applies duplicate-first precedence, creates no association for the 129th,
  and reuses the identity-safe completion cleanup path.
- Exact discovery inventories existing 1 MiB frame/document, 256 completion,
  diagnostic default/hard maxima, and RFC-0005 64-level Trait nesting without
  inventing allocator/process-memory or editor-only compiler semantics.
- `L-LSP-0002` and its exact `-32803` data are registered, versioned, bilingual,
  path/source/request-ID free, and covered by deterministic fixtures.

## Accepted closure

RFC-0051 defines the previously required contracts:

1. exact UTF-8-byte and logical-count units plus document/session scopes and
   explicit inclusion/exclusion rules;
2. fixed hard values, diagnostic-only lower configuration, exact Preview
   discovery, and new-marker migration requirements;
3. duplicate/cancellation/validation/resource/stale precedence, structured
   bilingual failure data, response-free notifications, retry, and atomicity;
4. snapshot, scheduling, cancellation, cleanup, privacy, and deterministic
   independence from allocator, CPU, load, path, and wall time; and
5. executable boundary, aggregate, concurrency, Unicode, BOM, CRLF,
   cancellation, retry, deterministic, and no-partial-publication evidence.

## Evidence and compatibility

This closure was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/ERROR-CODES.md`, `docs/ROADMAP-1.0.md`, RFC-0002, RFC-0005, RFC-0020,
RFC-0049, RFC-0050, RFC-0051, DEC-0019,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-project`, `crates/ling-bytecode`, `crates/ling-diagnostics`, and
the current LSP-2501/LSP-2502/LSP-2503 authority boundaries.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

Allocator/RSS guarantees, OOM recovery, configurable non-diagnostic quotas,
eviction, partial results, progress, deadlines, total compiler fuel, general
workspace/dependency memory accounting, persistence, Stable lifecycle, and
Semantic Transactions remain deferred. They are not required by RFC-0051's
complete Preview slice and need separate Accepted authority.
