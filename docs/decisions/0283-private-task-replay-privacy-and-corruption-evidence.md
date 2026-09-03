# DEC-0283: Private Task replay privacy and corruption evidence / 私有 Task Replay 隐私与损坏证据

> 状态：Accepted<br>
> 提出日期：2026-09-04<br>
> 决定日期：2026-09-04<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：DEC-0108 | DEC-0267 | DEC-0280 | DEC-0282 | GAP-DETERMINISTIC-REPLAY-001 | REP-2505<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable REP-2505 package that can
characterize privacy and corruption risks of the existing private DEC-0267
Task trace. It deliberately demonstrates raw-payload retention instead of
claiming redaction, and it treats truncation/mutation rejection as private
validation evidence rather than a public checksum or recovery protocol.

本决定定义 REP-2505 可执行证据的最小边界，用于刻画现有私有 DEC-0267 Task trace
的隐私与损坏风险。它明确证明 raw payload 仍被保留，而不是声称已经 redaction；它把
截断/变异拒绝视为私有验证证据，而不是公开 checksum 或恢复协议。

## Question

What exact crate-private evidence may demonstrate that the current in-memory
Task trace is unredacted, fail closed for bounded truncation and mutation, and
remain reproducible from explicit inputs, without inventing the unresolved
privacy, trimming, checksum, retention, key, schema, or migration contracts?

## Decision

1. **Scoped authority.** This decision authorizes only one crate-private,
   `cfg(test)` REP-2505 executable evidence matrix in `ling-eval` plus one
   complete private trace-prefix/gap validation assertion. It adds no
   production privacy transition, redactor, trimmer, chunk, checksum, decoder,
   retention store, key manager, public API, command, or protocol and does not
   close `GAP-DETERMINISTIC-REPLAY-001`.

2. **Existing trace only.** Evidence may exercise only validated in-memory
   DEC-0267 Task traces and the strict DEC-0282 fresh-runtime replay path. A
   private trace is not a checkpoint, Effect Log, persisted input, trusted
   external document, or public replay schema.

3. **Checked-only origin.** Every trace must originate from source that
   completes Source -> CST -> AST -> HIR -> resolution -> type checking ->
   Effect/Capability checking. The evidence cannot fabricate Checked Core,
   runtime identity, Task state, events, Faults, or an unchecked-AST execution
   route.

4. **No privacy classification.** The matrix may use one repository-local
   Unicode sentinel explicitly named as a privacy-boundary fixture. That
   sentinel is not secret, PII, a sensitivity label, or a policy category. Its
   sole purpose is to prove that current private Host and terminal payloads and
   canonical fixture bytes are raw and unredacted.

5. **Exact case set.** The matrix contains exactly these case families:
   `raw-payload-retention-boundary`, `prefix-truncation-refusal`,
   `validated-mutation-refusal`, `explicit-input-offline-reconstruction`, and
   `deferred-privacy-integrity-surface-absence`. New meanings or case families
   require separate Accepted authority.

6. **Raw-payload risk evidence.** The first case must prove that the sentinel
   appears unchanged in the typed Host event and private canonical trace bytes.
   It must also prove that the logical source name is excluded from those
   bytes. This is evidence that the trace is not privacy-safe for persistence
   or disclosure; it is not field classification or redaction.

7. **Strict-prefix refusal.** The second case must directly execute one
   complete Task-scheduler assertion that every strict event prefix, including
   the empty prefix, is invalid because it lacks the required finite closure.
   Removing any non-closure event while retaining the remainder must also be
   rejected. This proves fail-closed validation for bounded truncation/gaps,
   not dependency-preserving trimming or partial replay.

8. **Mutation refusal.** The third case must directly execute the complete
   existing structural and first-divergence assertions for version, event
   identity, closure, ready-set, selected Task, step, tick, deadline, Host text
   and outcome, and terminal value. It defines no checksum, authenticity,
   recovery, repair, or best-effort continuation.

9. **Explicit-input hermetic evidence.** The fourth case reconstructs an
   equivalent checked program across logical source-name, `SourceId`, LF, and
   BOM/CRLF changes and replays from explicit finite arguments, seed, limits,
   deadlines, and injected Host responses. Identical canonical trace bytes
   prove only this in-process hermetic test route; they do not prove an offline
   CLI, process sandbox, or cross-process equivalence.

10. **DEC-0108 disposition.** The fifth case retains all sixteen provisional
    DEC-0108 concerns exactly once and assigns only these private dispositions:

    - raw-payload risk evidence: `field-sensitivity`, meaning only that a raw
      fixture payload exists and requires a future policy;
    - private fail-closed trace evidence: `truncation`, `corruption`, and
      `failure-diagnostics`, where diagnostics means private error reason/event
      evidence only;
    - explicit-input hermetic test evidence: `offline-mode`;
    - deferred public contract: `field-redaction`, `secret-pii-exclusion`,
      `capability-resource-exclusion`, `authorization`, `key-handling`,
      `retention`, `dependency-closure`, `chunk-boundary`,
      `checksum-integrity`, `unknown-field`, and `migration`.

    These dispositions are traceability facts, not an implemented privacy or
    integrity policy.

11. **No trimming claim.** Rejecting a strict prefix or a gapped event sequence
    is not log trimming. The matrix must not compute dependency closure,
    rewrite references, preserve a checkpoint boundary, renumber a public log,
    or claim that any retained subset is replayable.

12. **No checksum claim.** Structural validation and exact replay comparison
    are not integrity checksums, signatures, MACs, authentication, provenance,
    or corruption recovery. No algorithm, scope, chunk framing, trust root, or
    public error code is inferred.

13. **Private failure boundary.** `TaskSchedulerError` and `TaskReplayError`
    reason/event fields remain private runtime/test evidence. DEC-0283 allocates
    no `L-REPLAY-*` code, bilingual diagnostic, exit status, JSON object,
    repair, telemetry field, or compatibility promise.

14. **Bounds and data safety.** Sources, sentinels, arguments, traces,
    deadlines, Host responses, mutations, and comparisons are fixed finite
    repository fixtures under explicit existing limits. Tests read no
    credential or personal data and create no logging, retention, deletion,
    authorization, encryption, redaction, or incident-response claim.

15. **Negative surface evidence.** The fifth case must prove that no production
    privacy policy, sensitivity type, redactor, trimmer, chunk/checksum,
    retention/key component, decoder/verifier, CLI command, diagnostic, schema,
    public fixture protocol, or implemented `PROTO-REPLAY` record is created.
    `PROTO-REPLAY` remains Future, unversioned, schema-less, and unimplemented.

16. **Public boundary.** No Ling syntax, value, type, Effect, Capability,
    Task/Actor semantic promise, CLI/REPL/LSP/editor route, public Rust API,
    diagnostic, schema, Semantic ID, protocol, package/ABI, bytecode, VM,
    Native/Wasm, remote behavior, stored data, migration, dependency, or Stable
    support is added.

17. **Completion boundary.** REP-2505 is Done only for this internal
    Experimental baseline when all five exact cases execute over real checked
    traces, the complete prefix/gap and mutation assertions run, all sixteen
    dispositions are complete and duplicate-free, negative surface assertions
    pass, repository gates pass, evidence is bound to a commit, and status,
    backlog, and gap records are synchronized.

18. **Deferred public work.** Sensitivity taxonomy; default allow/deny;
    secret/PII/Capability/Resource handling; redaction bytes; authorization;
    encryption and keys; retention/deletion; dependency-preserving trimming;
    chunk framing; checksum/integrity/authenticity; truncation/corruption
    taxonomy and recovery; unknown fields; diagnostics; resource policy;
    reader/writer compatibility; migration; public offline tools;
    cross-process/backend behavior; and Stable support remain blocked pending
    Accepted RFC-0010 or replacement authority. REP-2506 requires separate
    Accepted authority.

## Conformance plan

- Add one dedicated private `ling-eval` module with the exact five-case table
  and complete sixteen-concern disposition inventory.
- Produce checked Task traces containing only a repository-local Unicode
  sentinel and demonstrate raw retention without interpreting it as secret,
  PII, or a policy label.
- Directly execute complete prefix/gap validation and existing mutation/
  first-divergence assertions; reconstruct an equivalent trace only from
  explicit bounded in-memory inputs.
- Assert no production privacy/redaction/trimming/chunk/checksum/retention/key/
  decoder surface, CLI, diagnostic, schema, or implemented Replay protocol is
  added.
- Run focused `ling-eval` tests and strict Clippy, retained Task/Actor and VM
  differential gates, full locked/offline workspace gates, governance/status/
  docs/RC0 checks, formatting, and diff checks before marking REP-2505 Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, protocols,
  package/ABI versions, stored data, bytecode/VM/backends, dependencies,
  migration, and Unicode 17.0.0: unchanged.
- Runtime: no production privacy, redaction, trimming, checksum, persistence,
  decoding, recovery, or public Replay transition is added. The only scheduler
  addition is a `cfg(test)` complete prefix/gap validation assertion.
- Replay/privacy: evidence confirms raw private payload retention and
  fail-closed validation. It deliberately makes no privacy, integrity,
  authenticity, trimming, offline-tool, or compatibility promise.

## Unresolved alternatives

- Treating raw private canonical bytes as a redacted or safe persistence format
  is rejected; the sentinel proves the opposite.
- Naming structural validation a checksum or integrity protocol is rejected;
  it has no accepted algorithm, framing, threat model, or trust boundary.
- Treating prefix refusal as dependency-preserving trimming is rejected; no
  dependency graph or partial replay contract exists.
- Public privacy, trimming, corruption handling, and offline data tooling
  remain RFC-0010 and later authority work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
