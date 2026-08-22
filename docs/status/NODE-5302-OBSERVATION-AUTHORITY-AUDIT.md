# NODE-5302-OBSERVATION Authority Audit — Node Checked Core Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0186` permits only test-local Node Checked Core vocabulary. It
does not authorize a Core schema, AST/HIR/Typed-Core variant, graph or cycle
solver, fixed-point proof, diagnostics, CLI/LSP actions, or support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:226-240` is a
  non-normative Core proposal.
- `docs/SEMANTICS.md:372-404`, `:1380-1425`, and `:1914-1931` keep NodeStep
  out of the Seed implementation and reserve Node for a future version.
- `docs/LANGUAGE.md:857-866` is a design example, not accepted Core or
  lowering authority.
- `docs/status/NODE-5302-AUTHORITY-AUDIT.md` records missing RFC-K502 and
  dependent graph, timing, ownership, resource, target, and transaction
  authority.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Node Core,
port/state/tick/clock/graph/cycle/fixed-point, Fault/Contract, target,
diagnostic, and fixture boundaries, deterministic local ordering, duplicate
rejection, and an opaque observation tag. No Core variant, checker, solver,
diagnostic, CLI/LSP action, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define the versioned Core schema, port/state/tick/clock
semantics, graph identity/order, feedback/cycle/fixed-point soundness,
ownership and sampling/commit rules, Fault/Contract/resource/target
relations, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode
17.0.0 remain unchanged.

## Deferred work

NODE-5302 implementation, Node Checked Core schema/lowering, graph and
fixed-point analysis, diagnostics, CLI/LSP/evidence protocols, and support
claims remain deferred until RFC-K502 (or an Accepted replacement), the
dependent Node/Critical/runtime authorities, and executable offline evidence
exist. No placeholder Node Core variant or public API is created.
