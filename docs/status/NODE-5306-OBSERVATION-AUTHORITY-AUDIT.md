# NODE-5306-OBSERVATION Authority Audit — Node/Actor Boundary Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0190` permits only test-local Node/Actor bridge vocabulary. It
does not authorize a queue, envelope, bridge runtime, mailbox policy,
ownership model, replay schema, diagnostics, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:277-286` is a
  non-normative bridge proposal.
- `docs/status/NODE-5306-AUTHORITY-AUDIT.md` records the missing envelope,
  queue, sampling, delivery, ownership, turn, Fault, replay, and profile
  authority.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-ACTOR-AWAIT-REENTRY-001`,
  `GAP-STRUCTURED-TASK-001`, `GAP-DETERMINISTIC-REPLAY-001`, and
  `GAP-CRITICAL-PROFILE-001` remain open.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Node/Actor
identity, envelope, queue/delivery, ownership, turn/lifecycle,
replay/profile, diagnostic, and fixture boundaries. It sorts by explicit
local rank, rejects duplicates, compares canonical opaque bytes for
forward/reverse input order, and uses an observation-only tag. No bridge,
queue, envelope, scheduler, diagnostic, CLI/LSP action, dependency, or
support claim is introduced.

## Required authority and compatibility

Accepted authority must define bridge/envelope identity and serialization,
queue capacity/admission/backpressure/drop/expiry, sampling/commit and clock
conversion, deterministic delivery order, ownership/Move/Borrow/Managed,
Actor turn/await/reentry/cancellation/supervision/restart, Fault/fallback,
hard-real-time non-waiting, replay/privacy/migration, profiles/targets,
stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures.
Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

NODE-5306 implementation, Node/Actor queues/envelopes, bridge runtime,
backpressure/drop/expiry, sampling/commit, delivery/order,
ownership/serialization, supervision/Fault/restart/fallback, replay
integration, diagnostics, CLI/LSP/runtime protocols, and support claims
remain deferred until accepted authority and executable offline evidence
exist. No placeholder bridge API is created.
