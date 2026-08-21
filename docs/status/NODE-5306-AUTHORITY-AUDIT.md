# NODE-5306 Authority Audit — Node/Actor Boundary

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5306 proposes an explicit bridge from an Actor through a sampled input
queue into a Node and from a Node through a bounded output event back to an
Actor. Hard-real-time Nodes must not wait for Actors, networks, or dynamic
services; buffering, drop/expiry, and fallback are left to the Profile.

No Accepted RFC defines the bridge's queue capacity, sampling/commit clock,
backpressure, delivery/order, drop/expiry, ownership, serialization,
Actor-turn, restart, Fault, or fallback semantics. Node and Actor are both
reserved beyond the Seed subset. Implementing this boundary now would invent
observable loss, timing, liveness, and resource behavior.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:277-286` is a
  non-normative plan fragment. It names the two directions of the bridge and
  the hard-real-time non-waiting rule but defines no envelope, queue, clock,
  ownership, capacity, backpressure, drop/expiry, ordering, restart, or
  fallback contract.
- `docs/SEMANTICS.md:1283-1376` sketches Actor identity, state isolation,
  mailbox, turn, supervision, and delivery concepts, while
  `:1380-1425` sketches Node timing. These are not an Accepted bridge RFC;
  `:1914-1931` explicitly reserves Actor and Node for future functionality.
- `docs/LANGUAGE.md:827-866` gives surface examples for `actor` and `node`,
  not an accepted message/port/queue schema or runtime boundary. Its v0.0.1
  exclusions also list Task/Actor/Node as unimplemented.
- `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md:12-19` correctly states
  that mailboxes must be bounded and effects explicit, but it is a plan and
  leaves Actor turn/reentry, mailbox policy, and Replay under RFC-C202–C205.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` leaves mailbox capacity/result,
  backpressure, ordering, termination, restart budgets, escalation, and
  supervisor Faults Open. `GAP-ACTOR-AWAIT-REENTRY-001` leaves turn suspension
  and state invariants Open; `GAP-STRUCTURED-TASK-001` leaves cancellation,
  cleanup, and deterministic scheduling Open.
- `GAP-DETERMINISTIC-REPLAY-001` leaves event order, effect logs, privacy,
  corruption, divergence, and migration Open. `PROTO-REPLAY` is Future with no
  schema. `GAP-CRITICAL-PROFILE-001` leaves Node timing/Fault, boundedness, and
  Critical evidence Open; ownership and Native/Device gaps affect the bridge.
- No RFC-0008, RFC-0009, or RFC-K502 is Accepted for this boundary. Existing
  VM host-control and compiler-query scheduling decisions do not define Actor
  messages or a Node/Actor runtime.

## Current implementation evidence

- The compiler, evaluator, and VM have no Actor runtime, Node runtime,
  sampled-input queue, bounded-output event, message envelope, bridge
  scheduler, or Node/Actor fixtures.
- There is no rule for queue capacity/admission, backpressure (wait/reject/
  drop), stale/expired input, output loss, delivery order, sampling phase,
  state ownership, serialization, or Actor turn/reentry across the bridge.
- Existing VM limits and host cancellation are per-execution safety controls;
  they do not provide mailbox behavior, Node non-waiting guarantees, restart/
  supervision, or replayable bridge events.
- No target/profile matrix fixes which bridge policies are permitted in hard
  real-time versus non-real-time Nodes, or how unsupported network/service
  access fails and falls back.
- No stable bilingual diagnostic or schema fixes full/expired mailbox,
  dropped output, stale input, ordering, backpressure, ownership/serialization,
  deadline, Actor Fault, restart, fallback, or profile mismatch.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Versioned input/output envelope and bridge identity, including port/message
   schema IDs, source spans, Semantic IDs, serialization, ownership/Move/
   Borrow/Managed rules, and privacy/redaction.
2. Queue capacity, admission, backpressure, drop/expiry/stale-input policy,
   output delivery, ordering, sampling/commit phase, clock conversion,
   bounded memory, and deterministic tie-breaks for simultaneous events.
3. Actor turn, await/reentry, cancellation, supervision/restart, shutdown,
   Fault propagation, resource cleanup, and the hard-real-time rule that Node
   execution cannot wait on Actor/network/service operations.
4. Node schedule/deadline/overrun and Profile policy for bridge buffers,
   fallback, safe mode, unsupported capabilities, Native/Device/FFI, and
   target/evidence claims.
5. Replay/determinism boundary for bridge inputs, outputs, drops, retries,
   Faults, and supervision, including log versioning, corruption, divergence,
   migration, and cross-process equivalence.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for queue/backpressure/drop/expiry, stale input, ordering, ownership,
   serialization, deadline/overrun, Actor/Node Fault, restart/fallback, and
   unsupported profile/target behavior.
7. Offline executable positive/negative, capacity/backpressure, stale/drop,
   ordering, multi-rate/sampling, ownership/serialization, await/reentry,
   cancellation/restart, Fault/fallback, replay, Unicode/CRLF/BOM,
   migration, determinism, and interpreter/VM/Native differential fixtures.

## Evidence and compatibility impact

The eventual implementation must consume checked Node and Actor Core only after
both authorities are accepted, and must fail closed on unknown bridge or
delivery semantics. Queue/trace facts must be deterministic and target/profile
bound while excluding host paths, addresses, thread scheduling, allocator
details, timing noise, and debug text from Ling identity. Diagnostics and
envelopes must preserve original UTF-8 spans and Semantic IDs.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, scheduler, Task, Actor, Node runtime, Native backend,
memory or ownership behavior, replay protocol, diagnostics, schemas,
Semantic IDs, source spans, CLI, LSP, dependency lock, target/toolchain
support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

NODE-5306 implementation, Node/Actor bridge queues/envelopes, backpressure and
drop/expiry, sampling/commit, delivery/order, ownership/serialization,
supervision/Fault/fallback, replay integration, diagnostics, CLI/LSP/evidence
protocols, and support claims remain deferred until RFC-K502, RFC-0008,
RFC-0009, and RFC-0010 (or Accepted replacements), the Critical/ownership/
Native/Device authorities, and NODE-5301 through NODE-5305 are resolved with
independent offline fixtures. No placeholder bridge, queue, envelope, or
public API is created.
