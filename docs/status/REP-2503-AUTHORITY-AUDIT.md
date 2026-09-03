# REP-2503 Authority Audit: Effect Recorder

## Outcome

`REP-2503` is complete only for the private Experimental baseline authorized by
Accepted DEC-0281 and implemented in
`8a0f05b9a38f922c73339d4111e6927ac9cb693b`. The evidence observes the current
checked `Console.Write.write` host boundary after lexical Handler dispatch; it
does not implement the public Effect Recorder described by the lower-authority
execution plan.

The remaining public blocker is explicit: no Accepted RFC-C205,
RFC-0010, or replacement defines public recordability, Clock/Random/external
input producers, scheduler capture, recorder lifecycle, payload encoding,
privacy, integrity, migration, or Replay behavior. Accepted DEC-0281 therefore
authorizes only a private five-case checked interpreter evidence slice at the
existing injected Console host boundary.

No production Effect recorder, recording hook, event sink, payload serializer,
redaction policy, scheduler hook, diagnostic, protocol, or placeholder G2 API
has been added.

## Normative traceability

- The G2 execution package is non-normative. Its Clock, Random, external input,
  network/file/device, and scheduler list cannot authorize public recordability
  or data retention.
- Accepted RFC-0006 defines canonical Effect/operation identity and checked
  Handler contracts. Accepted DEC-0260 fixes the bounded operation registry,
  and DEC-0261 fixes deep lexical runtime dispatch for the only current source
  producer, `Console.Write.write(Text) -> Unit :: Once`.
- DEC-0261 explicitly leaves Clock/Random producers and Replay separate. The
  repository has no Accepted ExternalInput, NetworkReceive, or FileDeviceRead
  operation/runtime contract.
- Accepted DEC-0010/DEC-0013 retain Capability and structured Runtime Fault
  boundaries. Accepted DEC-0280 completes only private non-serialized Task
  trace structure evidence; it is not an Effect Log or recorder contract.
- Accepted DEC-0106 is vocabulary-only evidence for six proposed boundaries.
  It observes no execution and cannot be upgraded into recordability by
  analogy.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open. The Effect/Handler gaps are
  resolved for their accepted Experimental scope, but that does not resolve
  privacy, recorder lifecycle, schema, or Replay compatibility.

## Current implementation evidence

- `ling-effects` publishes canonical checked operation and Handler Core
  identity. `ling-eval` and bytecode/VM execute checked Console handlers with
  nearest/deep interception, clause-outside-handler behavior, structured host
  failures, committed effects, original spans, and bounded continuations.
- The injected `ling_eval::Console` adapter receives canonical-LF host text
  only after an operation escapes every applicable lexical handler. A test
  adapter can therefore observe this accepted boundary without changing the
  interpreter or exposing a production hook.
- `Clock.now` and `Random.next` are checked registry entries without accepted
  source/Core producers or runtime host adapters. External input, network
  receive, and file/device read remain planning vocabulary. Task/Actor schedule
  traces are separate private runtime evidence, not checked Effect operations.
- The workspace still has no public recorder interface, Effect Log,
  encoder/decoder, replay payload schema, privacy/redaction layer, event
  checksum, cross-process reader, recorder diagnostic, or implemented
  `PROTO-REPLAY` version.

## Implemented bounded evidence

Accepted DEC-0281 authorizes, and the implementation commit above provides,
only this internal Experimental baseline:

1. one crate-private `cfg(test)` matrix executes exactly five case families
   through the real checked interpreter and injected Console adapter;
2. observations occur after accepted Handler dispatch and contain only a
   test-local ordinal, constant Console operation identity, bounded repository
   literal host text, and structured success/failure outcome;
3. escaped operations retain strict source order, handled operations remain
   absent, clause operations are observed only after escaping outer handlers,
   and one host failure prevents later invocation;
4. Unicode/BOM/CRLF/logical-source reconstruction compares only bounded
   logical observations while Runtime Fault spans remain original-byte
   sidecars; and
5. all six DEC-0106 provisional boundaries retain explicit deferred
   dispositions, and negative evidence proves no production/public recorder,
   Effect Log, schema, privacy, diagnostic, CLI, or protocol surface exists.

This Accepted decision deliberately uses Console only as the current executable
host-boundary probe. It does not decide that Console output is a future Replay
input or alias Console to any DEC-0106 provisional name.

## Required authority for a public implementation

An Accepted RFC or replacement decision must still define, at minimum:

1. the public recordable-versus-reconstructible operation set, including
   Clock/Random/external input/network/file/device producers and Task/Actor
   scheduler or message boundaries;
2. event identity/order/logical time, input/result/Fault payload types and
   encoding, retries/duplicates, cancellation, supervision, and divergence;
3. recorder lifecycle, buffering, flush, backpressure, resource limits,
   failure/crash recovery, checkpoint interaction, and interpreter/VM/native
   equivalence;
4. privacy classification, redaction, retention, encryption/integrity,
   Capability boundaries, diagnostics, Audit/Semantic Graph projection,
   reader/writer compatibility, and migration; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures without unchecked-AST execution.

Until that authority is Accepted, a production recorder would silently decide
which external inputs become Replay authority and which sensitive values are
retained.

## Evidence and compatibility

This refreshed audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, RFC-0006, RFC-0020, DEC-0010,
DEC-0013, DEC-0106, DEC-0260, DEC-0261, DEC-0279, DEC-0280,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`, the governance registries,
the EFF-2105 and REP-2502 implementation reports, and the current effects,
evaluator, Task/Actor, bytecode, VM, diagnostic, and schema code.

The executable evidence and repository verification are bound to implementation
commit `8a0f05b9a38f922c73339d4111e6927ac9cb693b`; see
`docs/status/REP-2503-IMPLEMENTATION-REPORT.md` for the exact case matrix and
commands.

No source language, compiler, interpreter, VM, bytecode, scheduler, mailbox,
Actor protocol, recorder, Replay schema, diagnostic, Semantic ID, source-span,
runtime, dependency, or Unicode 17.0.0 behavior changed by this audit or the
Accepted private evidence work.

## Intentionally deferred

REP-2503 is Done only for the private test baseline. Public recording remains
blocked and requires Accepted RFC-0010 or replacement authority. REP-2504 and
later work must retain checked-Core-only execution, deterministic ordering,
explicit failure/privacy rules, and cross-process plus interpreter/VM evidence
before exposure.
