# REM-2601 Authority Audit: RemoteRef and Endpoint

## Outcome

`REM-2601` is Done only for the private Experimental local/remote reference
separation baseline authorized by Accepted DEC-0285 and bound to implementation
commit `ec5b2182676a1f2b8311b7a0e416dec42efe5705`. Its five-case matrix exercises
the existing checked local Actor runtime, proves `LocalActorRef` remains
runtime-scoped, and represents endpoint, remote-actor, protocol-version, and
capability dimensions only as structurally separate private test values.

The public G2 plan still requires remote identity/equality/lifecycle,
EndpointId/RemoteActorId allocation and reuse, endpoint authority, protocol
negotiation, authentication, Capability lifecycle, network Effects,
delivery/Fault semantics, serialization, transport, migration, and
cross-process evidence. No production/public remote type, registry, credential,
Effect, transport, diagnostic, schema, command, protocol, or placeholder G2 API
was added.

## Normative traceability

- The G2 execution package is non-normative. Its `RemoteRef<Message>` sketch
  cannot authorize network identity, wire fields, authentication, transport,
  or delivery behavior.
- ACT-2305 and REP-2506 dependencies are Done for their accepted private
  baselines. No Accepted RFC-C206 or replacement RFC-0009 exists, and RFC-0001
  remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` distinguishes local and remote Actor references while
  leaving remote delivery strategy to a future RFC. It does not define remote
  allocation, equality, incarnation, endpoint authority, protocol negotiation,
  Capability trust, failure, partition, or wire behavior.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit network Effects,
  remote delivery, authentication, and Capability boundaries but do not define
  a stable remote type, ABI, schema, or protocol.
- Accepted DEC-0270 and DEC-0274 define only checked local Actor Core and a
  runtime-scoped `LocalActorRef`. DEC-0110 records fourteen boundary concerns.
  DEC-0285 narrowly authorizes executable private separation evidence while
  explicitly deferring all public remote contracts.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open and continues to require
  positive, negative, migration, partition, duplication, ordering, and
  security evidence before public remote behavior can be exposed.

## Current implementation evidence

- `remote_ref_execution_evidence.rs` contains exactly the five DEC-0285 cases.
  Real runtime cases build only successfully checked Actor Core from fixed
  source and use the existing `ActorRuntime` and `LocalActorRef`.
- A cross-runtime send returns the existing `WrongRuntime`, retains the exact
  payload, and leaves the receiving runtime's command and queued-message counts
  at zero. This proves local runtime scope, not remote failure behavior.
- LF and BOM/CRLF sources under distinct source IDs/names preserve accepted
  local type/schema evidence and deterministic first local Actor ID while
  explicit runtime IDs differ. No instance identity becomes transferable.
- Candidate endpoint, remote-actor, protocol-version, and capability parts are
  private nominal test wrappers around inert constants. Their Rust structural
  inequality is not RemoteRef equality, canonical identity, serialization,
  negotiation, authentication, or compatibility.
- Static negative gates retain the absence of local-to-remote conversion,
  public remote runtime/type/Effect APIs, a CLI command, `L-REMOTE-*`
  diagnostics, remote schemas, and implemented `PROTO-REMOTE*` records.

## Required authority before public implementation

An Accepted RFC or decision must define, at minimum:

1. local versus remote reference identity, RemoteRef construction/equality/
   ownership/lifetime, EndpointId and RemoteActorId allocation/reuse,
   incarnation, liveness, serialization prohibition, and migration;
2. endpoint discovery/authority, protocol negotiation, authentication and
   authorization, Capability issuance/audience/scope/attenuation/expiry/
   revocation, trust roots, privacy, integrity, and replay protection;
3. Network and ActorSend Effects, delivery/Fault values, timeout/partition/
   disconnect behavior, mailbox/backpressure interaction, ordering, retry,
   duplication/deduplication, replay, and resource limits;
4. canonical bytes, schemas, diagnostics, Semantic Graph/Audit Source
   projection, protocol inventory/versioning, migration, and compatibility;
   and
5. executable positive/negative/migration/partition/duplication/ordering/
   security fixtures covering stale identities, endpoint mismatch, negotiation,
   capability revoke, Unicode/CRLF/BOM spans, deterministic output, and
   interpreter/VM/runtime behavior without unchecked-AST execution.

DEC-0285 avoids these unresolved choices by proving only that the accepted
local reference cannot cross runtime scope and that candidate remote-coordinate
dimensions must not collapse into one untyped value. Until the remaining
authority is Accepted, that result must not be presented as a public remote
identity, address, credential, transport, or delivery contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0012, DEC-0013,
DEC-0018, DEC-0110, DEC-0270 through DEC-0274, DEC-0284, DEC-0285, RFC-0001,
RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current Actor runtime,
compiler pipeline, evaluator, bytecode, VM, diagnostic, CLI, and schema code.

No production compiler, interpreter, VM, bytecode, scheduler, mailbox, local
Actor behavior, RemoteRef, endpoint, network, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed. All candidate remote
parts and negative scans are confined to `cfg(test)` evidence.

## Intentionally deferred

REM-2601 is Done only for the private DEC-0285 Experimental separation
baseline. Public RemoteRef identity/equality/lifecycle, endpoint addressing and
authority, incarnation, protocol negotiation, authentication, Capability
lifecycle, network Effects, delivery/Fault behavior, serialization, transport,
migration, cross-process/backend/platform fixtures, diagnostics, protocols,
and Stable support still require RFC-0009 or replacement Accepted authority.
See `docs/status/REM-2601-IMPLEMENTATION-REPORT.md` for the exact evidence and
executed verification.
