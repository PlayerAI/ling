# NODE-5301-OBSERVATION Authority Audit — Node Syntax/Semantics Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0185` permits only test-local Node vocabulary. It does not
authorize `node` syntax, a `NodeStep` Checked Core form, clocks, schedulers,
deadline/overrun/Fault semantics, diagnostics, CLI/LSP actions, or support
claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:199-224` is a
  non-normative Node proposal dependent on absent RFC-K502.
- `docs/SEMANTICS.md:372-404`, `:1380-1425`, and `:1914-1931` keep Node out
  of the v0.0.1 Seed implementation and reserve it for a future version.
- `docs/LANGUAGE.md:857-866` is a design example, not accepted grammar or
  timing authority.
- `docs/status/NODE-5301-AUTHORITY-AUDIT.md` records missing Node, timing,
  target, resource, and transaction authority.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Node syntax,
ports/state/timing/clock/composition, target evidence, diagnostic, and fixture
boundaries, deterministic local ordering, duplicate rejection, and an opaque
observation tag. No parser, Core variant, scheduler, runtime, diagnostic,
CLI/LSP action, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define Node grammar and checked representation,
logical-tick/sampling/commit/state semantics, clock and target/WCET evidence,
deadline/overrun/Fault/recovery behavior, resource/ownership/concurrency
relations, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode
17.0.0 remain unchanged.

## Deferred work

NODE-5301 implementation, Node syntax/Core/runtime, clock/scheduler/timing
analysis, diagnostics, CLI/LSP/evidence protocols, and support claims remain
deferred until RFC-K502 (or an Accepted replacement), the dependent Critical
and runtime authorities, and executable offline evidence exist. No placeholder
`node` parser or public API is created.
