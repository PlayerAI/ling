# TS-3105 Expression precedence implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `cf76a4268b5ec8d5cdd939749709cc0654cff732`
> Verified baseline: `main@10c4c5274a9d4f504d743c9d9af85fdfeced9111`

## Outcome

TS-3105 implements the complete Seed expression-precedence contract in both the authoritative compiler and the editor-only Tree-sitter grammar. The low-to-high order is assignment, pipeline, `||`, `&&`, equality, comparison, additive, multiplicative, application, projection, unary, and primary. Pipeline, binary operators, application, and projection associate left; unary operators associate right; an unparenthesized expression may contain only one assignment layer.

`&&` and `||` now lower to distinct AST/HIR/checked representations, require `Bool` operands, retain both operands for static effect and capability checking, and evaluate the left operand exactly once. Runtime evaluation skips the right operand when the left value determines the result, including any effect or `Fault` that the skipped expression would otherwise produce.

A shared 29-case table drives independent compiler-AST and generated Tree-sitter-CST renderers. It covers both groupings of every neighboring precedence pair, associativity, `f - x`, `f (-x)`, and the decision that textual `or` remains an ordinary identifier. The Tree-sitter public named CST remains shallow while private structural rules make precedence explicit rather than relying on generator conflict resolution.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3105: application, projection, unary, arithmetic, comparison, equality, boolean, pipeline, and assignment precedence, with explicit fixtures for each neighboring pair.
- Accepted [`DEC-0017`](../decisions/0017-seed-boolean-operators.md): `&&` and `||`, complete precedence and associativity, one unparenthesized assignment layer, left-once short-circuit evaluation, checked right operands, `f - x`, `f (-x)`, and no textual aliases.
- Accepted [`DEC-0004`](../decisions/0004-pipeline-syntax.md): pipeline syntax and its application-oriented lowering boundary.
- Accepted [`DEC-0009`](../decisions/0009-seed-borrow-and-mutation-boundary.md) and [`DEC-0010`](../decisions/0010-state-and-capability-model.md): checked place assignment and statically visible effects/capabilities.
- [`SEMANTICS.md`](../SEMANTICS.md) §8.3 and §13: evaluation order, short-circuiting, operator typing, and checked execution.

Tree-sitter remains an editor CST implementation, not a language authority. No Draft clause is promoted by this work.

## Implementation

- [`ling-syntax`](../../crates/ling-syntax/src/parser.rs) inserts explicit boolean-or and boolean-and levels between pipeline and equality. Assignment right-hand sides use the non-assignment expression entry, so a second unparenthesized `<-` is rejected.
- [`ling-ast`](../../crates/ling-ast/src/lib.rs) and [`ling-hir`](../../crates/ling-hir/src/lib.rs) append distinct `BooleanAnd` and `BooleanOr` operators without changing existing variant behavior.
- [`ling-types`](../../crates/ling-types/src/lib.rs) infers both operands before constraining each to `Bool` at its original operand span.
- [`ling-effects`](../../crates/ling-effects/src/lib.rs) continues to traverse the entire checked expression, so a runtime-skipped right side still contributes static effects and capability requirements.
- [`ling-eval`](../../crates/ling-eval/src/lib.rs) handles boolean nodes before ordinary eager binary evaluation, evaluates the left side once, and evaluates the right side only when required.
- [`ling-semantic`](../../crates/ling-semantic/src/lib.rs) preserves existing binary tags `0..=10` and appends `BooleanAnd = 11` and `BooleanOr = 12`.
- [`grammar.js`](../../editors/tree-sitter-ling/grammar.js) uses private explicit precedence layers aliased to the existing public `binary_expression` CST. A dedicated application-argument projection layer distinguishes subtraction in `f - x` from the parenthesized signed argument in `f (-x)`.
- [`expression-precedence.tsv`](../../editors/tree-sitter-ling/test/fixtures/expression-precedence.tsv) is the single shared case list. Rust and Node runners independently render and compare the compiler AST and actual generated Tree-sitter CST.
- Six conformance fixtures cover accepted execution, operand typing, missing operands, assignment chaining, reserved `and`, and ordinary-identifier `or` behavior.

The implementation follows KISS by adding only the accepted operator levels, DRY by sharing one precedence table across independent parsers, YAGNI by omitting textual aliases and future syntax, and SRP by keeping parsing, typing, effects, evaluation, semantic identity, and editor CST concerns in their existing layers.

## Specification gaps or conflicts

- `GAP-SEED-BOOLEAN-OPERATORS-001` was resolved before implementation by Accepted DEC-0017. No language behavior was inferred from Tree-sitter or implementation snapshots.
- No unresolved conflict was found between DEC-0017, the existing pipeline/place decisions, and SEMANTICS. The compiler implementation was changed wherever its previous rejection behavior differed from the accepted decision.
- `and` is already a reserved future keyword and therefore remains a syntax error; `or` is not reserved and remains an ordinary identifier. Neither spelling is a boolean alias.
- Tree-sitter could otherwise reinterpret the second `<-` in an invalid assignment chain as comparison plus unary minus. A private non-completing recovery anchor preserves a finite error tree without creating successful language syntax.
- The public semantic schema does not enumerate operator tags. The new tags are private canonical inputs, appended so existing expressions and Semantic IDs remain unchanged.

## Tests and verification

Executed locally on 2026-08-20 against implementation commit `cf76a4268b5ec8d5cdd939749709cc0654cff732`:

- Shared precedence differential — all 29 cases matched in the compiler AST and generated Tree-sitter CST.
- Compiler syntax negatives — all four missing-operand forms fail; multi-byte source retains the exact original UTF-8 EOF span; unparenthesized assignment chains and reserved `and` fail; textual `or` parses only as application.
- Type/effect/runtime/Semantic-ID tests — Bool constraints use operand spans, skipped right sides remain statically checked, runtime evaluates the left side once and skips right-side effects/Faults, and old/new binary tags are stable and distinct.
- Conformance — all 38 registered fixtures pass, including six TS-3105 fixtures and their stable `L-SYNTAX-0010`, `L-NAME-0001`, and `L-TYPE-0001` expectations.
- `npm run verify` with tree-sitter-cli 0.26.12 — 31/31 grammar cases, scanner/layout integrations, 18 Unicode differential cases, 29 precedence differential cases, and the package example pass.
- Generated parser idempotence — grammar JSON, node types, C parser, and three Tree-sitter headers retain identical SHA-256 hashes after regeneration.
- `cargo test --workspace --all-features --locked --offline` — all workspace unit, integration, conformance, and documentation tests pass.
- Rust formatting, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, and release build pass offline with locked dependencies.
- Governance, schema, traceability, support, CI-contract, status, and deterministic Seed-reproduction gates pass.
- All 27 execution-plan checksums match after the backlog transition; 851 local inline links across 88 active Markdown files resolve to repository targets.

## Compatibility, determinism, and Unicode impact

- **Diagnostics:** no error code, severity, payload schema, or bilingual template changed. New fixtures reuse registered codes and assert original UTF-8 byte spans.
- **Schemas and protocols:** no public schema, protocol marker, command shape, or exit-code contract changed.
- **CLI behavior:** accepted `&&` and `||` programs now pass `check` and `run`; invalid operands and malformed expressions retain structured failures.
- **Semantic IDs and canonical bytes:** tags `0..=10` and IDs for all previously representable programs remain stable. Boolean nodes use the appended deterministic tags 11 and 12 and therefore produce distinct IDs.
- **Source positions:** compiler spans remain offsets in original UTF-8 bytes; the new multi-byte EOF and operand tests exercise this invariant.
- **Determinism:** precedence is structurally explicit in both parsers, shared expected groupings are fixed, semantic tags are stable, and generated parser artifacts are byte-idempotent.
- **Unicode:** Unicode tables, NFC/security behavior, and the pinned Unicode 17.0.0 version are unchanged.

## Intentionally deferred work

- Pattern/type detail coverage remains TS-3106.
- Malformed edit recovery beyond the TS-3105 operator cases remains TS-3107.
- Whole-program compiler/Tree-sitter differential and randomized edit coverage remain TS-3108.
- Interpreter/VM differential remains VM-1209 because no second checked execution engine exists.
- Textual boolean aliases, chained assignment, and any post-Seed operators remain intentionally unsupported unless a later Accepted RFC or decision adds them.
