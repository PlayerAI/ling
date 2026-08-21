# INC-1407 Implementation Report: Clean/Incremental Equivalence

## Outcome

INC-1407 is complete. `ling-db` now has executable test evidence that an
incrementally updated compiler database and a freshly rebuilt database publish
the same checked results for a deterministic edit sequence. The harness is
test-only: it does not add a public query, cache, scheduler, or wire protocol.

## Normative traceability

- Accepted `DEC-0019` §§2, 4, and 7 authorize immutable VFS snapshots,
  deterministic dependency keys/traversal, and observable query outcomes for
  clean/incremental equivalence.
- Existing `ling-types`, `ling-effects`, `ling-semantic`, `ling-format`, and
  `ling-eval` contracts remain authoritative for checked Typed Core, effects,
  canonical semantic JSON, audit formatting, and interpreter behavior. The
  harness compares those existing outputs and does not reinterpret them.
- Accepted `DEC-0019` §6 keeps persistent caching, corruption recovery,
  parallel scheduling, and compiler-facing cancellation outside this slice.

## Implemented evidence

- `clean_database` rebuilds a new `CompilerDb` from the current canonical VFS
  snapshots and workspace inputs; `clean_file` maps the original logical name
  to the clean database without exposing physical paths.
- `clean_and_incremental_pipelines_match_across_deterministic_edit_sequence`
  compares the initial source and successive literal-body, CRLF/comment, and
  later-body edits. Each step compares:
  - module type/effect projections;
  - canonical semantic graph JSON;
  - canonical audit formatter output;
  - interpreter result, runtime diagnostic JSON, and console output.
- `clean_and_incremental_diagnostics_match_for_invalid_effects` verifies that
  structured effect-checking failures are equal between incremental and clean
  databases, including their stable diagnostics and source spans.
- The harness intentionally compares semantic/public outputs rather than
  internal `SourceId` or allocation identity, so clean rebuilds may assign
  different internal IDs without becoming observable language behavior.

## Compatibility and deferred work

- No language syntax or semantics, diagnostic allocation, schema, Semantic ID
  algorithm, CLI/LSP field, public protocol, persistence format, or Unicode
  table changed.
- Only existing repository-owned crates were added as `ling-db` development
  dependencies (`ling-format` and `ling-eval`) for test evidence; production
  dependencies and public APIs are unchanged.
- Deterministic parallel scheduling (INC-1408), persistent cache decisions
  (INC-1409), and performance baselines remain deferred under the open
  incremental-cache gap.

## Validation

The following gates passed:

- `cargo fmt --all -- --check`
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`
- `cargo test -p ling-db --all-targets --locked --offline` (16 tests)
- `cargo test --workspace --all-targets --locked --offline`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `cargo xtask traceability verify --release v0.0.1`
- `cargo xtask support verify`

INC-1408 is the next execution-plan item, but its parallel-scheduling design
remains blocked until the required accepted authority is available.
