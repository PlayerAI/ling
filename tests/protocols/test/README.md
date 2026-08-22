# `ling.test/0.1` protocol evidence

This fixture documents the Preview `ling test` explicit file-runner boundary
defined by DEC-0039. It runs standalone `.ling` `Main` programs in deterministic
logical-path order and captures their Console output in the JSON report.

The command does not read `ling.toml` or `ling.lock`, does not discover a
workspace, and does not define source-level test declarations, assertions,
filters, snapshots, property tests, parallelism, or cancellation. Compile and
runtime diagnostics remain bilingual `ling.diagnostic/0.1` JSON on stderr.

Executable evidence is in `crates/ling-cli/tests/test.rs`, the unit tests in
`crates/ling-cli/src/test_runner.rs`, and the schema corpus in
`schemas/test/0.1/`.
