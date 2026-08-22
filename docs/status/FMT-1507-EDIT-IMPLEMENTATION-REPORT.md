# FMT-1507-EDIT implementation report

## Outcome

The formatter now exposes a deterministic in-process whole-document edit
projection. A safely formatted valid source yields one replacement over the
exact original UTF-8 byte span; unchanged, invalid, and rejected sources yield
no edit. The parent `FMT-1507` task remains `BlockedSpec`: this slice does not
implement LSP methods, Workspace Edits, range formatting, format-on-save, or
Semantic Transactions.

Implementation commit: `e33f8496de369b8fc6364007a59a0a1fb4ca9e9f`.

## Normative traceability

- Accepted `DEC-0057` §§1–4 authorizes the `FormatEdit` value and
  `format_core_edit` function, including the no-edit and oversized-source
  boundaries.
- Accepted `DEC-0023` continues to govern the Format IR and conservative
  formatter publication decision.
- Accepted `DEC-0002` continues to govern original UTF-8 byte spans.
- `GAP-FORMATTER-CLI-PROTOCOL-001` and `GAP-LSP-TRANSACTION-PROTOCOL-001`
  remain open; no public wire behavior was inferred from the execution plan.

## Implementation

- Added `crates/ling-format/src/edit.rs` with `FormatEdit`,
  `FormatEditError`, and `format_core_edit`.
- Updated the repository governance/status count assertions so the new
  Accepted decision and Done task remain covered by aggregate tests.
- The implementation delegates candidate acceptance to the existing
  `format_core_with_disposition` boundary and emits exactly one replacement
  over the original source bytes when needed.
- The edit value is not connected to the CLI, `LspServer`, JSON-RPC transport,
  protocol inventory, or any source mutation path.

## Evidence

Executed locally against the implementation:

- `cargo fmt --all -- --check`;
- `cargo test -p ling-format --all-features --locked --offline` (24 tests);
- `cargo clippy -p ling-format --all-targets --all-features --locked --offline -- -D warnings`;
- `git diff --check`.

The tests cover changed and unchanged valid sources, invalid-source no-op
behavior, exact BOM/CRLF/Unicode byte ranges, source identity, and replacement
text.

## Compatibility, determinism, and Unicode

No language syntax, Typed Core, runtime, bytecode, diagnostics, schemas,
Semantic IDs, canonical Audit Source bytes, CLI behavior, LSP wire method,
protocol registration, dependency, or Unicode 17.0.0 data changed. The edit
range is derived from the original source byte length and is independent of
paths, allocation order, host memory, or debug formatting.

## Intentionally deferred

Minimal diffs, range formatting, URI and document-version association,
position encoding, stale-edit guards, `TextEdit`/`WorkspaceEdit` serialization,
format-on-save, public LSP publication, and Semantic Transaction lifecycle
remain deferred to the blocked parent and its registered specification gaps.
