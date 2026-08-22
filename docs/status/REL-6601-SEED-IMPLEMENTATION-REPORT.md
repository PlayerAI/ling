# REL-6601-SEED implementation report

## Result

`REL-6601-SEED` is complete as the bounded internal Seed fuzz-inventory and
corpus-drift gate. The parent `REL-6601` remains `BlockedSpec` for G6 release
coverage across future replay/evidence, archive, FFI, device, LSP/DAP, and
editor surfaces, and for the prerequisite G1--G5 exits.

## Authority and boundary

- Accepted DEC-0041 records the exact eight existing Seed harnesses and their
  eighteen corpus seed files.
- RFC-0020 remains the authority for the Experimental VM cancellation,
  bytecode fuzz, and resource evidence boundary.
- The gate validates declarations and inventory metadata only; it does not run
  libFuzzer, provide sanitizer results, or create a fuzz-result protocol.

## Implementation

- `tools/xtask/src/fuzz.rs` validates `fuzz/Cargo.toml`, target paths and
  `test/doc/bench` flags, `fuzz_target!` entry points, regular corpus files and
  expected counts, and inventory names.
- `fuzz/Cargo.lock` now records the already-declared direct `ling-source`
  dependency so the excluded fuzz workspace passes the locked offline check.
- `cargo xtask fuzz verify` reports deterministic target and corpus totals and
  fails closed on target-set, path, entry-point, nested-directory, file-count,
  or inventory drift.
- The command is included in the Seed reproducibility CI gate and documented in
  `fuzz/README.md`; future unimplemented input families remain explicit gaps.

## Verification

Executed locally, offline:

- `cargo xtask fuzz verify`
- `cargo test -p xtask --all-features --locked --offline --quiet`
- `cargo xtask ci verify`
- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline`

The gate reports eight targets and eighteen corpus files. Full workspace and
governance gates are run at the milestone integration boundary.

## Compatibility and deferrals

No Ling syntax, Checked Core, runtime, bytecode, diagnostic, schema, Semantic
ID, public protocol, dependency, or Unicode 17.0.0 behavior changed. LibFuzzer
execution, sanitizer availability, crash retention/triage, and future protocol
harnesses remain deferred to accepted G6 authority.
