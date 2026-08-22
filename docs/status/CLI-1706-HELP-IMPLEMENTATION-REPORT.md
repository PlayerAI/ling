# CLI-1706-HELP implementation report

## Result

`CLI-1706-HELP` is complete as the bounded internal help-truth fixture. The
broader `CLI-1706` task remains `BlockedSpec`: shell completion, future command
and option inventories, help lifecycle/versioning, locale policy, quoting, and
help compatibility are not accepted contracts.

## Authority and boundary

- `DEC-0040` accepts only a regression boundary for the current `PROTO-CLI`
  help behavior.
- The internal `Command` catalog is the source for the implemented-command
  test set; it is not a public registry or completion source.
- `--help`/`-h` coverage is semantic rather than a byte-level help promise.
- Unknown future commands retain the existing invalid-usage exit class and
  stderr usage path.

## Implementation

- `crates/ling-cli/src/command_catalog.rs` exposes a test-only complete list of
  implemented command values and checks uniqueness.
- `crates/ling-cli/src/main.rs` checks that usage text mentions every catalogued
  command and excludes stale `build`, `query`, `patch`, `zero`, and `.zero`
  spellings.
- `crates/ling-cli/tests/help.rs` runs independent `--help`, `-h`, and unknown
  `query` process fixtures, checking streams and exit class without snapshotting
  help layout.

## Verification

Executed locally, offline:

- `cargo fmt --all`
- `cargo test -p ling-cli --all-features --locked --offline --quiet`

The focused CLI suite passed. Full workspace and governance gates are run at
the milestone integration boundary.

## Compatibility and deferrals

No language syntax, semantics, diagnostics, schemas, Semantic IDs, runtime,
bytecode, VM, dependencies, or Unicode 17.0.0 behavior changed. Shell-specific
completion, aliases, generated scripts, hidden/deprecated command policy,
locale/order guarantees, help snapshots, and future commands remain deferred
to later Accepted authority.
