# MEM-3101-SEED-VALUE implementation report

## Outcome

**Status: Done (bounded Seed-only classification slice).**

`ling-types` now exposes the existing v0.0.1 value boundary as an explicit
`SeedTypeClass::Value` observation. The parent `MEM-3101` task remains
`BlockedSpec` because the Value/Managed/Resource model, ownership, and Native
authorities are not accepted.

Implementation commit: `c64ceb9101190d630125d3a7b6e1ede150c01488`.

## Normative traceability

- Accepted DEC-0008 and DEC-0009 govern Seed value restriction, copy semantics,
  and the absence of Borrow/Resource behavior.
- Accepted DEC-0061 §§1–4 authorizes only the completed-type classification;
  it deliberately excludes future memory kinds and ownership rules.

## Implemented slice

- Added `SeedTypeClass::Value`.
- Added `Type::seed_type_class`, returning `Some(Value)` for completed Seed
  primitive, aggregate, nominal, function, and collection forms.
- Unresolved type variables and the internal error sentinel return `None`.
- No memory layout, pointer identity, allocation, Drop, Managed, Resource,
  Borrow, lifetime, profile, ABI, FFI, or serialization data is exposed.

## Evidence

- `cargo fmt --all -- --check` passed.
- `cargo test -p ling-types --all-features --locked --offline` passed with
  39 unit tests.
- `cargo clippy -p ling-types --all-targets --all-features --locked --offline
  -- -D warnings` passed.
- Tests cover completed primitive, tuple, and function forms plus unresolved
  and error sentinels.

## Compatibility and deferred work

No source syntax, Checked Core field, diagnostics, schemas, Semantic IDs,
source spans, CLI, runtime, bytecode, VM, protocol, or Unicode 17.0.0 behavior
changed. Managed/Resource classification, kind constraints, Copy/Move,
ownership, borrowing, regions, Drop, profiles, Native, FFI, and memory
diagnostics remain deferred to the blocked `MEM-3101` parent.
