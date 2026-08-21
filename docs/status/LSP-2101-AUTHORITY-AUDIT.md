# LSP-2101 Authority Audit: Lifecycle skeleton

## Outcome

`LSP-2101` is correctly recorded as `BlockedSpec`. The execution plan asks for
`initialize`, `initialized`, `shutdown`, and `exit`, with server information,
capability negotiation, workspace folders, pre-initialize rejection, and
stdio protocol purity. No accepted LSP/JSON-RPC lifecycle or public protocol
inventory entry defines those behaviors in the repository.

No LSP server crate, transport loop, lifecycle state machine, capability field,
workspace-folder mapping, or placeholder command was added. Existing CLI and
compiler behavior remains unchanged.

## Normative traceability

- `docs/SEMANTICS.md` defines Semantic Graph and Transaction concepts but does
  not define a JSON-RPC transport, server lifecycle, capability schema, or
  editor protocol version.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version, Workspace Edit,
  and transaction boundaries open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves
  Semantic Graph and Transaction lifecycle/versioning open.
- `PROTO-SEMANTIC-GRAPH-JSON` is an Experimental graph projection, not an LSP
  server protocol. The protocol inventory contains no LSP lifecycle entry.
- Accepted DEC-0003 and DEC-0013 govern the `ling` CLI and its stdio/output
  behavior, not JSON-RPC framing or LSP shutdown semantics.
- The lower-authority LSP plan is an implementation proposal and cannot
  authorize a public server, editor repository, or capability negotiation.

## Current interface evidence

The current repository confirms the missing boundary:

- There is no LSP crate, JSON-RPC reader/writer, lifecycle state, server info,
  capability schema, workspace-folder resolver, or LSP fixture corpus.
- `ling-db` and `ling-source` provide compiler/VFS query boundaries, but no
  transport or request lifecycle and no guarantee that stdio is protocol-pure.
- The protocol inventory lists future LSP/editor consumers only; it does not
  claim an LSP implementation or version.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. JSON-RPC framing, protocol version, message size/error limits, stdio/log
   channels, and malformed-message behavior;
2. lifecycle state transitions, pre-initialize and post-shutdown handling,
   exit status, server info, capability negotiation, and workspace-folder
   identity/path policy;
3. request cancellation, document/project snapshot association, concurrent
   request ordering, resource limits, and deterministic failure responses;
4. Stable versus Experimental fields, protocol inventory/lifecycle records,
   editor repository ownership, and migration/version policy; and
5. positive, negative, pre/post lifecycle, capability, workspace, malformed
   framing, stderr/stdout purity, Unicode/CRLF, and deterministic fixtures.

Until those decisions and fixtures are Accepted, implementing an LSP lifecycle
would freeze an editor compatibility surface without the transaction and
snapshot guarantees required by the governance gaps.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, `docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`, `crates/ling-db`,
`crates/ling-source`, and the current workspace manifests.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2101` can begin after an accepted LSP lifecycle/transport decision and
protocol inventory entry provide executable fixtures. The implementation must
keep stdio protocol-pure, use the approved snapshot boundary, and avoid
creating a competing semantic or diagnostic authority.
