# REP-2501-OBSERVATION Authority Audit

## Outcome

The bounded child `REP-2501-OBSERVATION` is authorized by Accepted `DEC-0104`.
It keeps a test-local vocabulary for four proposed determinism classes and
checks existing effect canonicalization. Public `REP-2501` remains
`BlockedSpec`: no class inference, runtime equivalence, replay metadata, or
cross-process contract is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Determinism Class, Effect Log,
  Replay version, and privacy boundaries before replay support.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` leave runtime replay outside Seed.
- `DEC-0021` is compiler-query determinism only, not runtime replay authority.
- `DEC-0104` authorizes this child only; `GAP-DETERMINISTIC-REPLAY-001` remains
  open.

## Current implementation boundary

`determinism_evidence.rs` defines a test-local enum for Strict, Seeded,
RecordedEffects, and BestEffort. It combines each label with existing checked
`EffectRowModel` canonical bytes and verifies equivalent effect-label order.
The labels are planning vocabulary only and do not classify programs or
produce metadata.

No production determinism enum, class inference, build-metadata field,
Semantic Graph field, replay header, scheduler contract, diagnostic, Semantic
ID, CLI/LSP command, public protocol, or migration behavior was added. Stale
`zero` command names remain absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover the four provisional labels, deterministic test-local
ordering, and checked effect projection independence from input order. The
parent remains blocked until accepted authority defines class semantics,
effect/runtime boundaries, metadata, replay, privacy, corruption, divergence,
and cross-process evidence.
