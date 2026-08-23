# LSP-2101 Authority Audit: Lifecycle skeleton

## Outcome

The original `BlockedSpec` finding is now closed. Accepted RFC-0004 defines
`initialize`, `initialized`, `shutdown`, and `exit`, server information,
position-capability negotiation, opaque workspace folders, lifecycle errors,
bounded JSON-RPC framing, and stdio protocol purity. Accepted DEC-0257 confirms
that the implemented slice is the complete bounded LSP-2101 parent.

The closure adds no duplicate server or new protocol behavior. It recognizes
the existing `ling-lsp` state machine, `ling lsp --stdio` delegate, protocol
inventory entry, and executable fixtures as parent-level evidence.

## Normative traceability

- RFC-0004 is the Accepted lifecycle/transport authority and explicitly limits
  its claim to Preview lifecycle behavior.
- DEC-0029 owns position encoding; DEC-0257 composes the existing server and
  completed CLI command model as LSP-2101.
- RFC-0023 and RFC-0026 independently authorize later overlay and formatting
  capability extensions without changing lifecycle transitions.
- Open Semantic Graph/Transaction, Workspace Edit, snapshot, and cancellation
  gaps remain outside this parent rather than blocking its narrower scope.

## Current interface evidence

- `crates/ling-lsp` implements bounded framing, lifecycle state, initialization
  validation, deterministic response rendering, and the stdio loop.
- `crates/ling-lsp/tests/lifecycle.rs` covers lifecycle, workspace, malformed
  transport, ordering, limits, Unicode, and early-exit cases.
- `crates/ling-cli/tests/lsp.rs` verifies the real process keeps stdout framed
  and stderr quiet through initialization and shutdown.
- `PROTO-LSP-LIFECYCLE` inventories `ling.lsp.lifecycle/0.1` as Preview.

## Authority closure

RFC-0004 and DEC-0257 now define every parent requirement: protocol version,
framing and limits, lifecycle transitions and errors, server information,
capability/workspace validation, exit/channel behavior, Preview lifecycle,
and executable positive/negative fixtures. Snapshot, cancellation, edit, and
transaction rules are not requirements of the lifecycle skeleton.

## Evidence and compatibility

This closure changes no LSP bytes, CLI behavior, Ling semantics, diagnostic
allocation, schema, Semantic ID, source span, runtime, bytecode, VM, ABI,
filesystem/network behavior, or Unicode 17.0.0 data.

## Intentionally deferred

Incremental sync, workspace reload, diagnostics, navigation, completion, code
actions, semantic tokens, cancellation, Workspace Edits, Semantic
Transactions, and Stable editor compatibility remain separately governed.
