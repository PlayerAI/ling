# ZQ-3202 bracket matching implementation report

> Status: **Done**
> Completed: 2026-08-21
> Final verified implementation commit: `1106b323685ed4910e6580a4347dce47df466208`
> Verified baseline: `main@1106b323685ed4910e6580a4347dce47df466208`

## Outcome

ZQ-3202 adds the shared [`brackets.scm`](../../editors/tree-sitter-ling/queries/brackets.scm) editor query for `()`, `[]`, `{}`, and complete string quotes. It exposes only Zed's `@open` and `@close` captures, excludes string pairs from rainbow coloring, ignores escaped quotes, and deliberately treats nested block comments as opaque rather than inspecting bracket-like text inside them.

Three bounded fixtures contain 20 positive/negative assertions covering each pair class, nested structural pairs, escaped quotes, nested Chinese source, a nested block comment, and an emoji-prefix recovery tree that retains a valid canary pair. The query and its test runner are part of the locked `npm test` and `npm run verify` paths.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) ZQ-3202: pair `()`, `[]`, `{}`, and string quotes; test nested-comment behavior before choosing a policy; optionally exclude quotes from rainbow matching.
- [`SEMANTICS.md`](../SEMANTICS.md) §§3.8 and 29: nested block comments and the Seed literal/delimiter surface.
- [`grammar-map.md`](../grammar-map.md) §§2–4 and 7: anonymous delimiter tokens, `escape_sequence`, opaque external-scanner comments, query-friendly CST structure, and recovery boundaries.
- Accepted [`DEC-0005`](../decisions/0005-seed-literals-and-delimiters.md): the exact accepted parenthesis, list, record, string, and escape spellings.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): nested block-comment recognition and compiler-owned delimiter/comment diagnostics.

The query is an editor aid only. It does not define delimiter validity, synthesize source, or alter the compiler grammar.

## Implementation

- [`brackets.scm`](../../editors/tree-sitter-ling/queries/brackets.scm) contains exactly four reviewed sibling-token pair patterns and no language-semantic predicates.
- The quote pattern carries `#set! rainbow.exclude`. Only the two anonymous boundary quote tokens match; a `\"` spelling remains a named `escape_sequence` and cannot become a pair endpoint.
- The query contains no `block_comment` pattern. The external scanner emits an entire nested comment as one named node, so raw braces, brackets, parentheses, and quotes inside it produce zero captures.
- [`run-bracket-tests.js`](../../editors/tree-sitter-ling/test/run-bracket-tests.js) locks the fixture, capture, pair, and assertion inventories; verifies clean/recovery parse policy; bounds process time and output; executes standard query assertions twice; rejects nondeterministic output; and independently requires zero captures from the nested-comment fixture.
- [`basics.ling`](../../editors/tree-sitter-ling/test/fixtures/brackets/basics.ling) covers all pairs, nested pairs, escaped quotes, and a nested Chinese expression. [`comments.ling`](../../editors/tree-sitter-ling/test/fixtures/brackets/comments.ling) contains an actual nested block comment with bracket-like text. [`recovery.ling`](../../editors/tree-sitter-ling/test/fixtures/brackets/recovery.ling) proves a valid pair remains queryable after an emoji-prefix error.
- The fixtures live below `test/fixtures/brackets/` because Tree-sitter reserves a top-level `test/brackets/` directory for a different automatic query-test convention. The explicit runner owns these Ling source fixtures and is included in `npm test`.
- [`package.json`](../../editors/tree-sitter-ling/package.json) packages the pair fixtures and adds `test:brackets`; the editor and repository READMEs and [`KNOWN-DIFFERENCES.md`](../../editors/tree-sitter-ling/KNOWN-DIFFERENCES.md) record the supported boundary.

The implementation follows KISS by using four direct pair patterns, SRP by leaving validity and recovery diagnostics in the compiler/parser, DRY by sharing each fixture between standard query assertions and the contract runner, and YAGNI by declining comment-content parsing or missing-delimiter inference.

## Specification gaps or conflicts

- No unresolved semantic or public-protocol gap was encountered. ZQ-3201 and TS-3108 satisfy the declared editor prerequisites, and no registered gap blocks bracket matching.
- The plan intentionally left nested block-comment participation undecided. Executable evidence shows the scanner exposes nested comments as opaque nodes; inspecting their bytes in a query is neither available nor desirable. ZQ-3202 therefore excludes comments from matching, preventing false pairs inside comments without duplicating scanner logic.
- The plan permits disabling rainbow coloring for quotes rather than requiring it. This implementation disables it because the entire Text literal is one logical quote pair, and treating each string as another nesting depth adds visual noise without structural information.
- Incomplete delimiters remain parser recovery. The query does not fabricate a close token or claim a source is valid; the recovery fixture checks only that an independent complete pair remains usable.
- The lower-authority Zed plan's stale `zero` command examples are unrelated to bracket queries and did not enter implementation.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `1106b323685ed4910e6580a4347dce47df466208`:

- `npm run verify` with tree-sitter-cli 0.26.12 — all 41 grammar corpus cases, scanner/layout integrations, 18 Unicode cases, 29 precedence cases, 41 Pattern/Type cases, 10 static recovery cases, 9 incremental edits, 64 recovery mutations, 42 whole-program differential cases, 84 whole-corpus edits, 43 stable mappings, 3 highlight fixtures with 46 assertions, the 18-capture highlight contract, 3 bracket fixtures with 20 assertions, the 4-pair bracket contract, and the local example pass.
- ZQ-3202 runner — exact two-capture, four-pair, three-fixture, and assertion inventories pass; two query executions have identical normalized output/diagnostics; clean fixtures have no `ERROR`/`MISSING`; recovery remains finite; nested block-comment query output contains zero pair captures.
- Generated parser idempotence — `npm run verify` regenerated the parser and left all generated grammar/parser/scanner/header and Unicode identifier artifacts unchanged.
- `cargo xtask governance check-all` — 43 documents, 26 gaps, 18 lifecycle records, 18 protocols, and 56 diagnostic codes pass.
- Schema validation, N-1 compatibility declarations, 23 deterministic corrupt-input checks, traceability, support, CI-contract, Seed-reproduction, and the 20-task implementation-status gate pass.
- All 27 execution-plan checksums match after the backlog transition; 971 local inline links across 93 active Markdown files resolve to repository targets.
- `cargo test --workspace --all-features --locked --offline` passes all workspace tests, including 91 xtask tests.
- `cargo fmt --all -- --check`, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, release build, and `git diff --check` pass.

No remote CI result or live Zed visual smoke is claimed. Live pair behavior in Zed belongs to the grammar-only development extension milestone.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** no Ling error code, severity, bilingual message, typed Fact, Repair, or original UTF-8 byte-span behavior changed. Queries emit no Ling diagnostic.
- **Schemas and protocols:** no public schema, CLI contract, exit class, ABI, dependency, or grammar node changed. The two query captures remain an Experimental editor surface.
- **Language behavior:** unchanged; bracket matching neither parses nor validates Ling source.
- **Semantic IDs and canonical bytes:** unchanged; pair captures and rainbow metadata are excluded from semantic identity.
- **Determinism:** pair/capture/fixture/assertion inventories are exact; processes are bounded; query output is normalized and compared across two executions; nested-comment exclusion is independently checked.
- **Unicode:** Unicode remains 17.0.0. A nested Chinese expression exercises byte-offset traversal, and an emoji-prefix recovery case retains the next valid pair without changing compiler security rules.

## Intentionally deferred work

- ZQ-3203 owns indentation captures and their interaction with offside blocks; bracket matching does not prescribe indentation.
- Highlighting remains ZQ-3201; outline, text objects, runnables, overrides, injections, and redactions remain later query tasks.
- Live Zed file recognition, theme rendering, rainbow behavior, and extension configuration remain ZEXT-3301.
- Missing-delimiter diagnostics and any future delimiter syntax remain compiler/specification work, not bracket-query heuristics.
