# DEC-0286: Private transport-neutral envelope-parts evidence / 私有传输中立 Envelope 部件证据

> 状态：Proposed<br>
> 提出日期：2026-09-04<br>
> 决定日期：Pending<br>
> Owner role：remote-design<br>
> 相关 RFC/缺口：DEC-0111 | DEC-0270 | DEC-0271 | DEC-0274 | DEC-0285 | GAP-ACTOR-REMOTE-DELIVERY-001 | REM-2602<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the smallest executable REM-2602 package that can bind
existing checked local Actor type/schema evidence to structurally separate
candidate message metadata while refusing to define bytes, a checksum
algorithm, transport, remote identity, authentication, delivery, or a public
envelope.

本提案定义 REM-2602 可执行证据的最小边界：把现有 checked local Actor 的类型和
schema 证据绑定到结构上互相独立的候选消息元数据，同时拒绝定义 wire bytes、checksum
算法、transport、remote identity、authentication、delivery 或公开 Envelope。

## Question

What exact crate-private evidence may show that the G2 plan's minimum envelope
concerns remain typed and transport-independent, while the authority still
leaves canonical encoding, remote identity, checksum, authentication,
deadline/cancellation, delivery, extensions, resources, migration, and
compatibility unresolved?

## Decision

1. **Scoped authority.** If Accepted, this decision authorizes only one
   crate-private, `cfg(test)` REM-2602 evidence matrix in `ling-eval`. It adds
   no production/source envelope, encoder, decoder, bytes, checksum algorithm,
   protocol version, identity, credential, network Effect, transport,
   delivery/Fault result, API, command, diagnostic, schema, or protocol and
   does not close `GAP-ACTOR-REMOTE-DELIVERY-001`.

2. **Checked local input only.** Runtime evidence must build existing DEC-0274
   local Actor references only from successful DEC-0270 through DEC-0273
   Checked Actor Core. Source must pass the normal
   `SourceFile → parse → AST → HIR → resolve → typecheck → Effect check`
   pipeline. Unresolved AST, untyped HIR, malformed Core, decoded documents,
   hand-built Actor references, and unvalidated external bytes remain
   non-executable.

3. **Twelve private required parts.** The test module may define one private
   candidate aggregate with exactly twelve structurally named parts:

   - candidate protocol-version marker;
   - checked sender Actor type;
   - checked receiver Actor type;
   - checked receiver message-schema identity;
   - candidate message-id marker;
   - candidate correlation-id marker;
   - candidate deadline marker;
   - candidate cancellation marker;
   - candidate delivery-policy marker;
   - candidate authentication-metadata label;
   - one owned existing `ActorValue` payload; and
   - candidate payload-integrity marker.

   All candidate markers are fixed inert test labels with no units, clock,
   randomness, secrecy, cryptographic meaning, validation behavior, canonical
   form, or wire representation. The aggregate is not named or exported as a
   production Envelope.

4. **Source and runtime independence.** The
   `source-and-runtime-independent-envelope-binding` case must compile
   equivalent LF and UTF-8 BOM plus CRLF Actor programs under distinct source
   IDs/names, start distinct explicit local runtimes, and spawn checked sender
   and receiver Actors. Candidate aggregates built from their type/schema
   evidence and the same fixed private markers/payload must compare equal even
   though runtime and local Actor instance IDs differ. Runtime/local IDs must
   not enter the aggregate. All runtimes must shut down explicitly.

5. **Required-part separation.** The
   `candidate-required-part-separation` case must change each of the twelve
   parts independently and require every changed aggregate to remain
   structurally distinct from the baseline. Rust test equality is permitted
   solely as private regression evidence. It does not define Envelope or
   RemoteRef equality, identity, optionality, validation, encoding, field
   order, canonical bytes, negotiation, or compatibility.

6. **Payload and integrity-marker separation.** The
   `payload-and-integrity-marker-separation` case must prove that changing only
   the owned `ActorValue` payload leaves the inert integrity marker unchanged,
   while changing only the marker leaves the payload unchanged. No checksum,
   digest, signature, MAC, encryption, compression, serialization, or
   integrity decision may be computed or claimed.

7. **Transport and codec absence.** The
   `transport-codec-and-local-reference-absence` case must assert an exact
   twelve-part inventory and prove it contains no endpoint, address, local
   runtime/Actor ID, transport, socket, frame, codec, byte-buffer, canonical
   byte, checksum algorithm, or serialization part. Current local Actor and
   Checked Actor Core sources must retain the absence of an Actor-reference
   serialization or local-to-remote conversion surface.

8. **Exact DEC-0111 disposition.** All eighteen DEC-0111 concerns must occur
   exactly once and be classified as follows:

   - existing checked local evidence: sender semantic type, receiver semantic
     type, message schema, and payload;
   - test-only candidate parts: protocol version, message ID, correlation ID,
     deadline, cancellation, delivery policy, authentication metadata, and
     payload checksum/integrity marker;
   - deferred public contracts: extension fields, identity binding,
     incarnation binding, integrity behavior, resource limits, and migration.

   Candidate names are vocabulary and nominal-field evidence only. They do not
   define public representation or behavior; the six deferred concerns must
   not appear as candidate aggregate fields.

9. **Negative public-surface gate.** The
   `deferred-public-envelope-surface-absence` case must prove that current
   evaluator, local Actor runtime, Effect/type/project sources expose none of
   the named Envelope/encode/decode/checksum/protocol/authentication/transport
   APIs; the CLI exposes no remote/envelope command; no `L-REMOTE-*` code or
   remote-envelope schema exists; and no `PROTO-REMOTE*` protocol record claims
   a public implementation.

10. **Security and resource boundary.** Authentication and integrity markers
    are inert labels, not secrets, authenticators, bearer credentials, trust
    roots, signatures, hashes, authorization decisions, quotas, or validated
    limits. Evidence performs no network, DNS, socket, process, environment,
    file, registry, key-store, random-source, or clock access and allocates
    only the existing bounded local test runtime and fixed in-memory data.

11. **Determinism and Unicode.** Evidence depends only on fixed checked source,
    explicit runtime IDs/limits, local commands, owned Actor values, and private
    constants. Source names/IDs, LF versus BOM/CRLF, runtime/local Actor IDs,
    host paths, allocation addresses, Rust debug/layout text, hash-map order,
    wall time, and threads must not enter candidate parts. Original UTF-8 spans
    and Unicode 17.0.0 remain unchanged.

12. **Completion boundary.** Acceptance plus passing evidence may mark
    REM-2602 `Done` only as an internal Experimental typed-parts separation
    baseline. Public Envelope representation, required/optional/extension
    rules, canonical encoding, version negotiation, remote identity/incarnation
    binding, message/correlation allocation, deadline/cancellation semantics,
    delivery policy, authentication, checksum/integrity, resource limits,
    transport/framing, schemas, migration, cross-process operation, and Stable
    compatibility remain blocked pending RFC-0009 or replacement Accepted
    authority.

## Conformance plan

The implementation must provide exactly five named evidence cases:

1. `source-and-runtime-independent-envelope-binding`;
2. `candidate-required-part-separation`;
3. `payload-and-integrity-marker-separation`;
4. `transport-codec-and-local-reference-absence`; and
5. `deferred-public-envelope-surface-absence`.

The first must exercise real checked Actor sources and existing local runtimes.
The next two may use only private inert candidate values and owned existing
Actor values. The final two must enforce the exact part inventory, transport/
codec/local-reference absence, public-surface absence, and exact eighteen-
concern disposition. Focused `ling-eval` tests, retained local Actor and
REM-2601 suites, lint, full workspace tests, governance/status checks,
formatting, and offline dependency gates must pass.

No acceptance claim may rely on encoded bytes, field order, a computed
checksum, a remote address/reference, credential, transport, external process,
environment state, file, socket, clock, host path, Rust layout/debug output, or
unregistered protocol.

## Compatibility impact

- **Source and semantics:** none. Existing checked local Actor declarations and
  DEC-0274 runtime behavior are exercised without adding a Ling construct,
  remote message, or observable envelope behavior.
- **CLI/LSP/editor and diagnostics:** none. No command, option, server method,
  error code, output format, or remote configuration is added.
- **Schema/protocol/data/security:** none. Candidate parts are private in-memory
  test values, not bytes, a format, a checksum, or a credential. No schema,
  protocol, address, trust, key, stored data, reader/writer, or migration
  contract is created.
- **Semantic IDs and runtime:** none. Existing Actor type/schema evidence and
  owned Actor values are observed without changing the compiler, evaluator,
  scheduler, bytecode, VM, ABI, package, dependency, or backend.
- **Determinism and Unicode:** the matrix asserts only local reconstruction and
  private structural distinctions. Unicode remains 17.0.0 and original UTF-8
  spans remain authoritative.

## Unresolved alternatives

- RFC-0009 or replacement authority must define the public Envelope type,
  canonical bytes/field order/encoding, required/optional/extension rules,
  protocol-version negotiation, unknown-field/version behavior, remote
  identity/incarnation and schema/Program/Semantic ID bindings.
- Message authority must define message/correlation ID allocation and scope,
  deadline units/clock/expiry, cancellation propagation/races, delivery policy,
  ordering, retry, duplicate/deduplication, replay, and Fault behavior.
- Security/resource authority must define authentication metadata, Capability
  lifecycle, trust, privacy, redaction, integrity/checksum algorithm and scope,
  compression/framing, size/depth limits, quotas, denial-of-service behavior,
  diagnostics, and failure observability.
- Transport adapters, endpoint integration, public schemas/protocol records,
  migration and compatibility, cross-process/backend/platform fixtures, and
  Stable support remain future work. Private candidate wrappers are not
  reserved public names or candidate wire layouts.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
