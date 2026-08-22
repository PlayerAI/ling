# REP-2506-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2506-OBSERVATION` is authorized by Accepted
`DEC-0109`. It records only a test-local inventory of proposed cross-process
replay acceptance boundaries. Public `REP-2506` remains `BlockedSpec`: no
process harness, replay acceptance, reproducibility, or observable-equivalence
contract is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted replay, privacy, and
  differential contracts before cross-process replay acceptance.
- `DEC-0012` covers Seed Semantic IDs/canonical bytes only.
- `DEC-0108` keeps replay privacy and integrity boundaries test-only.
- `DEC-0109` authorizes this child only; `GAP-DETERMINISTIC-REPLAY-001`
  remains open.

## Current implementation boundary

`replay_cross_process_evidence.rs` defines eighteen test-local boundaries,
sorts them by local rank, rejects duplicates, and compares forward/reverse
insertion order. The evidence tag is test-only and is not a process harness,
replay result, acceptance artifact, or reproducibility claim.

No process runner, clean-cache isolation, toolchain lock, generator/player,
observable comparator, mutation validator, diagnostic, Semantic ID, public
protocol, or CI acceptance rule was added. Stale `zero` command names remain
absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover complete boundary inventory, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines process/toolchain identity,
generator/player binding, observable equivalence, repeatability, divergence,
provenance, platform limits, privacy/corruption, offline guarantees, and
runtime evidence.
