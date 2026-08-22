# NODE-5304-OBSERVATION Authority Audit — Virtual-Time Runtime Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0188` permits only test-local virtual-time/runtime vocabulary.
It does not authorize a clock, Node runtime, input/output trace schema,
overrun/Fault simulation, replay adapter, diagnostics, CLI/LSP actions, or
support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:254-264` is a
  non-normative runtime proposal.
- `docs/SEMANTICS.md:1380-1425` and `:1914-1931` keep Node out of Seed;
  `docs/LANGUAGE.md:857-866` is a surface example.
- RFC-0019/0020 are limited to interpreter/VM differential and Experimental VM
  host cancellation/resource evidence.
- Future `PROTO-REPLAY` and `GAP-DETERMINISTIC-REPLAY-001` leave replay
  semantics unresolved.
- `docs/status/NODE-5304-AUTHORITY-AUDIT.md` records the missing runtime,
  replay, target, and transaction authority.

## Current implementation evidence

The observation adds one isolated test with sixty explicit virtual-time,
clock/tick/input/output/trace, overrun/replay/privacy/migration, diagnostic,
and fixture boundaries, deterministic local ordering, duplicate rejection, and
an opaque observation tag. No clock, runtime, trace, replay, diagnostic,
CLI/LSP action, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define the reference-runtime boundary, virtual-clock
epoch/units/advancement, input/output traces and state commits, overrun/Fault,
replay equivalence/order/privacy/corruption/migration, target distinction,
stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures.
Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

NODE-5304 implementation, virtual clock/reference runtime, injected input and
output traces, overrun/Fault, replay integration, diagnostics, CLI/LSP/evidence
protocols, and support claims remain deferred until RFC-K502 (or an Accepted
replacement), replay authority, dependent Node/runtime decisions, and
executable offline evidence exist. No placeholder clock or public API is
created.
