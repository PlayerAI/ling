# FMT-1507-CLI implementation report

## Outcome

The dependency-complete formatter slice adds a Preview `ling fmt` command.
It formats one `.ling` file or UTF-8 standard input, never mutates the input,
and can emit either human-readable source or one `ling.format/0.1` JSON report.
Invalid UTF-8, lexical errors, parse errors, unreadable input, and rejected
formatter candidates produce a bounded diagnostic result without partial
formatted output.

The parent FMT-1507 task remains `BlockedSpec`: this slice does not implement
LSP, Workspace Edit, format-on-save, range formatting, or Semantic Transaction
behavior.

## Normative traceability

- Accepted `DEC-0028` §§1–3 defines the `ling fmt` command, `.ling` and stdin
  input boundary, logical stdin names, non-mutating stdout behavior, and the
  Preview `ling.format/0.1` report.
- Accepted `DEC-0028` §4 defines check-mode exit status: zero for unchanged,
  one for changes or invalid input, and two for usage errors.
- Accepted `DEC-0028` §5 defines the JSON fields, dispositions, diagnostics,
  and the rule that formatted text is omitted in check mode.
- Accepted `DEC-0002` continues to govern original UTF-8 byte spans. The CLI
  passes the original bytes to `SourceFile` and does not publish source edits.
- Accepted `DEC-0023` continues to govern the formatter IR and conservative
  candidate validation; the CLI consumes the existing `ling-format` boundary.

## Implementation

- Added `fmt` command parsing, `--check`, `--format human|json`, and
  `--stdin-name` validation to `crates/ling-cli/src/main.rs`.
- Added file/stdin loading, compiler-parser diagnostics, formatter IR
  execution, conservative candidate rejection, and stdout/report emission.
- Registered `PROTO-FORMAT-CLI` and `SCHEMA-FORMAT-REPORT-JSON` as Preview
  public surfaces, with a schema and valid/invalid fixtures under
  `schemas/format/0.1/`.
- Added parser unit tests covering file mode, stdin mode, and invalid input
  combinations.

## Tests and evidence

Executed locally on 2026-08-22 against the implementation worktree:

- `cargo check -p ling-cli --locked --offline`;
- `cargo test -p ling-cli --locked --offline` (29 tests across library,
  binary, and conformance targets);
- `cargo run -p ling-cli --locked --offline -- fmt --check examples/hello.ling`;
- `cargo run -p ling-cli --locked --offline -- fmt --format json examples/hello.ling`;
- `cargo run -p ling-cli --locked --offline -- fmt --format json --stdin-name stdin.ling -`;
- `cargo xtask schema validate-all`;
- governance authority, lifecycle, gap, and protocol checks; and
- `git diff --check` (to be repeated before commit).

## Compatibility, determinism, and Unicode

The command uses the existing parser and formatter IR, preserves original
source bytes for diagnostics, emits a single deterministic JSON object, and
does not expose filesystem ordering, allocation identity, Rust debug output,
or host paths as language semantics. No diagnostic code, Semantic ID rule,
Audit Source byte, compiler grammar, or Unicode 17.0.0 table changed.

## Specification gaps and intentionally deferred work

The following remain outside this slice:

- `GAP-FORMATTER-AUTHOR-SOURCE-001`: author-source boundary decisions for
  broader recovery and authoring behavior;
- `GAP-LSP-TRANSACTION-PROTOCOL-001`: LSP positions, versions, and Workspace
  Edit transaction fields; and
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`: Semantic Transaction lifecycle and
  compatibility policy.

Accordingly, FMT-1507 itself remains `BlockedSpec` even though its CLI
subtask is complete under DEC-0028.
