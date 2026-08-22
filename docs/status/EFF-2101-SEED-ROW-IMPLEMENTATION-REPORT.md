# EFF-2101-SEED-ROW implementation report

## Outcome

**Status: Done (bounded Seed-only snapshot slice).**

`ling-effects` now exposes a deterministic in-process snapshot of the existing
v0.0.1 closed `EffectRow`. The parent `EFF-2101` task remains `BlockedSpec` for
the v0.2 Effect model because open rows, handlers, State masking, and related
concurrency authorities are not accepted.

Implementation commit: `fb949ce2b28fc73a1668806952c3f8e790cd6d7a`.

## Normative traceability

- Accepted DEC-0010 governs the Seed `Console.Write`, `State<T>`, capability,
  and canonical row boundary.
- Accepted DEC-0060 §§1–4 authorizes only the path-free canonical snapshot;
  it does not select any v0.2 row or handler semantics.
- `docs/SEMANTICS.md` continues to define Seed Effect rows as deduplicated
  labels and leaves handler/masking semantics for later authority.

## Implemented slice

- Added `EffectRow::seed_snapshot`.
- Added immutable `SeedEffectRowSnapshot` with canonical names and `is_pure`.
- Canonical names are deduplicated and sorted by canonical identity, not by
  display spelling or host representation.
- The snapshot carries no source path, host state, allocation identity, row
  variable, handler, capability, protocol, or schema field.

## Evidence

- `cargo fmt --all -- --check` passed.
- `cargo test -p ling-effects --all-features --locked --offline` passed with
  10 unit tests.
- `cargo clippy -p ling-effects --all-targets --all-features --locked --offline
  -- -D warnings` passed.
- Tests cover mixed rows, duplicate elimination, insertion-order independence,
  display-spelling independence, pure rows, and repeated snapshot equality.

## Compatibility and deferred work

No source syntax, checked semantics, diagnostics, Semantic IDs, Audit Source,
schemas, CLI, runtime, bytecode, VM, protocols, or Unicode 17.0.0 behavior
changed. Open/closed rows, Effect IDs, row-variable constraints, operation
signatures, handlers, resume rules, State masking, Task/Actor labels, and v0.2
diagnostics remain deferred to the blocked `EFF-2101` parent.
