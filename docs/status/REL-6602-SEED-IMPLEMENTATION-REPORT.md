# REL-6602-SEED implementation report

## Result

`REL-6602-SEED` is complete as the bounded internal fault-matrix drift gate.
The parent `REL-6602` remains `BlockedSpec` for release-level fault injection
and future crash/network/device/Actor/replay/proof/LSP recovery behavior.

## Authority and boundary

- Accepted DEC-0042 covers only the existing Seed evidence inventory.
- The matrix records eleven scenarios: one `Covered`, two `Partial`, and eight
  `Deferred`; state labels are not implementation or support claims.
- The verifier checks documentation consistency only and does not inject
  faults, run crash simulators, or define recovery semantics.

## Implementation

- `tools/xtask/src/fault.rs` validates the exact scenario set and state values,
  rejects duplicates and drift, and requires the documented deterministic fault
  policy phrases.
- `cargo xtask fault verify` reports deterministic scenario counts and fails
  closed with internal `GOV-FAULT-*` messages.
- The command is included in the Seed reproducibility CI gate and documented
  beside the existing cache, lock, database, and VM evidence.

## Verification

Executed locally, offline:

- `cargo xtask fault verify`
- `cargo test -p xtask --all-features --locked --offline --quiet`
- `cargo xtask ci verify`
- `cargo test --workspace --all-features --locked --offline --quiet`

The gate reports eleven scenarios with the expected 1/2/8 state split.

## Compatibility and deferrals

No Ling syntax, Checked Core, runtime, bytecode, diagnostics, schemas,
Semantic IDs, public protocols, dependencies, or Unicode 17.0.0 behavior
changed. Portable fault seams, crash artifacts, cross-process recovery, and
future protocol fault models remain deferred.
