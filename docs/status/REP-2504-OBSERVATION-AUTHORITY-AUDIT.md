# REP-2504-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2504-OBSERVATION` is authorized by Accepted `DEC-0107`.
It records only a test-local inventory of proposed replay-player boundaries.
Public `REP-2504` remains `BlockedSpec`: no checkpoint validation, event
application, divergence engine, or replay playback is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted replay and differential
  contracts before playback.
- `DEC-0012` covers Seed Semantic IDs/canonical bytes only.
- `DEC-0106` keeps effect-recording boundaries test-only.
- `DEC-0107` authorizes this child only; `GAP-DETERMINISTIC-REPLAY-001` remains
  open.

## Current implementation boundary

`replay_player_evidence.rs` defines eleven test-local boundaries, sorts them by
local rank, rejects duplicates, and compares forward/reverse insertion order.
The evidence tag is test-only and is not a checkpoint, event log, or player
protocol.

No replay player, preflight validator, checkpoint format, event-log reader,
divergence engine, CLI command, diagnostic, Semantic ID, public protocol, or
migration behavior was added. Stale `zero` command names remain absent from
implementation surfaces.

## Evidence and deferred work

Focused tests cover complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines checkpoint binding, event semantics,
Fault/cancellation, divergence, privacy, integrity, migration, and runtime
evidence.
