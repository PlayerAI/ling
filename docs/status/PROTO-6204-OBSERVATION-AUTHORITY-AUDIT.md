# PROTO-6204-OBSERVATION Authority Audit

- Task: `PROTO-6204-OBSERVATION` — Internal CLI and Exit-Code Freeze boundary evidence
- Parent: `PROTO-6204` — CLI and Exit-Code Freeze
- Decision: Accepted `DEC-0222`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0013`, `DEC-0036`, `DEC-0037`, and `DEC-0040` authorize exact
internal evidence for the commands and exit values already implemented.
Accepted scoped decisions additionally account for formatter, init,
standalone test, LSP, and project-check entries. Accepted `DEC-0222` permits
the exact inventory to be regression tested without upgrading its lifecycle.

No Accepted authority defines the plan's complete Stable 1.0 matrix, shared
flags and defaults, color/path policy, all-command offline behavior, or
plan-only build/query/patch/replay/explain/evidence commands. Those surfaces
remain outside this child.

## Authorized implementation

1. Assert the exact ordered internal catalog, parseable roots, hierarchical
   project-check spelling, and rejection of plan-only roots.
2. Assert the exact assigned exit list and the absence of an assigned value
   `3`.
3. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes outside public CLI protocols.
4. Correct the parent authority audit's stale project/formatter gap and
   implemented-command facts; register decision, lifecycle, report, and task
   traceability.

## Explicit exclusions

This slice adds no command, alias, option, default, output contract, exit
meaning, completion artifact, color/path/offline policy, schema, diagnostic,
stability promotion, public Rust API, or stale `zero` spelling. Parent
`PROTO-6204` remains `BlockedSpec`.
