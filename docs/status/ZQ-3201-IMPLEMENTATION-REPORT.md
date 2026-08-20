# ZQ-3201 syntax highlighting implementation report

> Status: **Done**
> Completed: 2026-08-21
> Final verified implementation commit: `77aab24ff8160e1535ea15b67d5302c1a4bb3fc8`
> Verified baseline: `main@77aab24ff8160e1535ea15b67d5302c1a4bb3fc8`

## Outcome

ZQ-3201 adds the first shared Zed-facing Tree-sitter query for Ling without changing the language grammar or compiler. [`highlights.scm`](../../editors/tree-sitter-ling/queries/highlights.scm) exposes exactly 18 reviewed capture names for current Seed keywords, types, constructors, functions, parameters, variables, properties, literals, operators, comments, brackets, and delimiters.

The query remains deliberately syntactic. It assigns a specialized role only when the CST proves that role, uses conservative captures where resolution is required, and leaves later semantic-token work to refine names. Three standard Tree-sitter fixtures contain 46 passing highlight assertions and cover every capture, paired ASCII/Chinese structural roles, a decomposed combining identifier, an emoji-prefix recovery tree, and a negative assertion preventing the future `trait` word from being colored as active syntax.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) ZQ-3201: the complete requested base capture set, structural parity for Chinese identifiers, conservative syntax-only classification, a fixture for every capture, and no cosmetic future keyword support.
- [`SEMANTICS.md`](../SEMANTICS.md) §§3.3–3.8 and §29: Unicode identifiers, the current keyword set, comments, Seed literals, and the current type surface.
- [`grammar-map.md`](../grammar-map.md) §§2–8: anonymous keyword/punctuation tokens, shallow query-friendly CST roles, current expression/pattern/type nodes, recovery boundaries, and exclusion of post-Seed syntax.
- Accepted [`DEC-0005`](../decisions/0005-seed-literals-and-delimiters.md): current literal, escape, and delimiter spellings.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): comments/layout remain parser structure and compiler diagnostics remain authoritative.
- Accepted [`DEC-0014`](../decisions/0014-seed-prelude-option-result.md): `Option`/`Result` are nominal Prelude types and their constructors follow ordinary resolution rather than lexer magic.
- Accepted [`DEC-0017`](../decisions/0017-seed-boolean-operators.md): `&&`/`||` are operators; `and` is not a boolean alias.

The execution plan is engineering guidance, not language authority. The query adds no successful syntax production and cannot make a Tree-sitter recovery parse valid Ling.

## Implementation

- [`highlights.scm`](../../editors/tree-sitter-ling/queries/highlights.scm) contains one explicit current-keyword list; `true`/`false` are booleans rather than keywords. The lexically reserved `and` is colored as a keyword but remains finite error input, while post-Seed words such as `trait`, `impl`, `effect`, and `actor` are absent.
- Unqualified `Unit`, `Bool`, `Int`, `f64`, `Text`, and `List` in type positions receive `type.builtin`. User-defined and Prelude nominal names receive `type`; qualified names do not acquire primitive status merely because their final spelling resembles a built-in.
- Variant declarations and constructor-pattern CST nodes receive `constructor`. Bare expressions are not classified from capitalization or Prelude spelling because local resolution and shadowing can change their role.
- Direct call targets receive `function`; bare names remain `variable`. Function definitions use `@function @function.definition`, letting Zed try the rightmost definition style and fall back to the ordinary function style.
- Identifier patterns and name expressions provide the conservative variable baseline. Function parameters, record declarations/literals/patterns, and projected fields receive structural parameter/property captures; later, more-specific query patterns override the baseline.
- Literal nodes and escapes, all current operators, and context-sensitive punctuation/brackets are mapped directly from generated CST nodes and anonymous tokens. Generic angle brackets override their otherwise valid comparison-operator spelling only inside type argument/parameter lists.
- [`tree-sitter.json`](../../editors/tree-sitter-ling/tree-sitter.json) registers the query path. [`package.json`](../../editors/tree-sitter-ling/package.json) includes queries and highlight fixtures in its file set and makes the new runner part of `npm test`/`npm run verify`.
- [`run-highlight-tests.js`](../../editors/tree-sitter-ling/test/run-highlight-tests.js) locks the exact fixture, capture, keyword, and built-in-type inventories; checks clean versus recovery parse policy and process/output bounds; validates fallback ordering; executes all standard query assertions twice; and rejects nondeterministic output.
- [`basics.ling`](../../editors/tree-sitter-ling/test/highlight/basics.ling), [`unicode.ling`](../../editors/tree-sitter-ling/test/highlight/unicode.ling), and [`recovery.ling`](../../editors/tree-sitter-ling/test/highlight/recovery.ling) are normal Ling/query fixtures rather than snapshots of terminal colors or a particular theme.

The implementation applies KISS and SRP by using one query plus one bounded contract runner, DRY by making standard Tree-sitter fixtures serve both the built-in highlight test and the inventory runner, and YAGNI by declining namespace guesses, capitalization heuristics, semantic-token behavior, or future syntax.

## Specification gaps or conflicts

- The backlog still labeled ZQ-3201 “Blocked by G0/interface,” but no registered specification gap names this task, and completed TS-3108 supplies its only implementation dependency. Because highlighting is editor-only and introduces no language or public protocol semantics, the stale label was planning drift rather than an RFC gate.
- The plan requests `function.definition`, while [Zed's current language-extension documentation](https://zed.dev/docs/extensions/languages) does not name it in the standard theme-capture list. Zed officially resolves multiple captures right-to-left, so the required `@function @function.definition` pair preserves the requested specialization with a supported `function` fallback. Tree-sitter's integrated fixture test verifies the selected rightmost definition capture; a separate call-site assertion verifies `function` itself.
- `and` is listed as a current keyword by `SEMANTICS.md` and is reserved by both compiler and editor lexers, but Accepted DEC-0017 forbids treating it as a boolean operator alias and no mutual-recursion production exists. Highlighting its lexical class does not imply successful syntax.
- The lower-authority Zed plan still contains historical `zero` command examples. ZQ-3201 creates no command or extension manifest, so none of those stale spellings entered implementation.
- Syntax cannot prove whether every bare name is a value, function value, zero-payload constructor, namespace, or shadowed Prelude item. Those distinctions are explicitly deferred to resolver-backed semantic tokens rather than guessed in this query.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `77aab24ff8160e1535ea15b67d5302c1a4bb3fc8`:

- `npm run verify` with tree-sitter-cli 0.26.12 — all 41 grammar corpus cases, scanner/layout integrations, 18 Unicode cases, 29 precedence cases, 41 Pattern/Type cases, 10 static recovery cases, 9 incremental edits, 64 recovery mutations, 42 whole-program differential cases, 84 whole-corpus edits, 43 stable mappings, 3 highlight fixtures with 46 assertions, the 18-capture contract runner, and the local example pass.
- ZQ-3201 runner — exact 18-capture, 16-keyword, 6-built-in-type, and 3-fixture inventories pass; two query-test executions have identical normalized output and diagnostics; clean fixtures contain no `ERROR`/`MISSING`, while the emoji-prefix fixture contains bounded recovery evidence. The inline assertions follow [Tree-sitter's documented highlight-test format](https://tree-sitter.github.io/tree-sitter/3-syntax-highlighting.html#unit-testing).
- Generated parser idempotence — `npm run verify` regenerated the parser and left `grammar.json`, `node-types.json`, `parser.c`, scanner/header files, and Unicode identifier data unchanged.
- `cargo xtask governance check-all` — 43 documents, 26 gaps, 18 lifecycle records, 18 protocols, and 56 diagnostic codes pass.
- Schema validation, N-1 compatibility declarations, and 23 deterministic corrupt-input checks pass.
- Traceability remains 7 features, 42 conformance fixtures, 69 evidence records, and 7 explicitly deferred differential paths; support, CI-contract, Seed-reproduction, and implementation-status gates pass.
- All 27 execution-plan checksums match after the backlog transition; 957 local inline links across 92 active Markdown files resolve to repository targets.
- `cargo test --workspace --all-features --locked --offline` passes all workspace unit, integration, conformance, governance, and documentation tests, including 91 xtask tests.
- `cargo fmt --all -- --check`, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, release build, and `git diff --check` pass.

No remote CI result or live Zed visual smoke is claimed. The latter belongs to the grammar-only development extension milestone after the required query files exist.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** no Ling error code, severity, bilingual message, typed Fact, Repair, or original UTF-8 byte-span behavior changed. Tree-sitter queries emit no Ling diagnostics.
- **Schemas and protocols:** no public schema, protocol marker, CLI behavior, exit class, ABI, or dependency changed. Query captures remain an Experimental editor integration surface.
- **Language behavior:** unchanged. The query consumes CST nodes only and neither accepts source nor reaches AST, Typed Core, or evaluation.
- **Semantic IDs and canonical bytes:** unchanged; highlights, capture names, fixture annotations, and theme selection are excluded from semantic identity.
- **Determinism:** capture/keyword/type inventories and fixture names are exact and ordered; query execution is run twice with normalized output comparison; no theme, filesystem path, hash-map order, or timing value enters expected results.
- **Unicode:** Unicode remains 17.0.0. ASCII and Chinese names share the same structural rules, the decomposed identifier remains intentionally non-normalized in the editor fixture, and compiler-owned NFC/security checks remain authoritative.

## Intentionally deferred work

- ZQ-3202 owns bracket-pair matching, including string-quote/rainbow behavior; ZQ-3201 only assigns highlight punctuation.
- Indentation, outline, text objects, runnables, overrides, injections, and redactions remain ZQ-3203 onward.
- Semantic token taxonomy and resolver/type-driven refinement remain LSP-2401 onward.
- Live Zed file recognition and visual/theme smoke remain ZEXT-3301; no extension manifest or unpinned grammar reference is added here.
- All post-Seed keywords and syntax remain unavailable until accepted authority and their own grammar/query tasks exist.
