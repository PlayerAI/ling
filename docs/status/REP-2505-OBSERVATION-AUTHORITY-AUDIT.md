# REP-2505-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2505-OBSERVATION` is authorized by Accepted
`DEC-0108`. It records only a test-local inventory of proposed replay privacy,
trimming, and corruption boundaries. Public `REP-2505` remains `BlockedSpec`:
no data classification, redaction, retention, trimming, checksum, corruption,
or offline replay behavior is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted replay, privacy, and
  differential contracts before replay data tooling.
- `DEC-0012` covers Seed Semantic IDs/canonical bytes only.
- `DEC-0107` keeps replay-player boundaries test-only.
- `DEC-0108` authorizes this child only; `GAP-DETERMINISTIC-REPLAY-001`
  remains open.

## Current implementation boundary

`replay_privacy_evidence.rs` defines sixteen test-local boundaries, sorts them
by local rank, rejects duplicates, and compares forward/reverse insertion
order. The evidence tag is test-only and is not a privacy policy, replay
chunk, checksum, diagnostic, or offline protocol.

No sensitivity classifier, redactor, trimmer, retention store, key manager,
chunk decoder, checksum implementation, corruption diagnostic, offline command,
Semantic ID, or public protocol was added. Stale `zero` command names remain
absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines sensitivity and redaction policy,
dependency-preserving trim closure, chunk/checksum bytes, corruption handling,
offline guarantees, migration, and runtime evidence.
