# NODE-5303-OBSERVATION Authority Audit — Static Scheduling Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0187` permits only test-local scheduling vocabulary. It does not
authorize a Node scheduler, graph/rate analysis, multi-rate bridge,
schedulability/WCET proof, manifest, diagnostics, runtime behavior, or support
claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:242-252` is a
  non-normative scheduling proposal.
- `docs/SEMANTICS.md:1380-1425` and `:1914-1931` keep Node out of Seed.
- `docs/decisions/0019-incremental-query-boundary.md:39-49` specifies only
  internal compiler-query scheduling.
- `docs/status/NODE-5303-AUTHORITY-AUDIT.md` records missing RFC-K502,
  schedulability, target, concurrency, replay, and manifest authority.

## Current implementation evidence

The observation adds one isolated test with sixty explicit graph/order,
rate/clock/bridge, release/deadline/priority, overrun/replay, target/manifest,
diagnostic, and fixture boundaries, deterministic local ordering, duplicate
rejection, and an opaque observation tag. No scheduler, bridge, manifest,
diagnostic, CLI/LSP action, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define graph/schedule identity and ordering,
clock/rate/bridge semantics, priority/release/deadline/admission/WCET,
overrun/Fault/replay behavior, target/compiler evidence, manifest migration,
stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures.
Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

NODE-5303 implementation, graph/schedule analysis, bridges,
schedulability/WCET, scheduler manifest, diagnostics, CLI/LSP/runtime
protocols, and support claims remain deferred until RFC-K502 (or an Accepted
replacement), the dependent concurrency/replay/Node authorities, and
executable offline evidence exist. No placeholder scheduler or public API is
created.
