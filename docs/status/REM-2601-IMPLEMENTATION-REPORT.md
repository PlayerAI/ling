# REM-2601 Implementation Report

## Outcome

REM-2601 is complete only for the private Experimental local/remote reference
separation baseline authorized by Accepted DEC-0285. Implementation commit
`ec5b2182676a1f2b8311b7a0e416dec42efe5705` adds one crate-private,
`cfg(test)` evidence module to `ling-eval`. It exercises the existing checked
local Actor runtime, demonstrates runtime-scoped `LocalActorRef` behavior, and
keeps four candidate remote-coordinate dimensions structurally separate in
private inert test data.

This package is not a public `RemoteRef`, endpoint address or registry,
credential format, identity/equality/lifecycle contract, network Effect,
delivery/Fault result, serialization format, transport, or cross-process
runtime. `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open, and no `PROTO-REMOTE*`
implementation exists.

## Normative clauses covered

- DEC-0285 clauses 1-2 and 12: the module is test-only and consumes only Actor
  sources that pass the complete `SourceFile → parse → AST → HIR → resolve →
  typecheck → Effect check` pipeline. It does not expose a source construct or
  production remote API.
- Clause 3: two explicit nonzero local runtime IDs are distinct. A reference
  spawned in the first runtime is rejected by the second with the existing
  `WrongRuntime` result before payload consumption or command/queue mutation;
  both runtimes shut down explicitly.
- Clause 4: equivalent LF and UTF-8 BOM plus CRLF programs under different
  source IDs and names preserve the accepted definition, `ActorTypeId`,
  message-schema identity, and deterministic first local `ActorId`, while the
  runtime IDs remain distinct.
- Clauses 5-6 and 10: endpoint, remote-actor, protocol-version, and
  capability-token candidate parts are private nominal wrappers around fixed
  inert labels. Changing each one independently yields a structurally distinct
  aggregate; this defines no identity, credential, or wire behavior.
- Clauses 7-9: static negative gates retain the absence of local-to-remote
  conversion/serialization/accessors, public remote APIs, a CLI route,
  `L-REMOTE-*` diagnostics, remote schemas, and implemented `PROTO-REMOTE*`
  records. All fourteen DEC-0110 concerns occur exactly once with the required
  dispositions.
- Clause 11: tests use fixed sources, explicit runtime IDs, finite limits, and
  private constants. Host paths, wall time, environment, network, allocation
  addresses, and Rust debug/layout details do not participate.

## Exact evidence matrix

The dedicated matrix contains exactly these cases:

1. `local-reference-runtime-scope`
2. `source-independent-local-type-evidence`
3. `candidate-remote-dimension-separation`
4. `local-to-remote-conversion-absence`
5. `deferred-public-remote-surface-absence`

The DEC-0110 concern inventory is complete and duplicate-free:

- existing local evidence: local-reference separation and the local
  serialization boundary;
- test-only candidate dimensions: remote-reference identity, endpoint
  identity, remote-actor identity, protocol version, and capability token;
- deferred public contracts: endpoint authority, protocol negotiation,
  Network Effect, ActorSend Effect, delivery outcome, Fault outcome, and
  incarnation.

The candidate labels are vocabulary and nominal-field regression evidence
only. They are not reserved public names, addresses, secrets, authenticators,
canonical bytes, equality rules, or candidate wire layouts.

## Executed verification

Commands executed locally on 2026-09-04:

- `cargo test -p ling-eval --lib remote_ref_execution_evidence --locked --offline`
  — passed: 5 cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 97
  library tests with 4 unrelated replay child probes intentionally ignored;
  Actor/runtime integration suites also passed.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test --workspace --all-targets --locked --offline` — passed.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` —
  passed.
- `cargo xtask governance check-all` — passed with 352 authority documents, 29
  gaps, 327 lifecycle records, 50 protocols, and 104 diagnostic codes.
- `cargo xtask status verify` — passed: 499 tasks, 357 Done.
- `cargo xtask docs verify` and `cargo xtask rc0 verify` — passed.
- `cargo fmt --all` and `git diff --check` — passed.

## Compatibility impact

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, Semantic IDs,
  public protocols, package/ABI versions, stored data, dependencies, bytecode,
  VM, backends, and migration: unchanged.
- Production local Actor scheduling, mailbox, send, and shutdown behavior:
  unchanged. The module only asserts existing DEC-0274 behavior.
- No network, DNS, socket, process, environment, file, registry, key-store, or
  clock access is added. Candidate capability bytes are inert local fixtures,
  not credentials or authorization inputs.
- Unicode remains 17.0.0, and original UTF-8 spans remain authoritative
  sidecar evidence.

## Deferred work

Public `RemoteRef` construction, equality, ownership, lifetime, liveness, and
incarnation; EndpointId and RemoteActorId representation/allocation/reuse;
endpoint discovery and authority; protocol negotiation; authentication,
authorization, trust, token lifecycle, privacy, integrity, and replay
protection; Network/ActorSend Effects; delivery/Fault values; timeout,
partition, disconnect, ordering, retry, duplication, deduplication,
backpressure, and resource behavior; serialization, canonical bytes, schemas,
diagnostics, transports, migration, cross-process/backend/platform evidence,
and Stable compatibility remain intentionally deferred to RFC-0009 or
replacement Accepted authority.
