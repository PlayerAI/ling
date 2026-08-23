# DOC-6702-EXECUTION-MANIFEST Implementation Report

## Result

The Seed example inventory and CLI process test now consume one strict six-case
manifest. All six checked-in examples are validated as existing sources, and
the process test checks, runs, and emits Semantic output for every case.

The parent `DOC-6702` remains `BlockedSpec`. The manifest and successful Seed
execution do not establish Stable 1.0 coverage.

## Implementation

- `tests/examples/seed-cases.toml` records one minimal, three realistic, and two
  tutorial examples with exact output and Semantic witnesses.
- `tools/xtask/src/examples_matrix.rs` strictly parses and validates the
  manifest alongside the existing documentation matrix.
- A focused negative test rejects classification drift, traversal/missing
  source paths, and non-LF output.
- `crates/ling-cli/tests/conformance.rs` replaces its hard-coded example list
  with the shared manifest and adds `examples/hello.ling` to the same real
  process loop.
- `tools/xtask/src/main.rs` reports the executable-case count.

## Acceptance evidence

- The exact manifest has six unique IDs and six exact existing `.ling` paths.
- The role distribution is one core-minimal, three core-realistic, and two
  tutorial cases; four cases record Chinese identifier use.
- Every case passes `ling check`, produces its exact UTF-8 stdout through
  `ling run`, and emits `ling.semantic/0.1` with its named definition witness.
- The existing negative conformance and deterministic Audit tests remain in
  the workspace gate.
- Focused and full offline repository gates are required before completion is
  recorded.

## Compatibility and deferrals

No Ling behavior or public contract changes. Future Stable examples,
profile/target policy, public manifest/schema commitments, cross-host fixtures,
future feature examples, and G6 sign-off remain deferred.
