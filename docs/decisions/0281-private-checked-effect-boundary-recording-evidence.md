# DEC-0281: Private checked Effect-boundary recording evidence / 私有 Checked Effect 边界记录证据

> 状态：Accepted<br>
> 提出日期：2026-09-03<br>
> 决定日期：2026-09-03<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：RFC-0006 | DEC-0106 | DEC-0260 | DEC-0261 | DEC-0280 | GAP-DETERMINISTIC-REPLAY-001 | REP-2503<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable package that can close an
internal Experimental REP-2503 baseline over the accepted checked Handler and
host-capability boundary. It deliberately does not define a production Effect
recorder, a public recordable-Effect policy, an Effect Log, replay payloads, or
privacy and compatibility behavior.

本决定基于已接受的 checked Handler 与 host Capability 边界，定义可完成内部
Experimental REP-2503 基线的最小可执行证据包。它刻意不定义 production Effect
recorder、公开的可记录 Effect 策略、Effect Log、Replay payload、隐私或兼容性行为。

## Question

What exact crate-private evidence may observe real checked Effect operations
that escape lexical handlers and reach an injected host adapter, including
ordered success and failure, without treating the observation as a public
recorder hook or deciding the unresolved Clock, Random, external-input,
scheduler, payload, privacy, and Replay contracts?

## Decision

1. **Scoped authority.** This decision authorizes only one crate-private,
   `cfg(test)` REP-2503 executable evidence matrix in `ling-eval`. It may wrap
   the existing injected `Console` test adapter and execute behavior already
   fixed by Accepted RFC-0006, DEC-0260, and DEC-0261. It adds no production
   transition, recorder trait, event sink, public API, or protocol and does not
   close `GAP-DETERMINISTIC-REPLAY-001`.

2. **Only executable probe.** `Console.write text` is currently the only source
   producer that can perform a checked user-visible operation through a host
   adapter. The probe therefore observes only the exact accepted
   `Console.Write.write(Text) -> Unit :: Once` host-escape boundary. This is a
   boundary probe, not a decision that Console output belongs in a future
   public Effect Log.

3. **Checked-only source.** Every observation must originate from source that
   completes Source -> CST -> AST -> HIR -> resolution -> type checking ->
   Effect/Capability checking -> `ProgramSnapshot`, then executes through the
   real checked interpreter. The matrix cannot fabricate `CheckedProgram`,
   `HandlerCore`, operation identity, Runtime Fault, or unchecked AST
   execution.

4. **Observation point.** The test adapter is downstream of accepted lexical
   Handler dispatch. It records an attempt only when `Console.Write.write`
   actually reaches the injected host boundary. An operation consumed by the
   nearest matching handler produces no host observation; an operation
   performed by a selected clause remains outside that selected handler and is
   observed only if no outer handler consumes it.

5. **Exact private record.** Each finite in-memory observation contains only a
   consecutive test-local ordinal, the constant canonical operation identity
   `Console.Write.write`, the exact bounded canonical-LF host text supplied to
   `Console::write`, and either `Succeeded(Unit)` or
   `Failed(HostErrorCategory)`. A corresponding Runtime Fault category,
   operation, and original UTF-8 span may be compared as sidecar evidence but
   is not stored in the observation. No record type or field is exported,
   encoded, hashed, persisted, registered, or treated as a Replay payload.

6. **Attempt and failure boundary.** One adapter invocation creates exactly
   one observation, including when the scripted adapter returns a structured
   failure. A failing host operation terminates checked execution through the
   accepted Runtime Fault route, so no later source operation is observed.
   This test-local rule does not define recorder crash, flush, retry,
   backpressure, partial-write, or recovery semantics.

7. **Exact case set.** The matrix contains exactly these bounded case
   families: `escaped-success-order`,
   `handled-elision-and-clause-escape`, `failure-stop-and-fault-sidecar`,
   `checked-reconstruction-and-source-independence`, and
   `deferred-boundaries-and-public-surface-absence`. New observation meanings
   or case families require separate Accepted authority.

8. **Escaped success order.** The first case executes two unhandled checked
   Console operations and requires two consecutive successful observations in
   strict left-to-right source order with the accepted canonical LF host text.
   Pure computation between them produces no observation.

9. **Handler elision and clause escape.** The second case exercises direct,
   transitive, resumed, and nested accepted Handler paths. Body operations
   intercepted by the nearest matching handler are absent; a bounded clause
   operation that escapes all outer handlers appears once at the correct
   position. `State<T>` mutations and continuation mechanics are checked by
   their existing runtime evidence and never become recorder events here.

10. **Failure and sidecar.** The third case scripts one success, one
    `BrokenPipe` failure, and a source operation after the failure. It requires
    exactly two ordered observations, no third adapter invocation, and the
    accepted `L-RUNTIME-0001` host-capability Fault projection at the original
    failing operation span. Host error prose, OS codes, and debug text are not
    retained.

11. **Reconstruction and source independence.** The fourth case rebuilds an
    equivalent checked fixture across logical source-name changes and
    LF/BOM/CRLF spellings. Each execution must produce the same bounded logical
    observation projection, while original source spans remain correct
    sidecars for the corresponding bytes. Physical paths and `SourceId` values
    cannot enter equality.

12. **Deferred-boundary disposition.** The fifth case keeps all six DEC-0106
    provisional names duplicate-free and explicit:

    - `Clock` and `Random` have accepted checked operation contracts but no
      accepted source/Core producer or runtime host boundary;
    - `ExternalInput`, `NetworkReceive`, and `FileDeviceRead` remain
      plan-only vocabulary without accepted operation or runtime behavior; and
    - `SchedulingNondeterminism` remains private Task/Actor scheduling evidence,
      not a checked Effect operation or recorder hook.

    None receives a placeholder record, guessed payload, default outcome, or
    alias to the Console probe.

13. **Bounds and privacy.** Every source, literal host value, adapter response,
    observation, handler path, and comparison is a fixed finite test fixture.
    Raw payload evidence is limited to non-sensitive repository literals and
    remains in process. The matrix reads no environment variable, clock,
    entropy source, network, external file, device, credential, or personal
    data and creates no redaction, retention, encryption, or privacy claim.

14. **Forbidden observations.** Paths, physical source identity, wall time,
    duration, thread or worker identity, addresses, allocation layout,
    hash-map order, panic/debug text, host locale, platform error text, and
    unspecified scheduler metrics cannot enter the observation or select an
    outcome.

15. **Negative surface evidence.** The fifth case must also prove that no
    production recorder trait/struct/hook, exported event model, Effect Log,
    encoder/decoder, payload schema, checksum, redaction field, migration
    adapter, build/Semantic Graph/Audit field, CLI command, diagnostic, schema
    registry entry, fixture protocol, or implemented `PROTO-REPLAY` record is
    created. Existing `PROTO-REPLAY` remains `Future`, unversioned,
    schema-less, and unimplemented.

16. **Task, Actor, VM, and public boundary.** This slice does not observe Task
    or Actor scheduling, serialize continuations, or install the same adapter
    in bytecode/VM production code. Existing interpreter/VM Handler
    differential suites remain regression gates, not evidence of a shared
    recorder ABI. No Ling syntax, type, Effect, Capability, CLI/LSP/editor
    route, public Rust API, diagnostic, schema, Semantic ID, protocol,
    package/ABI, bytecode, VM, Native/Wasm, remote, stored-data, migration, or
    Stable behavior is added.

17. **Completion boundary.** REP-2503 is Done only for this internal
    Experimental baseline when all five exact cases execute against real
    checked interpreter paths, the six deferred dispositions are complete and
    duplicate-free, negative public-surface assertions pass, focused and full
    repository gates pass, evidence is bound to a commit, and status/backlog/
    gap records are synchronized. Existing tests may be reused only by
    directly executing their complete assertions.

18. **Deferred public recorder.** Public recordability, Clock/Random/external
    input/network/file/device producers, scheduler capture, operation/event
    identity across Task/Actor boundaries, payload/result/Fault encoding,
    buffering/flush/backpressure, limits, privacy/redaction/retention,
    integrity, corruption/divergence, diagnostics, reader/writer migration,
    checkpoints, cross-process/backend behavior, and Stable support remain
    blocked pending Accepted RFC-0010 or replacement authority and REP-2504
    through REP-2506.

## Conformance plan

- Add one dedicated private `ling-eval` evidence module with the exact
  five-case table, a bounded scripted `Console` adapter, and the complete six-
  boundary deferred inventory.
- Compile every positive and negative source through the ordinary checked
  pipeline; exercise escaped success order, nearest/nested/deep Handler
  interception, clause escape, structured failure stop, and Runtime Fault span
  sidecars.
- Reconstruct equivalent Unicode/BOM/CRLF/logical-source variants and compare
  only the exact bounded logical observations while preserving original byte
  spans outside the record.
- Assert no unsupported boundary gets a record or alias and no production or
  public recorder/Effect Log/Replay/schema/diagnostic/protocol surface exists.
- Run focused `ling-eval` tests and strict Clippy, retained interpreter/VM
  Handler differential gates, the full locked/offline workspace suite,
  governance/status/docs/RC0 gates, formatting, and diff checks before marking
  REP-2503 Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, protocols,
  package/ABI versions, stored data, bytecode/VM/backends, dependencies, and
  migration: none; this decision authorizes private `cfg(test)` evidence only.
- Runtime: no production transition, recorder, hook, event type, buffer, or
  public API is added. Tests invoke the existing accepted `Console` adapter
  boundary after checked Handler dispatch.
- Determinism/privacy: observations use finite repository literals and explicit
  scripted outcomes without defining a public Effect Log or retention policy.
  Unicode remains 17.0.0 and original UTF-8 byte spans remain authoritative.

## Unresolved alternatives

- Instrumenting interpreter/VM internals before Handler dispatch is rejected
  because it would record operations that accepted handlers consume and would
  invent a production recorder hook.
- Treating Console output as a publicly recordable replay input is rejected;
  this decision uses it only as the existing executable host-boundary probe.
- Clock, Random, external input, network/file/device reads, scheduler choices,
  Task/Actor messages, payload schemas, privacy, integrity, failure recovery,
  and a shared interpreter/VM recorder ABI remain RFC-0010 and later REP work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
