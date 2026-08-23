# STD-6302-OBSERVATION Authority Audit

- Task: `STD-6302-OBSERVATION` — Internal Convenience API Removal Audit boundary evidence
- Parent: `STD-6302` — Remove Convenience APIs
- Decision: Accepted `DEC-0224`
- Release: G6
- Status: authorized bounded evidence

## Authority conclusion

Accepted `DEC-0011` and `DEC-0014` protect the exact current Seed symbols;
Accepted `DEC-0223` freezes their inventory and truthful support state.
Accepted `DEC-0224` therefore authorizes an empty removal-set audit and a
negative gate for plan-only convenience names, not a source compatibility
change.

No Accepted authority names a removable symbol, replacement, compatibility
window, diagnostic, package transition, or migration. The parent remains
blocked until such authority exists.

## Authorized implementation

1. Assert that the compiler-injected surface contains only the twelve Accepted
   Seed symbols and excludes representative plan-only convenience surfaces.
2. Add a sixty-category test-local inventory with deterministic ordering,
   duplicate rejection, and opaque bytes outside public semantics.
3. Register decision, lifecycle, implementation report, backlog, and task
   traceability.

## Explicit exclusions

No source API, built-in, Prelude definition, package, profile, diagnostic,
Semantic ID, or runtime behavior is deleted, hidden, rejected, deprecated,
migrated, replaced, or added. Parent `STD-6302` remains `BlockedSpec`.
