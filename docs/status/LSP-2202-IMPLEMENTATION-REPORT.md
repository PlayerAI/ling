# LSP-2202 implementation report

> Status: implementation complete; registry binding pending
> Task: `LSP-2202`
> Authority: Accepted `RFC-0032`, `RFC-0031`, `RFC-0030`, `RFC-0029`,
> `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0034`, `DEC-0035`, `DEC-0071`,
> and `DEC-0072`

## Scope

This milestone implements deterministic push diagnostics for the current Ling
LSP server. Successful source mutations schedule complete-state analysis;
only an exact-current immutable result may atomically replace the published
URI ledger. It deliberately does not implement pull diagnostics, suppression,
general cancellation requests, or Workspace Edits.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2202: push triggers,
  logical debounce, complete result publication, version association,
  stale-result rejection, and deterministic replacement/clearance.
- RFC-0032 §§1–3: successful-mutation scheduling, explicit message-boundary
  flush, immutable snapshot tickets, full freshness comparison, syntax
  precedence, and checked-workspace compiler diagnostics.
- RFC-0032 §§4–6: adapter 0.2 temporary identities, exact notification params,
  URI order, version rules, empty clearance, atomic ledger replacement,
  transport bounds, and exact initialize discovery marker.
- RFC-0004/RFC-0023/RFC-0029/RFC-0030 and DEC-0019/DEC-0071: lifecycle,
  negotiated encoding, overlay versions, atomic workspace reload, revisioned
  project inputs, and immutable request observation.

## Implementation

- `ling-db::CompilerDb::workspace_diagnostics` exposes one deterministic
  compiler bridge: lexical errors precede parse errors, any syntax error stops
  semantic checking, and valid non-temporary workspaces run HIR lowering,
  resolution, type checking, and Effect checking.
- `ling-lsp::DiagnosticAnalysisTicket` captures owned visible bytes, document
  identity/origin/open/version state, encoding, lifecycle, session revision,
  and workspace inputs. Compilation reads only this ticket.
- `LspServer` tracks pending work and a committed publication ledger. It
  suppresses exact no-ops, rejects stale results, emits URI-sorted changed or
  removed entries, validates all encoded frame sizes, and mutates the ledger
  only after complete success.
- The stdio loop flushes after each handled message and writes any request
  response before notifications caused by that request.
- `ling.lsp.diagnostic/0.2` extends RFC-0031 only for exact validated temporary
  sources; `ling.lsp.publish-diagnostics/0.1` is registered separately.

## Tests and evidence

- `crates/ling-db/tests/workspace_diagnostics.rs` covers clean input,
  registered lexical, parse, HIR, resolution, type, and Effect failures,
  syntax-over-semantic precedence, and repeated output.
- `crates/ling-lsp/tests/diagnostic_adapter.rs` retains the full RFC-0031
  Unicode/position/field/ordering/error corpus and adds temporary-identity
  acceptance plus non-temporary rejection evidence.
- `crates/ling-lsp/tests/push_diagnostics.rs` covers exact capability and wire
  framing, versioned replacement, coalescing, stale completion, temporary
  isolation/clearance, close-to-disk and multi-document URI-ordered disk
  publication, disk/reload no-ops, source removal, workspace-input staleness,
  response-before-notification order, and oversized-output failure atomicity.
- The focused locked-offline tests and strict Clippy checks pass. The full
  repository gates and exact implementation commit are recorded during the
  follow-up status-binding step.

## Compatibility, schemas, and Semantic IDs

- Adds Experimental `ling.lsp.publish-diagnostics/0.1` and advances
  Experimental `ling.lsp.diagnostic/0.1` compatibly to 0.2. Neither is Stable.
- No registered diagnostic code, severity, message, Facts, repair, Semantic
  ID, core diagnostic JSON schema, Ling syntax/semantics, Typed Core, runtime,
  bytecode, VM, ABI, or Unicode table changes.
- Notifications contain path-free logical URIs and deterministic values only;
  host paths, clocks, worker order, allocation order, and debug text are not
  observable.

## Intentionally deferred

LSP-2203 owns pull diagnostics and push/pull parity. LSP-2204 owns root-cause
grouping, deduplication, caps, and suppression. Cancellation requests, partial
results, progress, tags, code-description URLs, repair application, Workspace
Edits, Semantic Transactions, wall-clock debounce, worker pools, and Stable
compatibility require later Accepted authority.
