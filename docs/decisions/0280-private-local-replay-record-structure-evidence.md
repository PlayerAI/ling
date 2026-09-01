# DEC-0280: Private local replay-record structure evidence / 私有本地 Replay Record 结构证据

> 状态：Proposed<br>
> 提出日期：2026-09-01<br>
> 决定日期：Pending<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：DEC-0105 | DEC-0267 | DEC-0279 | GAP-DETERMINISTIC-REPLAY-001 | REP-2502<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision proposes the smallest executable structure-evidence package that
can close an internal Experimental REP-2502 baseline over the existing private
DEC-0267 Task trace. It deliberately does not define a public Replay log schema,
Effect Log, encoder/decoder, checksum, privacy policy, or compatibility format.

本决定提议基于既有私有 DEC-0267 Task trace，建立可完成内部 Experimental REP-2502
基线的最小可执行结构证据包。它刻意不定义 public Replay log schema、Effect Log、
encoder/decoder、checksum、privacy policy 或 compatibility format。

## Question

What exact crate-private, non-serialized matrix may bind REP-2502's proposed
schema concerns to fields and validation behavior already present in a real
`TaskScheduleTrace`, while preventing those observations from becoming a
public Replay envelope or resolving the open wire/privacy/migration contract?

## Decision

1. **Scoped authority.** This decision authorizes only one crate-private,
   `cfg(test)` REP-2502 executable structure-evidence matrix in `ling-eval`.
   It may inspect and validate behavior already fixed by Accepted DEC-0267 and
   reuse complete DEC-0279 Task evidence assertions. It adds no production
   schema model, runtime transition, public API, or protocol and does not close
   `GAP-DETERMINISTIC-REPLAY-001`.

2. **Evidence source.** Every positive record comes from a successfully
   validated in-memory `TaskScheduleTrace` produced by the real DEC-0267 test
   scheduler from immutable Checked Core, exact arguments, explicit bounds,
   logical deadlines, and a deterministic host script. Hand-constructed field
   lists, documentation names, or unvalidated bytes are not execution
   evidence.

3. **Not an Effect Log.** `TaskScheduleTrace`, its private version marker, and
   `canonical_bytes()` remain a publish-disabled in-process test boundary.
   They are not the `EffectLog` named by `SEMANTICS.md`, the future
   `PROTO-REPLAY`, a public schema, or a compatibility promise. The matrix must
   not export, persist, decode, migrate, or register those bytes.

4. **Exact concern inventory.** The matrix records exactly the thirteen
   vocabulary concerns retained by Accepted DEC-0105: `canonical-envelope`,
   `event-id`, `event-kind`, `ordering`, `identity`, `checksum`,
   `determinism-class`, `toolchain`, `profile`, `schema`, `payload`,
   `migration`, and `privacy`. These are test-local traceability labels, not
   field names, tags, ordinals, JSON keys, or a wire-schema registry.

5. **Concern disposition.** The inventory must distinguish existing private
   trace evidence from unresolved public concerns:

   - private trace evidence exists only for `canonical-envelope`, `event-id`,
     `event-kind`, `ordering`, `identity`, `schema`, and `payload`; `schema`
     here means only the existing private trace-version check;
   - `checksum`, `determinism-class`, `toolchain`, `profile`, `migration`, and
     `privacy` remain explicitly deferred and must not receive guessed values,
     empty placeholders, default tags, or inferred encodings.

   No disposition creates an optional public field or permits upgrading the
   trace to a Replay format.

6. **Evidence case set.** The matrix contains exactly these bounded case
   families: `validated-private-envelope-projection`,
   `event-identity-kind-order-projection`,
   `typed-payload-terminal-projection`, `mutation-and-limit-rejection`, and
   `public-replay-schema-absence`. New record meanings or case families require
   separate Accepted authority.

7. **Envelope projection.** The first case observes only the trace's existing
   private version, scheduler/runtime configuration, runtime identity,
   canonicalized deadlines, deterministic host script, finite event count, and
   canonical bytes. Repetition and equivalent Unicode/BOM/CRLF/source-identity
   reconstruction must validate and produce identical bytes. No filesystem
   path or physical source identity enters the projection.

8. **Event identity, kind, and order.** The second case asserts consecutive
   event IDs, monotonic logical ticks, canonical ready/deadline Task paths,
   explicit event kinds, and exactly one terminal closure over a finite trace.
   This is only DEC-0267's local event order; it does not define cross-process,
   Actor-message, external-Effect, or production worker ordering.

9. **Typed payload projection.** The third case exercises bounded typed
   `TaskValue`, host success/failure outcome, terminal state, Fault summary,
   and exactly-once cleanup observations already present in the private trace.
   Source names and original UTF-8 spans remain sidecar evidence and are not
   serialized into logical equality. Resource, Managed, Capability, arbitrary
   bytes, nested public values, and Actor messages are outside this slice.

10. **Mutation and limits.** The fourth case directly executes complete
    DEC-0267 validation/replay assertions for unsupported version, malformed or
    nonconsecutive event identity, invalid ordering/closure, changed selection,
    step, tick, deadline, host outcome, terminal result, and every zero resource
    bound. Rejection must identify the first affected private event or fail
    before a run; it creates no public corruption diagnostic or recovery rule.

11. **No checksum claim.** Equality of private canonical bytes is not an
    integrity checksum, signature, corruption detector, authenticity proof, or
    privacy mechanism. The matrix must not hash the trace and call that value a
    checksum, because checksum algorithm, scope, keying, framing, and migration
    are unresolved.

12. **Bounds and forbidden observations.** Inputs, deadlines, host responses,
    ready sets, event collections, payloads, Fault summaries, and cleanup sets
    are explicitly finite. Paths, source IDs, wall time, duration, thread or
    worker identity, addresses, allocation layout, hash-map order, panic/debug
    text, host locale, and unspecified scheduler metrics cannot enter a retained
    projection or determine a concern disposition.

13. **Negative surface evidence.** The fifth case must prove that no
    production/public Replay envelope or event model, Effect Log, encoder,
    decoder, reader/writer version, checksum/integrity algorithm, redaction or
    privacy field, migration adapter, source annotation, build/Semantic
    Graph/Audit/header field, CLI command, diagnostic, schema-registry entry,
    fixture, or implemented protocol-inventory record is created. Existing
    `PROTO-REPLAY` must remain `Future`, unversioned, schema-less, and
    unimplemented.

14. **Public boundary.** No Ling syntax, value, type, Effect, Capability,
    Actor/Task semantic promise, CLI/REPL/LSP/editor route, public Rust API,
    diagnostic, schema, Semantic ID, protocol, package/ABI, bytecode, VM,
    Native, Wasm, remote behavior, stored data, migration, or Stable support is
    added.

15. **Completion boundary.** REP-2502 is Done only for this internal
    Experimental baseline when all five exact cases execute against real
    validated DEC-0267 traces, the thirteen concern dispositions are complete
    and duplicate-free, mutation/limit and negative boundaries pass, focused
    and full repository gates pass, evidence is bound to a commit, and
    status/backlog/gap records are synchronized. Existing tests may be reused
    only by directly executing their complete assertions.

16. **Deferred public schema.** Canonical public envelope and encoding, field
    types/tags/optionality, event identity and ordering across effects and
    actors, payload serialization, checksum/integrity, class/toolchain/profile
    metadata, privacy/redaction/retention, corruption/divergence, resource
    limits, reader/writer compatibility, unknown fields, migration, fixtures,
    cross-process/backend behavior, and Stable support remain blocked pending
    Accepted RFC-0010 or replacement authority and REP-2503 through REP-2506.

## Conformance plan

- Add one dedicated private `ling-eval` structure-evidence module with an exact
  five-case table and the complete thirteen-concern disposition inventory.
- Produce validated success and host-Fault traces with explicit seeds,
  arguments, deadlines, limits, and host scripts; reconstruct them across
  Unicode/BOM/CRLF/source identity and compare only accepted private bytes and
  bounded projections.
- Assert consecutive event IDs, logical ordering, typed payload/outcome/Fault,
  terminal closure, and exactly-once cleanup through public DEC-0267 accessors.
- Directly reuse complete private DEC-0267 mutation, validation, replay, and
  zero-limit assertions; names alone are not evidence.
- Add bounded source/module/schema/protocol inventory assertions for the absent
  production and public Replay surfaces, including the unchanged Future
  `PROTO-REPLAY` record.
- Run focused `ling-eval` tests and strict Clippy, retained CLI Task/Actor
  boundaries, the full locked/offline workspace suite, governance/status/docs/
  RC0 gates, formatting, and diff checks before marking REP-2502 Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, protocols,
  package/ABI versions, stored data, bytecode/VM/backends, dependencies, and
  migration: none; this proposal authorizes private `cfg(test)` evidence only.
- Runtime: no production transition, schema type, recorder, writer, reader, or
  public API is added. Tests execute only existing Accepted routes.
- Replay/determinism: the matrix records bounded structure evidence without
  defining an Effect Log, public Replay schema, checksum, privacy policy, or
  compatibility relation. Unicode remains 17.0.0 and original UTF-8 byte spans
  remain authoritative.

## Unresolved alternatives

- Canonical JSON, CBOR, protobuf, custom binary, or another encoding cannot be
  selected without field types, ordering, unknown-field, integrity, privacy,
  reader/writer, and migration authority.
- Treating private `TaskScheduleTrace::canonical_bytes()` as the first public
  Replay revision is rejected because it intentionally excludes public
  versioning, Actor/external-Effect scope, privacy, corruption, compatibility,
  and cross-process guarantees.
- Hashing the private trace to manufacture a checksum is rejected because
  algorithm, scope, keying, framing, authenticity, truncation, and migration
  remain unresolved.
- Public schema and event protocol work remains RFC-0010 and REP-2503 through
  REP-2506 scope.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
