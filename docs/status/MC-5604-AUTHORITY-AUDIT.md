# MC-5604 Authority Audit

- Task: `MC-5604` — Replay Counterexample
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:471-473`
- Release: G5
- Status: `BlockedSpec`

## Decision

MC-5604 is `BlockedSpec`. The execution-plan sentence is a non-normative
proposal: it asks for a model-checking counterexample to become a deterministic
scheduler/replay fixture, to run in a reference runtime, and to retain source
linkage. No accepted specification defines that boundary or the data needed to
make such a fixture reproducible.

The repository has no accepted RFC-K506 authority, replay schema, deterministic
scheduler trace contract, counterexample converter, replay reader/writer,
reference-runtime replay route, or replay fixtures. Implementing any of these
would invent public semantics and would also require unresolved Node, Task,
Actor, boundedness, Contract/Proof, Critical Profile, and evidence decisions.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:471-473` is the only direct plan statement. It does
  not define an event vocabulary, scheduler policy, logical clock, input/effect
  boundary, state identity, failure behavior, or a file format.
- `GAP-DETERMINISTIC-REPLAY-001` is still Open. It explicitly leaves
  determinism classes, recorded effects, event order, log versioning,
  privacy/redaction, corruption handling, and replay divergence unaccepted;
  its candidate RFC is RFC-0010, not an accepted authority for MC-5604.
- `PROTO-REPLAY` is catalogued as Planned public/Future with no version, public
  schema, canonical encoding, reader policy, writer policy, unknown-field
  policy, migration tool, or fixtures. Its authorities are the roadmap and gap
  register, so it cannot authorize an implementation.
- MC-5601, MC-5602, and MC-5603 remain blocked. There is therefore no accepted
  projection, exploration trace, or model-check result/counterexample schema to
  convert.
- Accepted RFC-0019 defines a bounded interpreter–VM differential harness. It
  compares logical events, Unit results, stable Fault projections, original
  source spans, and checked snapshot identity; it does not define a scheduler
  replay log or a counterexample format.
- Accepted RFC-0020 defines experimental host cancellation for the VM and
  explicitly excludes Ling Task/Actor cancellation, a common logical heap,
  scheduler semantics, and a replay protocol. DEC-0019's deterministic
  single-threaded scheduler is an internal incremental-query implementation
  boundary and likewise defers persistence/corruption and parallel-scheduler
  protocols.
- Open Node/Task/Actor and Critical Profile gaps leave mailbox ordering,
  backpressure, reentry, ownership, clocks, Fault/restart behavior, and
  bounded execution unresolved. Those choices directly affect whether a
  counterexample can be replayed and what a source link means.

## Evidence in this repository

There is no replay or counterexample implementation under `crates/` or
`tests/`, and no versioned replay schema or fixture corpus. Existing VM
differential tests and host-cancellation controls are scoped to their accepted
RFCs and are not a replay consumer. No `ling` CLI, LSP request, diagnostic, or
public protocol currently claims replay support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. The conversion boundary between a checked model, exploration result,
   counterexample, and replay fixture, including the non-proof status of a
   bounded counterexample.
2. A versioned canonical replay schema with model/runtime/scheduler identity,
   logical time, inputs, host effects, state snapshots or checksums, Fault and
   restart events, and deterministic event ordering.
3. Node/Task/Actor semantics for ownership, mailbox order, backpressure,
   reentry, drops/expiry, cancellation, and restart, plus the capability and
   resource boundaries that may be recorded.
4. Stable source linkage using Semantic IDs and original UTF-8 byte spans,
   checked snapshot identity, and explicit privacy/redaction rules. Host paths,
   addresses, wall-clock timing, allocator layout, and debug output must not
   become Ling identity.
5. Reference-runtime replay behavior and fail-closed handling for divergence,
   malformed/corrupt data, unknown fields or versions, migration, unavailable
   inputs, and unsupported Faults, with registered bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics and documented process/fixture outcomes.
6. Offline positive, negative, corruption, migration, divergence, source-link,
   Unicode 17.0.0, BOM/CRLF, and repeated-run determinism fixtures. The
   fixtures must identify the accepted model/runtime versions and must never
   present replay as proof of an unbounded property.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately does not add a replay
reader/writer, scheduler trace, model-check converter, reference-runtime hook,
or placeholder API, and it does not introduce stale `zero` names.

MC-5604 remains deferred until the replay, model-checking, runtime, scheduling,
source-link, evidence, and governance authorities above are Accepted and their
executable fixtures are present.
