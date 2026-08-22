# REP-2503-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2503-OBSERVATION` is authorized by Accepted `DEC-0106`.
It records only a test-local inventory of proposed recordable effect
boundaries. Public `REP-2503` remains `BlockedSpec`: no effect observation,
recorder hook, operation identity, payload, or replay event is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Effect Row/Handler and Replay
  contracts before recording can be exposed.
- Seed closed effect rows and `DEC-0021` compiler-query scheduling are not
  runtime recorder authority.
- `DEC-0105` keeps replay fields test-only.
- `DEC-0106` authorizes this child only; the effect and replay gaps remain
  open.

## Current implementation boundary

`effect_recorder_evidence.rs` defines six test-local proposed boundaries,
sorts them by local rank, rejects duplicates, and compares forward/reverse
insertion order. The evidence tag is test-only and is not an event log or
recorder protocol.

No Effect recorder, operation ID, event sink, payload serializer, redaction
policy, scheduler hook, diagnostic, Semantic ID, CLI/LSP command, public
protocol, or migration behavior was added. Stale `zero` command names remain
absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines Effect operation semantics,
recordability/reconstruction, lifecycle/failure, privacy, payloads, replay,
and runtime differential evidence.
