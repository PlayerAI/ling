# REP-2502-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2502-OBSERVATION` is authorized by Accepted `DEC-0105`.
It records only a test-local replay-field vocabulary and deterministic ordering
evidence. Public `REP-2502` remains `BlockedSpec`: no wire schema, payload,
checksum, privacy policy, decoder, or replay behavior is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Determinism Class, Effect Log,
  Replay version, and privacy boundaries before replay support.
- `DEC-0012` covers Seed Semantic IDs/canonical bytes only; it is not replay
  wire authority.
- `DEC-0104` keeps determinism-class labels test-only.
- `DEC-0105` authorizes this child only; `GAP-DETERMINISTIC-REPLAY-001` remains
  open.

## Current implementation boundary

`replay_schema_evidence.rs` defines thirteen test-local proposed fields,
sorts them by local rank, rejects duplicate labels, and compares forward and
reverse insertion order. The evidence tag is test-only and is not a wire
schema or protocol.

No replay schema, encoder/decoder, event ID assignment, checksum, redaction
policy, protocol inventory entry, diagnostic, Semantic ID, CLI/LSP command,
public protocol, or migration behavior was added. Stale `zero` command names
remain absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover complete field inventory, deterministic ordering,
duplicate rejection, and explicit non-protocol boundaries. The parent remains
blocked until accepted authority defines payloads, identities, ordering,
integrity, versions, migration, privacy, corruption, divergence, and runtime
replay evidence.
