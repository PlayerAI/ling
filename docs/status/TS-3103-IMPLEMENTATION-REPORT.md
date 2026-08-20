# TS-3103 offside/layout scanner implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `28750bcbd458322e856cf45842b8241047a8e41b`
> Verified baseline: `main@70e44ea72d6cfd70ed988c15cba5738c2fd9c38e`

## Outcome

TS-3103 replaces the provisional newline-plus-space approximation in [`tree-sitter-ling`](../../editors/tree-sitter-ling/) with a stateful external scanner implementing the editor-facing layout mechanics required by Accepted DEC-0006. Relative indentation, sibling newlines, nested dedents, EOF closure, same-column match/variant cases, line-leading pipelines, delimiter-local soft newlines, blank/comment-only lines, and nested block comments now have explicit grammar and scanner behavior.

The complete incremental state is serialized and validated. All 29 grammar corpus cases, nine dedicated layout integration scenarios, scanner-state tests, the package example, and all four shared root examples pass without unexpected `ERROR` or `MISSING` nodes. Invalid over-depth and unclosed block comments terminate with finite error trees.

This remains a tolerant editor parser. It does not decide Ling validity, emit diagnostics, lower an AST, or modify language semantics. Accepted specifications and the compiler retain their established authority.

## Normative clauses and decisions covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3103: an external scanner or equivalent, compiler-owned tab diagnostics, error recovery, incremental reparsing, serializable scanner state, and coverage for CRLF, blank/comment-only lines, delimiter newlines, and EOF dedents.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): relative offside columns; `Newline`, `Indent`, `Dedent`, and `SoftNewline`; same-column match cases and pipelines; blank/comment-only handling; delimiter, layout, and comment depth 256; nested block comments; and finite EOF recovery.
- Accepted [`DEC-0002`](../decisions/0002-source-position-units.md): LF, CRLF, and lone-CR inputs are exercised without changing the compiler's original UTF-8 byte-span authority.
- Accepted [`DEC-0004`](../decisions/0004-pipeline-syntax.md): a line-leading `|>` continues only at the current pipeline column.
- Accepted [`DEC-0005`](../decisions/0005-seed-literals-and-delimiters.md): `()`, `[]`, and `{}` are the delimiter forms whose internal newlines become soft.

Draft RFC-0001, SEMANTICS, and LANGUAGE remain lower-authority Seed inputs. No Draft clause is promoted by this implementation.

## Implementation

- [`scanner.c`](../../editors/tree-sitter-ling/src/scanner.c) owns nine private external symbols in a fixed order: four layout tokens, a same-indent line-leading-bar token, nested block comments, two zero-width delimiter-state markers, and a recovery sentinel.
- The scanner stores a strictly increasing indentation stack rooted at column zero plus delimiter depth. It caps layout, delimiter, and block-comment nesting at 256.
- LF, CRLF, and lone CR are recognized as one logical newline. Blank lines are skipped for indentation comparison; comment-leading lines are inspected without consuming the comment node.
- A same-column line-leading bar token avoids the grammar ambiguity between a trailing newline and another match/variant case or pipeline continuation. The normal lexer still owns `|` and `|>`.
- Zero-width `_delimiter_open` and `_delimiter_close` grammar markers update scanner depth around every `()`, `[]`, and `{}` form. This provides reliable soft-newline context even when a closing literal is not yet valid in the current parser state.
- Nested block comments are scanned as one retained CST extra. Depth 256 succeeds; depth 257 and unclosed input return control to bounded Tree-sitter recovery.
- Scanner serialization version 2 stores a little-endian stack length, delimiter depth, and every indentation column. Its maximum 517-byte payload remains below Tree-sitter's 1,024-byte scanner-state limit. Deserialization rejects wrong versions, truncated lengths, excessive depth, nonzero roots, and non-monotonic stacks by resetting to root state.
- Tabs count as one recovery column, and inconsistent dedents recover toward the nearest lower stored indentation. The scanner emits no diagnostic; `ling-syntax` remains the only validity and registered bilingual diagnostic authority.
- [`ADR-0001`](../../editors/tree-sitter-ling/docs/ADR-0001-layout-scanner.md) records the non-normative engineering contract, including symbol order, marker placement, serialization format, limits, and rejected alternatives.
- Package tests compile the scanner directly with C11 warnings as errors and drive the pinned Tree-sitter CLI through exact-byte integration fixtures. No new dependency was added.

The implementation follows KISS by keeping one scanner with one compact state model, DRY by leaving diagnostics and semantic validity in the compiler, YAGNI by adding no public bindings or future syntax, and SRP by separating grammar structure, scanner mechanics, state tests, and CLI integration tests.

## Specification gaps or conflicts

- No new language-specification gap was found. DEC-0006 defines every language behavior needed by this task.
- The initial engineering hypothesis that valid closing literals alone could identify delimiter context was disproved by record-update states where a field is required and `}` is intentionally invalid. The package-local ADR was corrected to use serialized zero-width delimiter markers. This is an implementation design correction, not a language semantic change.
- [`GAP-GOV-RFC-STATUS-001`](spec-gaps/GAP-GOV-RFC-STATUS-001.md) remains open; the grammar does not stabilize Draft surrounding syntax.
- [`GAP-SEED-BOOLEAN-OPERATORS-001`](spec-gaps/GAP-SEED-BOOLEAN-OPERATORS-001.md) remains open and continues to block TS-3105 from selecting `&&`/`||` syntax or precedence.
- Tree-sitter recovery columns are bounded to 65,535 leading characters by the private `uint16_t` representation. The compiler uses its own authoritative source/layout representation and diagnoses pathological input.

## Tests and verification

Executed locally on 2026-08-20 against the implementation ending at `28750bcbd458322e856cf45842b8241047a8e41b`:

- Test-first layout corpus — six targeted cases were introduced before the scanner; five failed under the provisional TS-3102 layout token, and the scanner test failed because no scanner existed.
- `npm run verify` — generated the parser, passed 29/29 corpus cases, passed scanner-state and layout-integration suites, and parsed the package Hello World example.
- Layout integration — nine scenarios cover CRLF, lone CR, no-final-newline EOF dedents, line and nested-block comment-only lines, comment depth 256/257, unclosed comments, record-update soft newlines, and an actual incremental edit/reparse.
- Scanner-state tests — complete state round-trip, maximum layout serialization, maximum delimiter serialization, delimiter boundary enforcement, and corrupt/truncated/non-monotonic state rejection passed under `-std=c11 -Wall -Wextra -Werror`.
- Shared examples — `adt-match.ling`, `hello.ling`, `pipeline.ling`, and `人物.ling` all parsed successfully.
- Generated-parser idempotence — six generated files, zero SHA-256 changes after regeneration.
- Named-node coverage — 60 generated named CST node types, zero absent from corpus expectations.
- Locked dependency audit — still only `tree-sitter-cli@0.26.12`.
- `cargo xtask governance check-all` — five checks, 42 documents, 26 gaps, 17 lifecycle records, 18 protocols, and 56 diagnostic codes.
- `cargo xtask schema validate-all`, compatibility, and corrupt-input checks — three schemas, four valid fixtures, six invalid fixtures, one canonical fixture, three `NoPreviousVersion` records, and 23 deterministic mutations passed.
- `cargo xtask traceability verify --release v0.0.1`, `support verify`, `status verify`, `ci verify`, and `seed reproduce` — all passed; status contains 14/14 Done tasks, and Seed reproduction compared 41,866 bytes across four surfaces and eight processes.
- `cargo fmt --all -- --check` and Clippy with `-D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 234 tests passed, plus doc-test harnesses.
- Rust 1.85 MSRV check, workspace documentation build, and release build — passed offline and locked.
- Execution-plan checksums — all 27 entries matched after the backlog transition.
- Active Markdown targets — 773 local targets across 84 files resolved; frozen baseline and installed npm dependencies were excluded.
- `git diff --check` — passed.

## Compatibility impact

- Language semantics, compiler lexer/parser, AST/HIR/Typed Core, and evaluator: unchanged.
- Public diagnostics: unchanged; no error code, severity, bilingual template, Fact, Repair, or original UTF-8 byte span changed.
- Public schemas, protocols, CLI behavior, and ABI: unchanged.
- Semantic IDs and canonical bytes: unchanged; Tree-sitter CST nodes, private markers, and serialized scanner state are excluded from identity inputs.
- Tree-sitter internal compatibility: external-symbol order and scanner serialization changed from no scanner to private version 2. No Stable Tree-sitter node or scanner-state compatibility promise exists.
- Dependencies: unchanged; normal Rust builds and tests remain offline after locking.

## Determinism and Unicode

The generated parser is byte-idempotent under the pinned CLI. Scanner output depends only on source bytes, parser-valid symbol sets, and fully serialized indentation/delimiter state. It does not expose allocation identity, host paths, timestamps, hash-map order, or debug output.

Ling remains pinned to Unicode 17.0.0. TS-3103 changes no identifier table, NFC rule, confusable policy, security diagnostic, or source-span calculation. Tree-sitter's current Unicode property token remains an explicitly documented approximation until TS-3104.

## Intentionally deferred

- TS-3104 generated Unicode 17.0.0 identifier parity, exact reserved-word behavior, and compiler lexer differential evidence. This is the next Ready task.
- TS-3105 exhaustive precedence-pair evidence; boolean operators remain `BlockedSpec`.
- TS-3106 complete pattern/type edge coverage, TS-3107 systematic malformed-edit recovery, and TS-3108 synchronized compiler/Tree-sitter differential testing.
- Tree-sitter queries, Zed extension packaging, LSP integration, formatter integration, language-specific bindings, and publication.
- Any Stable Tree-sitter CST, external-symbol, or scanner-serialization compatibility guarantee.
