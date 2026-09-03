# DEC-0282: Private Task replay-player evidence / 私有 Task Replay Player 证据

> 状态：Accepted<br>
> 提出日期：2026-09-03<br>
> 决定日期：2026-09-03<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：DEC-0107 | DEC-0267 | DEC-0280 | DEC-0281 | GAP-DETERMINISTIC-REPLAY-001 | REP-2504<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable package that can close an
internal Experimental REP-2504 baseline over the existing private DEC-0267
Task trace and strict fresh-runtime replay path. It deliberately does not
define a checkpoint, persisted log reader, public Replay Player, CLI command,
wire protocol, privacy policy, migration rule, or cross-process guarantee.

本决定基于既有私有 DEC-0267 Task trace 与严格的 fresh-runtime replay 路径，定义可
完成内部 Experimental REP-2504 基线的最小可执行证据包。它刻意不定义 checkpoint、
持久化日志 reader、公开 Replay Player、CLI 命令、wire protocol、隐私策略、迁移规则或
跨进程保证。

## Question

What exact crate-private evidence may exercise the existing checked Task replay
path, including validation, recipe binding, deterministic reconstruction, and
first divergence, without interpreting private trace bytes as a checkpoint or
authorizing the unresolved public Replay contracts?

## Decision

1. **Scoped authority.** This decision authorizes only one crate-private,
   `cfg(test)` REP-2504 executable evidence matrix in `ling-eval`. It may call
   the existing DEC-0267 `run_task_schedule` and `replay_task_schedule` APIs and
   reuse complete private mutation assertions. It adds no production runtime
   transition, player type, reader, restore hook, public API, or protocol and
   does not close `GAP-DETERMINISTIC-REPLAY-001`.

2. **Existing replay boundary.** The only executable player-like boundary in
   scope is strict in-memory Task replay. It validates one existing
   `TaskScheduleTrace`, reconstructs a fresh checked Task runtime from the
   caller-supplied recipe, consumes the recorded private scheduling choices and
   deterministic inputs, and compares the complete resulting trace. This is
   not Event Log interpretation or checkpoint restoration.

3. **Checked-only recipe.** Every trace and replay recipe must originate from
   source that completes Source -> CST -> AST -> HIR -> resolution -> type
   checking -> Effect/Capability checking. The evidence cannot fabricate a
   `CheckedProgram`, Task Core, Task machine, runtime identity, trace, terminal,
   Fault, or unchecked-AST execution route.

4. **Exact preflight order.** Replay must first execute the complete existing
   structural trace validation. Only a valid trace may reach private runtime
   recipe identity comparison. The identity must include the Accepted DEC-0012
   Body ID of every reachable checked Task in addition to existing Task
   Core/machine and argument bytes; its private domain advances from
   `ling.task-runtime-recipe/0` to `ling.task-runtime-recipe/1`. A changed
   checked Task closure, root Task, or argument must fail at private event `0` with
   `runtime_identity_mismatch`; no scheduling or host action may execute before
   that rejection. This private string is test evidence, not a public
   diagnostic or compatibility code.

5. **Source reconstruction independence.** Equivalent checked programs rebuilt
   across logical source-name changes, `SourceId` changes, and LF versus
   BOM/CRLF spelling must satisfy the same private recipe identity and replay
   to identical private canonical trace bytes. Physical paths, source IDs, and
   original source spelling cannot enter logical replay equality; original
   UTF-8 spans remain authoritative sidecar evidence for Faults.

6. **Exact case set.** The matrix contains exactly these bounded case families:
   `validated-trace-exact-replay`, `checked-recipe-preflight`,
   `first-event-divergence`, `fault-and-cancellation-replay`, and
   `deferred-checkpoint-and-public-surface-absence`. New meanings or case
   families require separate Accepted authority.

7. **Exact successful replay.** The first case produces a validated finite Task
   trace from immutable Checked Core and explicit arguments, scheduler/runtime
   limits, deadlines, seed, and host script. A source-equivalent reconstructed
   program must replay every recorded choice without seed fallback and produce
   identical private canonical trace bytes and terminal evidence.

8. **Recipe preflight.** The second case accepts only the equivalent checked
   recipe and rejects changed arguments, changed Task Core behavior, and a
   changed root Task at private event `0`. Rejection must occur before runtime
   event or host observation. No public Program ID, profile, target, toolchain,
   Capability, config, or message-schema compatibility contract is inferred.

9. **First divergence.** The third case directly executes the complete existing
   DEC-0267 mutation assertions for selected Task, step kind, tick, ready-set
   canonicality, deadline application, host text/outcome, and terminal value.
   Structurally invalid traces fail validation; valid mutations report the
   first mismatching private event. The evidence defines no recovery or
   best-effort playback.

10. **Fault and cancellation reconstruction.** The fourth case replays finite
    host-Fault and deadline-cancellation traces from fresh runtimes. Terminal,
    cleanup, Fault category/operation/detail, host event, and complete canonical
    trace equality must match. It does not define external cancellation,
    partial persistence, checkpoint recovery, retry, or supervision replay.

11. **DEC-0107 disposition.** The fifth case retains all eleven provisional
    DEC-0107 concerns exactly once and assigns only these private dispositions:

    - existing bounded Task replay evidence: `program-canonical-bytes`,
      `preflight-binding`, `event-application`, `ordering`, `divergence`,
      `fault`, and `cancellation`; here `event-application` means only fresh-
      runtime re-execution under recorded choices, and `program-canonical-bytes`
      means only the existing private Task runtime recipe identity;
    - deferred public contract: `checkpoint-identity`, `privacy`, `integrity`,
      and `migration`.

    Deferred concerns receive no placeholder value, guessed format, empty
    default, checksum, redaction rule, or compatibility version.

12. **No checkpoint or reader claim.** A private in-memory trace is not a
    checkpoint, Effect Log, persisted input, public schema, or trusted external
    document. The evidence does not decode `canonical_bytes()`, restore heap or
    Actor state, seek to an event, resume after a partial trace, or accept
    untrusted bytes.

13. **Private mismatch boundary.** `TaskReplayError` event IDs and reason text
    remain private DEC-0267 test/runtime evidence. DEC-0282 allocates no
    `L-REPLAY-*` code, bilingual diagnostic, JSON error object, exit status,
    repair, telemetry field, or compatibility promise.

14. **Bounds and privacy.** Sources, values, tasks, deadlines, host responses,
    traces, events, and comparisons are fixed finite repository fixtures under
    explicit existing limits. Tests read no environment variable, wall clock,
    entropy source, network, external file, device, credential, or personal
    data and create no retention, authorization, encryption, or redaction
    claim.

15. **Negative surface evidence.** The fifth case must prove that no production
    Replay Player/checkpoint/log reader, restore or seek API, payload decoder,
    integrity verifier, redaction/migration adapter, CLI command, diagnostic,
    schema registry entry, public fixture protocol, or implemented
    `PROTO-REPLAY` record is created. `PROTO-REPLAY` remains Future,
    unversioned, schema-less, and unimplemented.

16. **Public boundary.** No Ling syntax, value, type, Effect, Capability,
    Task/Actor semantic promise, CLI/REPL/LSP/editor route, public Rust API,
    diagnostic, schema, Semantic ID, protocol, package/ABI, bytecode, VM,
    Native/Wasm, remote behavior, stored data, migration, dependency, or Stable
    support is added.

17. **Completion boundary.** REP-2504 is Done only for this internal
    Experimental baseline when all five exact cases execute against real
    validated traces and fresh checked runtimes, all eleven dispositions are
    complete and duplicate-free, negative public-surface assertions pass,
    focused and full repository gates pass, evidence is bound to a commit, and
    status/backlog/gap records are synchronized. Existing tests may be reused
    only by directly executing their complete assertions.

18. **Deferred public player.** Checkpoint contents and identity, public
    Program/Schema/profile/target/toolchain/Capability/config/message-schema
    binding, encoded log input, event application, privacy, integrity,
    corruption, authorization, diagnostics, resource policy, reader/writer
    compatibility, migration, partial replay, cross-process/backend behavior,
    and Stable support remain blocked pending Accepted RFC-0010 or replacement
    authority and REP-2505/REP-2506.

## Conformance plan

- Add one dedicated private `ling-eval` evidence module with the exact
  five-case table and complete eleven-concern disposition inventory.
- Produce validated traces from checked Task sources and replay them against
  fresh source-equivalent runtimes reconstructed across Unicode, BOM/CRLF,
  logical source identity, host-Fault, and deadline-cancellation variants.
- Reject changed checked behavior, root, and arguments during private recipe
  preflight; directly execute complete mutation assertions for structural and
  first-event divergence behavior.
- Assert no checkpoint decoding, persistence, public player, CLI, diagnostic,
  schema, fixture protocol, or implemented Replay protocol is added.
- Run focused `ling-eval` tests and strict Clippy, retained Task/Actor and VM
  differential gates, the full locked/offline workspace suite, governance/
  status/docs/RC0 gates, formatting, and diff checks before marking REP-2504
  Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, protocols,
  package/ABI versions, stored data, bytecode/VM/backends, dependencies, and
  migration: none; this decision authorizes private `cfg(test)` evidence only.
- Runtime: no production transition, player/checkpoint/log-reader type, restore
  path, decoder, or public API is added. Tests execute only existing Accepted
  DEC-0267 Task trace and replay routes. The publish-disabled test scheduler's
  opaque runtime-recipe identity advances from `/0` to `/1` so reachable
  DEC-0012 Body IDs participate in preflight; no persisted reader or migration
  obligation exists for the previous in-memory identity.
- Replay/determinism: the matrix verifies bounded fresh-runtime reconstruction
  without defining an Effect Log, public player, checkpoint, integrity,
  privacy, or compatibility relation. Unicode remains 17.0.0 and original
  UTF-8 byte spans remain authoritative.

## Unresolved alternatives

- Treating `TaskScheduleTrace::canonical_bytes()` as a decodable public log or
  checkpoint is rejected because it is an internal one-way fixture projection.
- Adding a `ling replay` command around the in-memory Task helper is rejected;
  no Accepted input schema, diagnostics, privacy, integrity, or migration
  contract exists.
- Best-effort continuation after structural or event mismatch is rejected for
  this evidence; the existing strict path reports the first private divergence.
- Public checkpoint, log reader, player, and compatibility work remains
  RFC-0010 and REP-2505/REP-2506 scope.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
