# REP-2503 Authority Audit: Effect Recorder

## Outcome

`REP-2503` is correctly recorded as `BlockedSpec`. The G2 plan proposes
recording Clock, Random, external input, network/file/device reads, and
user-selected scheduling nondeterminism at declared Effect boundaries while
omitting pure or reconstructible intermediate state. The Effect Row/Handler
contract, replay event schema, scheduling class, payload encoding, privacy,
and failure behavior are not accepted.

No Effect recorder, recording boundary, event sink, payload serializer,
redaction policy, scheduler hook, diagnostic, protocol, or placeholder G2 API
was added.

## Normative traceability

- The G2 execution package is non-normative. Its recorder list cannot authorize
  a new Effect, handler ABI, event sink, or data-retention policy.
- REP-2503 depends on RFC-C201 (Effect Row/Handler), RFC-C205 (Determinism/
  Replay), REP-2501, and REP-2502. No Accepted RFC-C201/C205 or replacement
  RFC-0006/RFC-0010 exists; EFF-2101 through EFF-2105 and REP-2501/2502 are
  `BlockedSpec`.
- `docs/SEMANTICS.md` sketches Effect Rows and future recordable effects, but
  v0.0.1 supports only the Seed closed rows and `Console.Write`; it does not
  define handler elimination, effect operation identity, recording order,
  payload schemas, scheduler inputs, or recorder failure semantics.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit Effects,
  deterministic boundaries, and privacy-aware replay, but do not define a
  stable recorder API or event protocol.
- Accepted DEC-0010/DEC-0013 cover Seed Capability/State and runtime Faults;
  DEC-0021 is compiler-query-only; RFC-0020 excludes Task/Actor scheduling and
  replay. None authorizes Effect recording.
- `GAP-EFFECT-HANDLER-001`, `GAP-EFFECT-STATE-MASKING-001`, and
  `GAP-DETERMINISTIC-REPLAY-001` remain Open and jointly leave the recording
  boundary, order, privacy, and migration unresolved.

## Current implementation evidence

- `ling-effects` computes only Seed closed effect rows and module Capability
  closure; it has no user Effect operations, Handler boundary, operation ID,
  recorder interface, or scheduling nondeterminism model.
- `ling-eval` and `ling-vm` execute Seed Core/bytecode directly with no Effect
  event sink, Clock/Random/input/network/device abstraction, or replay hook.
- The workspace has no replay schema/encoder, redaction layer, event checksum,
  scheduler recorder, or public protocol. Existing Semantic Graph and
  bytecode schemas are not Effect logs.
- No fixture covers recordable versus reconstructible Effects, handler
  nesting/masking, effect ordering, recorder failure, sensitive values,
  scheduler inputs, Unicode/CRLF/BOM spans, or interpreter/VM equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Effect Row/Handler operation identity, declaration/inference, elimination,
   masking, nesting, unhandled-effect diagnostics, and the exact recorder
   boundary in Typed Core and runtime;
2. which Clock/Random/input/network/file/device/scheduler operations are
   recordable, their canonical payload/result/Fault types, ordering, retries,
   duplicate behavior, logical time, and reconstruction rules;
3. recorder lifecycle, buffering/flush/backpressure, limits, crash/cancellation
   behavior, integrity/checksum, interaction with Task/Actor/supervision, and
   interpreter/VM/native equivalence;
4. replay schema/version/class metadata, Semantic ID and profile/target
   binding, privacy/redaction/retention/encryption, Capability boundaries,
   diagnostics, Audit Source/Semantic Graph projection, and migration; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures for each recordable Effect, nested handlers, masked and
   unhandled operations, recorder failure, sensitive payloads, scheduler
   nondeterminism, Unicode/CRLF/BOM spans, deterministic output, and
   interpreter/VM/runtime behavior without unchecked-AST execution.

Until these decisions are Accepted, adding a recorder would silently decide
which external inputs become replay authority and how sensitive data is
captured.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Effect recorder, replay schema, diagnostic, Semantic ID, source-span, runtime,
or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REP-2503` can begin only after Accepted RFC-C201/C205 (or replacement
RFC-0006/RFC-0010), EFF-2101 through EFF-2105, and REP-2501/2502 resolve
Effect/Handler boundaries, recordable operations, event schema, privacy,
corruption, and migration. The future recorder must consume accepted checked
Core effect operations only, preserve deterministic ordering and explicit
failure semantics, and publish cross-process and interpreter/VM evidence
before exposing replay recording.
