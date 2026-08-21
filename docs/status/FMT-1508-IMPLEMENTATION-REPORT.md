# FMT-1508 Implementation Report: Audit Source Separation

## Outcome

FMT-1508 adds a deterministic regression property proving that Author Source
formatting does not replace or mutate canonical Audit Source rendering. Each
fixture is checked through the existing compiler-owned AST → HIR → resolver →
types → effects → Semantic Graph pipeline before and after formatting. The
resulting `ProgramSnapshot::audit_model()` is rendered by the existing
`ling-format::render_audit` function, and the complete canonical Audit Source
text must be byte-identical.

No CLI/LSP adapter, formatter command, range edit, format-on-save hook, JSON
report, diagnostic, schema, or second semantic authority was introduced.

## Normative traceability

- Accepted `DEC-0023` §8 requires Author Source formatting to remain disjoint
  from Audit Source and not change canonical Audit bytes, Semantic IDs, source
  spans, or evaluator inputs.
- Accepted `DEC-0015` remains the authority for Audit Source grammar, canonical
  ordering, and round-trip rendering.
- FMT-1508 in `03-G1-V0.1-LIVING.md` §7 requires a test proving that the
  formatter does not replace or change canonical Audit output.
- Accepted `DEC-0002` continues to govern original UTF-8 byte spans; the test
  does not normalize or publish source spans.

## Implementation

- Refactored the existing property helper into `checked_snapshot`, so semantic
  JSON and Audit assertions use exactly the same checked pipeline.
- Added `audit_source`, which renders only `ProgramSnapshot::audit_model()` via
  the existing canonical Audit renderer.
- Reused the fixed offline property corpus for the new byte-equivalence test:
  generic core syntax; CRLF, Unicode, documentation/trailing comments, and
  `Console.Write`; and nested multiline block comments.
- Updated `xtask` repository self-test counts to reflect the intentionally
  registered FMT-1507 gap and BlockedSpec task (27 gaps, 58 tasks, 57 Done).

## Tests and evidence

Executed locally on 2026-08-21 against implementation commit
`f247dfed98f104ea5227532965e8b579938a213e`:

- `cargo fmt --all -- --check`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo clippy -p xtask --all-targets --locked --offline -- -D warnings`;
- `cargo test -p ling-format --all-targets --locked --offline` (20 tests);
- `cargo test --workspace --all-targets --locked --offline` (all targets; 92
  `xtask` tests passed, with one project fixture test intentionally ignored);
- `cargo xtask governance check-all`;
- `cargo xtask status verify`; and
- `git diff --check`.

## Compatibility, determinism, and Unicode

The implementation changes only test helpers, an in-process property test, and
governance count assertions. It does not change language grammar, diagnostics,
Semantic IDs, canonical identity bytes, Audit renderer behavior, CLI/LSP
protocols, ABI, or Unicode 17.0.0 tables. The corpus is fixed and offline;
assertions compare exact canonical output and expose no paths, allocation
identity, map order, timestamps, or debug text.

## Specification gaps and intentionally deferred work

FMT-1507 remains `BlockedSpec` on
`GAP-FORMATTER-AUTHOR-SOURCE-001`,
`GAP-FORMATTER-CLI-PROTOCOL-001`,
`GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`. Those gaps still own formatter command,
stdin/check/report, range-edit, format-on-save, position, snapshot, and
transaction decisions. FMT-1508 deliberately does not infer or prototype any
of those public behaviors.
