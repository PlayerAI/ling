# DEC-0285: Private remote-reference separation evidence / 私有远程引用分离证据

> 状态：Accepted<br>
> 提出日期：2026-09-04<br>
> 决定日期：2026-09-04<br>
> Owner role：remote-design<br>
> 相关 RFC/缺口：DEC-0110 | DEC-0270 | DEC-0274 | DEC-0284 | GAP-ACTOR-REMOTE-DELIVERY-001 | REM-2601<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable REM-2601 package that can prove
the accepted local Actor reference remains runtime-scoped and structurally
separate from candidate remote-coordinate dimensions. All candidate remote
parts remain private test data. This is not a RemoteRef type, endpoint address,
capability credential, network Effect, delivery result, or wire protocol.

本决定定义 REM-2601 可执行证据的最小边界：证明已经接受的本地 Actor 引用仍受
runtime scope 限制，并与候选远程坐标维度保持结构分离。所有候选 remote parts 都只是
私有测试数据；它们不是 RemoteRef 类型、endpoint 地址、Capability 凭证、网络 Effect、
delivery 结果或 wire protocol。

## Question

What exact crate-private evidence may demonstrate that `LocalActorRef` cannot
act as a remote address and that endpoint, remote-actor, protocol-version, and
capability dimensions must not collapse into one untyped value, without
inventing unresolved remote identity, equality, serialization, authentication,
network Effect, delivery/Fault, incarnation, migration, or transport contracts?

## Decision

1. **Scoped authority.** This decision authorizes only one
   crate-private, `cfg(test)` REM-2601 evidence matrix in `ling-eval`. It adds
   no production or source `RemoteRef`, endpoint registry, remote identity,
   protocol version, Capability token, conversion, network Effect, transport,
   delivery result, public API, command, diagnostic, schema, or protocol and
   does not close `GAP-ACTOR-REMOTE-DELIVERY-001`.

2. **Accepted local input only.** Runtime evidence must construct an existing
   DEC-0274 local Actor runtime only from successful DEC-0270 through DEC-0273
   Checked Actor Core. Source must pass the normal
   `SourceFile → parse → AST → HIR → resolve → typecheck → Effect check`
   pipeline. Unresolved AST, untyped HIR, malformed Core, decoded documents,
   and hand-built Actor references remain non-executable.

3. **Local runtime scope.** The `local-reference-runtime-scope` case must start
   two local runtimes with distinct explicit nonzero `ActorRuntimeId` values,
   spawn one checked Actor in the first, and prove the second rejects that
   reference with the existing `WrongRuntime` result before consuming the
   payload or changing command/queue state. Both runtimes must shut down
   explicitly. This reuses DEC-0274 behavior and adds no remote failure class.

4. **Source-evidence independence.** The
   `source-independent-local-type-evidence` case must compile equivalent LF and
   UTF-8 BOM plus CRLF Actor programs under distinct source IDs/names, start
   distinct explicit local runtimes, and compare only the accepted definition,
   `ActorTypeId`, message-schema identity, and deterministic first local
   `ActorId`. Runtime IDs must remain different. This does not make any local
   instance identity stable or transferable across runs.

5. **Test-only candidate parts.** The evidence module may define four private
   nominal wrappers named as candidate endpoint, remote-actor, protocol-version,
   and capability-token parts plus one private aggregate used only by its test.
   Fixed labels are non-secret fixtures and must never leave the test process,
   be formatted as credentials, or enter production code.

6. **Dimension separation, not identity semantics.** The
   `candidate-remote-dimension-separation` case must change each candidate part
   independently and require every changed aggregate to remain structurally
   distinct from the baseline. Rust test equality is permitted solely as
   structural regression evidence. It does not define RemoteRef equality,
   endpoint identity, token participation in identity, canonical bytes,
   serialization, negotiation, authentication, attenuation, revocation, or
   compatibility.

7. **No Local-to-Remote conversion.** The
   `local-to-remote-conversion-absence` case must scan the current local Actor
   runtime and Checked Actor Core for named conversion, serialization, network
   address, and endpoint accessor surfaces. A `LocalActorRef` may expose only
   its accepted local runtime, Actor, Actor type, and message-schema evidence;
   it is not an input to the test-only candidate aggregate.

8. **Exact DEC-0110 disposition.** All fourteen DEC-0110 concerns must occur
   exactly once and be classified as follows:

   - existing local evidence: local-reference separation and the local
     serialization boundary;
   - test-only candidate dimensions: remote-reference identity, endpoint
     identity, remote-actor identity, protocol version, and capability token;
   - deferred public contracts: endpoint authority, protocol negotiation,
     Network Effect, ActorSend Effect, delivery outcome, Fault outcome, and
     incarnation.

   The names in the candidate group are vocabulary and nominal-field evidence
   only. They do not define public representation or behavior.

9. **Negative public-surface gate.** The
   `deferred-public-remote-surface-absence` case must prove that current
   evaluator, local Actor runtime, Effect/type/project sources expose none of
   the named RemoteRef/endpoint/token/connect/authenticate/send/serialization
   APIs; the CLI exposes no remote command; no `L-REMOTE-*` code or remote
   schema exists; and no `PROTO-REMOTE*` protocol record claims a public
   implementation.

10. **Security boundary.** Candidate capability bytes are inert test labels,
    not secrets, authenticators, bearer credentials, trust roots, signatures,
    or authorization decisions. The evidence must perform no network, DNS,
    socket, process, environment, file, registry, key-store, or clock access.
    It makes no confidentiality, integrity, authenticity, or revocation claim.

11. **Determinism and Unicode.** Evidence depends only on fixed checked source,
    explicit runtime IDs, fixed limits, local commands, and private candidate
    constants. Source names, source IDs, LF versus BOM/CRLF, host paths,
    allocation addresses, Rust debug text, hash-map order, wall time, and
    threads must not become remote identity. Original UTF-8 spans and Unicode
    17.0.0 remain unchanged.

12. **Completion boundary.** Acceptance plus passing evidence may mark
    REM-2601 `Done` only as an internal Experimental local/remote separation
    baseline. Public RemoteRef construction/equality/lifecycle, endpoint
    addressing/discovery/authority, remote incarnation, protocol negotiation,
    authentication, Capability lifecycle, Network/ActorSend Effects,
    delivery/Fault behavior, serialization, transport, migration, cross-process
    operation, and Stable compatibility remain blocked pending RFC-0009 or
    replacement Accepted authority.

## Conformance plan

The implementation must provide exactly five named evidence cases:

1. `local-reference-runtime-scope`;
2. `source-independent-local-type-evidence`;
3. `candidate-remote-dimension-separation`;
4. `local-to-remote-conversion-absence`; and
5. `deferred-public-remote-surface-absence`.

The first two must exercise real checked Actor sources and the existing local
runtime. The third may use only private inert candidate values. The final two
must enforce conversion and public-surface absence plus the exact fourteen-
concern disposition inventory. Focused `ling-eval` tests, lint, full workspace
tests, governance/status checks, formatting, and retained local Actor suites
must pass offline.

No acceptance claim may rely on a network address, serialized reference,
credential, external process, environment state, file, socket, wall-clock
timing, host path, Rust layout/debug output, or unregistered protocol.

## Compatibility impact

- **Source and semantics:** none. Existing checked local Actor declarations and
  DEC-0274 runtime behavior are exercised without adding a Ling construct,
  local/remote conversion, or observable remote behavior.
- **CLI/LSP/editor and diagnostics:** none. No command, option, server method,
  error code, output format, or remote configuration is added.
- **Schema/protocol/data/security:** none. Candidate parts are private in-memory
  test values and not a format or credential. No schema, protocol, address,
  trust, key, token lifecycle, stored data, or migration contract is created.
- **Semantic IDs and runtime:** none. Existing Actor type/schema identities and
  runtime-scoped local IDs are observed without changing the compiler,
  evaluator, scheduler, bytecode, VM, ABI, package, dependency, or backend.
- **Determinism and Unicode:** the matrix asserts only current local
  reconstruction and private structural distinctions. Unicode remains 17.0.0
  and original UTF-8 spans remain authoritative.

## Unresolved alternatives

- RFC-0009 or replacement authority must define public RemoteRef type/equality,
  EndpointId and RemoteActorId representation/allocation/reuse, incarnation,
  liveness, endpoint discovery and authority, protocol version negotiation,
  serialization, canonical identity, and migration.
- Security authority must define token issuance, audience/scope, attenuation,
  expiry, revocation, replay protection, authentication, authorization, trust
  roots, privacy, redaction, integrity, key storage, and failure behavior.
- Effect and delivery authority must define Network and ActorSend Effects,
  payload ownership, delivery/Fault values, timeout/partition/disconnect,
  mailbox/backpressure interaction, ordering, retry, duplication,
  deduplication, resource limits, and replay behavior.
- Transport adapters, endpoint registries, public schemas/diagnostics, protocol
  inventory entries, cross-process/backend/platform fixtures, and Stable
  support remain future work. The candidate wrappers are not reserved names or
  candidate wire layouts.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
