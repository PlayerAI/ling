# TS-3104 Unicode identifier implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `16e61caf1340611c4752196b47da2973aca6978b`
> Verified baseline: `main@108042a584412fbae8e3dbfba4352e47a079dbd5`

## Outcome

TS-3104 removes the Tree-sitter grammar's dependency on the CLI's own `XID_Start` and `XID_Continue` property version. The repository Unicode generator now emits the exact 779 start ranges and 1,422 continuation ranges from the committed, checksum-verified Unicode 17.0.0 `DerivedCoreProperties.txt`. A small editor adapter adds Ling's `_` start rule and renders explicit Rust-regex scalar ranges for `grammar.js`.

All Seed keywords are globally reserved. The otherwise syntax-deferred `and` terminal is kept reachable through a private non-completing error rule, so exact `and` cannot fall back to adjacent identifier prefixes while `a`, `an`, and `and_then` remain identifiers. This adds no recursive-binding production.

A shared 18-case corpus is consumed independently by the compiler lexer and the generated Tree-sitter parser. It covers ASCII, Chinese, `_`, NFC-equivalent spellings, combining continuations, supplementary-plane and Unicode-17 upper boundaries, emoji, compiler-only forbidden XID continuations, mixed-script metadata, and `and`/`and_then` token boundaries.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3104: evaluate grammar regex coverage, prefer generated Unicode 17.0.0 ranges, establish compiler/Tree-sitter token differential evidence, preserve compiler/LSP diagnostic authority, and keep security checks out of queries.
- [`SEMANTICS.md`](../SEMANTICS.md) §3.3–3.7: Unicode 17.0.0, `XID_Start | _`, `XID_Continue`, NFC name identity, forbidden properties, confusable/mixed-script policy, and Seed keywords.
- [`LANGUAGE.md`](../LANGUAGE.md) §5.1–5.7: first-class multilingual identifiers, XID shape, NFC comparison, forbidden characters, and identifier security.
- Accepted [`DEC-0012`](../decisions/0012-semantic-identity-and-canonical-bytes.md): normalized names affect definition identity, while recursive `let rec ... and ...` groups remain deferred.

Tree-sitter remains an editor CST implementation, not a language authority. No Draft clause is promoted by this work.

## Implementation

- [`unicode-gen`](../../tools/unicode-gen/src/main.rs) selects and validates Unicode 17.0.0 XID ranges in the same checksum-verified database load used by compiler security tables. Validation rejects empty, unordered, overlapping, non-scalar, or non-subset data and checks the underscore assumptions.
- The default generator invocation now writes both the existing Rust security table and [`unicode-identifiers.generated.js`](../../editors/tree-sitter-ling/src/unicode-identifiers.generated.js). CI rejects drift in either output.
- [`unicode-identifiers.js`](../../editors/tree-sitter-ling/src/unicode-identifiers.js) contains only regular token mechanics: merge `_`, render `\x{...}` classes, and compose start-plus-continuation. It performs no normalization or security policy.
- [`grammar.js`](../../editors/tree-sitter-ling/grammar.js) consumes the generated explicit pattern. The generated grammar contains no `\p{XID_*}` property escape.
- A private `_reserved_and_error` sequence makes the `and` terminal eligible for global keyword extraction but requires the external scanner's non-emitting error sentinel. It cannot complete as successful Ling syntax and creates no named CST node.
- [`unicode-identifiers.tsv`](../../editors/tree-sitter-ling/test/fixtures/unicode-identifiers.tsv) is the single differential case list. Rust tests assert compiler tokens, NFC values, diagnostics, bilingual JSON, and original UTF-8 byte spans; the Node runner asserts generated UCD equality and actual Tree-sitter parse results.
- [`ADR-0002`](../../editors/tree-sitter-ling/docs/ADR-0002-unicode-identifiers.md) records the editor-only design, authority boundary, reproducibility contract, and rejected alternatives.

The implementation follows KISS by using regular generated ranges instead of another scanner responsibility, DRY by sharing one corpus and one pinned UCD, YAGNI by adding no query security logic or future syntax, and SRP by separating generated data, regex adaptation, grammar structure, and compiler diagnostics.

## Specification gaps or conflicts

- No new language-specification gap was found. The identifier and keyword behavior required here is already defined by SEMANTICS and Accepted DEC-0012.
- Tree-sitter's supported Rust-regex subset has no lookaround. A first regex-only attempt to exclude `and` still permitted prefix tokenization as `an` plus `d`; the shared differential test caught this, and the implementation moved exact exclusion to the grammar's global reserved-word mechanism.
- XID_Continue itself includes characters such as join controls and variation selectors that Ling later forbids. This is an intentional lexer/security layering boundary, not a parity defect.
- No `ling-lsp` adapter exists yet. The compiler-side differential test proves that every permissive-editor/compiler-rejected case produces the stable `L-LEX-0004` payload, bilingual messages, and complete original-byte span that LSP-2201 must convert. TS-3104 does not claim an implemented LSP transport.
- [`GAP-SEED-BOOLEAN-OPERATORS-001`](spec-gaps/GAP-SEED-BOOLEAN-OPERATORS-001.md) remains open and continues to block TS-3105's boolean precedence decision.

## Tests and verification

Executed locally on 2026-08-20 against implementation commit `16e61caf1340611c4752196b47da2973aca6978b`:

- `npm run verify` with tree-sitter-cli 0.26.12 — 29/29 grammar corpus cases, scanner-state tests, nine layout integration scenarios, 18 shared Unicode differential cases, and the package example passed.
- Shared differential — 18/18 cases agreed with their explicit compiler and editor expectations; exact `and` was rejected by Tree-sitter, `and_then` remained valid, and compiler-only security cases retained editor CST structure.
- Generated UCD equality — all 779 `XID_Start` ranges and 1,422 `XID_Continue` ranges matched the pinned source in order and value.
- Exhaustive compiler XID conformance — every Unicode scalar was compared with the same Unicode 17.0.0 UCD; NFC conformance also passed.
- Generated artifact idempotence — the Rust Unicode table, JavaScript XID table, grammar JSON, node types, C parser, and three Tree-sitter headers retained identical SHA-256 hashes after regeneration.
- Dependency audit — the editor package still has one dependency, tree-sitter-cli 0.26.12.
- `cargo test --workspace --all-features --locked --offline` — 237 tests passed, including the new compiler differential and four generator tests; all doctests passed.
- `cargo fmt`, full Clippy with warnings denied, Rust 1.85 workspace check, docs build, and release build passed.
- Governance, schema, traceability, support, CI-contract, status, and deterministic Seed-reproduction gates passed. Seed output remained 41,866 compared bytes.

## Compatibility, determinism, and Unicode impact

- **Diagnostics:** no code, severity, payload schema, or wording changed. Existing `L-LEX-0004` is now directly exercised as the editor/LSP authority boundary.
- **Schemas and protocols:** no public schema or protocol marker changed. The generated JavaScript table is a private editor build artifact.
- **Semantic IDs and canonical bytes:** unchanged. Tree-sitter tokens and generated ranges do not enter compiler Semantic Graph or identity inputs.
- **Source positions:** compiler original UTF-8 byte spans are unchanged and explicitly asserted for multi-byte rejected identifiers.
- **Determinism:** both Unicode outputs and all generated parser files are byte-idempotent; the CI contract now checks the editor Unicode table.
- **Unicode:** the language and compiler remain at 17.0.0. Editor XID tokenization is now pinned to that same version instead of inheriting the Tree-sitter CLI's property database.

## Intentionally deferred work

- LSP protocol adaptation, UTF-8/UTF-16 position conversion, and publication remain LSP-2101/LSP-2102/LSP-2201/LSP-2202.
- Query-level highlights and semantic-token refinement remain ZQ-3201 and LSP-2401 onward; no query attempts security or confusable classification.
- Exhaustive expression precedence remains TS-3105 and is blocked by the accepted-syntax gap for boolean operators.
- Pattern/type edge coverage, malformed edit recovery, and whole-program compiler/Tree-sitter differential remain TS-3106, TS-3107, and TS-3108 respectively.
