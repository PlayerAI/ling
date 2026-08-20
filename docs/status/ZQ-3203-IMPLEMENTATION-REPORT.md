# ZQ-3203 indentation-query implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `a4377450d26374098d95a9bb38520d3e3552dfd7`
> Verified baseline: `main@a4377450d26374098d95a9bb38520d3e3552dfd7`

## Outcome

ZQ-3203 adds the shared [`indents.scm`](../../editors/tree-sitter-ling/queries/indents.scm) editor query over 15 current CST node types. It defines relative ranges for module, type, function, value-binding, match-arm, conditional, record, tuple, list, and pipeline-continuation structure without choosing a whitespace width or changing Ling parsing.

Four clean/recovery fixtures lock 38 `@indent`, 14 `@end`, and 4 `@start` captures at exact source coordinates. They cover two- and four-space relative layouts, Chinese and decomposed identifiers, every requested delimiter form, aligned match cases, both same-line and next-line pipeline operands, and a valid list range retained after an emoji-prefix error.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) ZQ-3203: conservative CST-based support for declaration/control-flow bodies, record/tuple/list contents, match arms, and pipeline continuation; editor indentation must not compete with formatting.
- Accepted [`DEC-0004`](../decisions/0004-pipeline-syntax.md): a line-leading `|>` aligns with the pipeline start, while a right operand continued to the next nonempty line must be further indented.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): relative indentation, four-space formatter output, delimiter-local soft newlines, match-case alignment, indented arm bodies, and compiler-owned layout validity and diagnostics.
- Accepted [`DEC-0005`](../decisions/0005-seed-literals-and-delimiters.md): the current record, tuple, list, and grouping delimiter surface.
- [`SEMANTICS.md`](../SEMANTICS.md) §3.9 and [`grammar-map.md`](../grammar-map.md) §§2–7: offside rules, editor-only CST authority, current named nodes, and recovery boundaries.
- Zed's [language-extension guide](https://zed.dev/docs/extensions/languages#auto-indentation) defines `@indent`/`@end`. The current engine at upstream commit [`cb1352a`](https://github.com/zed-industries/zed/commit/cb1352a29d6c99226d942328c14fae9d3c5e0ded) also registers optional `@start`/`@end` captures in [`grammar.rs`](https://github.com/zed-industries/zed/blob/cb1352a29d6c99226d942328c14fae9d3c5e0ded/crates/language_core/src/grammar.rs#L639-L674) and applies `@start` at the captured node end and `@end` at its start in [`buffer.rs`](https://github.com/zed-industries/zed/blob/cb1352a29d6c99226d942328c14fae9d3c5e0ded/crates/language/src/buffer.rs#L3688-L3707).

The query is an editor aid only. It does not validate layout, emit a Ling diagnostic, rewrite source, or make a recovered Tree-sitter CST valid Ling.

## Implementation

- Offside ranges capture `module_declaration`, `type_declaration`, `function_definition`, and `let_declaration`. Capturing the containing declaration includes its first body line; capturing `block` itself would begin at that line and could not supply its initial extra level.
- `match_case` supplies the arm-body level. `match_expression` is deliberately absent: DEC-0006 requires each case marker to stay aligned with `match`.
- Two `if_expression` patterns split at the same `else` token: the consequence uses `@end`, and the alternative uses `@start`. This keeps `else` aligned with `if` while indenting both bodies independently.
- `record_type`, `record_pattern`, `record_expression`, `record_update_expression`, `tuple_type`, `tuple_pattern`, `tuple_expression`, and `list_expression` use the conventional `@indent` plus closing-token `@end` shape.
- `pipeline_expression` starts its range at `|>` and ends at the immediate right operand. A line-leading operator therefore remains aligned, while only an operand placed on a later line gains a level. Nested left-associative pipeline matches remain separate and bounded.
- [`run-indent-tests.js`](../../editors/tree-sitter-ling/test/run-indent-tests.js) fixes the query capture/node/fixture inventories, exact range-start multiset, clean/recovery policy, process limits, Unicode canaries, match/pipeline alignment, and deterministic normalized output across two query-test and two per-fixture capture executions.
- [`blocks.ling`](../../editors/tree-sitter-ling/test/fixtures/indents/blocks.ling) covers module/type/function/value bodies, match arms, split conditionals, Chinese names, a decomposed combining identifier, and relative two-space indentation. [`delimiters.ling`](../../editors/tree-sitter-ling/test/fixtures/indents/delimiters.ling) covers all eight requested delimiter CST forms. [`pipeline.ling`](../../editors/tree-sitter-ling/test/fixtures/indents/pipeline.ling) covers aligned operators and next-line/same-line operands. [`recovery.ling`](../../editors/tree-sitter-ling/test/fixtures/indents/recovery.ling) retains an exact canary range after emoji recovery.
- The package's default `test`/`verify` path now includes `test:indents`; root/editor documentation and [`KNOWN-DIFFERENCES.md`](../../editors/tree-sitter-ling/KNOWN-DIFFERENCES.md) expose the implemented boundary.

The implementation applies KISS through direct node/range patterns, SRP by leaving validity and formatting outside the query, DRY by sharing each fixture between parse and range checks, and YAGNI by omitting unavailable syntax and speculative indentation heuristics.

## Specification gaps or conflicts

- No unresolved Seed semantic or public-protocol gap blocks ZQ-3203. The relevant syntax and layout rules are already Accepted, and the query adds no language behavior.
- The lower-authority execution plan names the stale command `zero fmt`. Repository authority fixes the CLI name as `ling`, and no public `ling fmt` command is implemented yet. ZQ-3203 therefore states only that a formatter remains authoritative when implemented; it neither invokes nor advertises a nonexistent formatter.
- The plan's broad “match body” wording cannot override DEC-0006's exact alignment rule. A blanket `(match_expression) @indent` would push case markers beneath `match`, so the implementation represents the actual extra level with `match_case` ranges instead.
- The current Zed guide's abbreviated capture table omits `@start`; the pinned current Zed source confirms the optional capture and its exact range semantics. No undocumented capture is inferred from third-party editor behavior.
- ZQ-3204 remains blocked as written because its required v0.1 outline includes `trait` and `impl`, for which Seed has neither Accepted Author Source syntax nor CST nodes. Implementing only an easier Seed subset would not complete that target.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `a4377450d26374098d95a9bb38520d3e3552dfd7`:

- `npm run verify` with tree-sitter-cli 0.26.12 — all 41 grammar cases, scanner/layout integrations, 18 Unicode cases, 29 precedence cases, 41 Pattern/Type cases, 10 static recovery cases, 9 incremental edits, 64 deterministic recovery mutations, 42 whole-program differential cases, 84 whole-corpus edits, 43 stable mappings, 3 highlight fixtures/18 captures, 3 bracket fixtures/4 pairs, 4 indentation fixtures/15 node types, and local examples pass.
- ZQ-3203 runner — the exact three-capture, 15-node, four-fixture, 38-`@indent`/14-`@end`/4-`@start` range inventory passes; clean fixtures contain no `ERROR`/`MISSING`; emoji recovery remains finite; all processes stay within 10 seconds and 300,000 output bytes; repeated normalized outputs and diagnostics are identical.
- Generated Tree-sitter idempotence — `npm run verify` regenerated the parser and left grammar JSON, node types, parser/scanner sources, headers, and Unicode identifier data unchanged.
- Current Zed interface audit — public upstream commit `cb1352a29d6c99226d942328c14fae9d3c5e0ded` registers and consumes all three captures with the expected boundaries.
- `cargo xtask governance check-all` — 43 documents, 26 gaps, 18 lifecycle records, 18 protocols, and 56 diagnostic codes pass.
- Schema validation covers 3 schemas, 4 valid fixtures, 6 invalid fixtures, and 1 canonical-byte fixture; N-1 declarations pass; all 23 deterministic corrupt-input checks pass.
- Traceability remains 7 features, 42 conformance fixtures, 69 evidence records, and 7 deferred differential paths; support, CI-contract, deterministic Seed reproduction, and implementation-status gates pass.
- `cargo test --workspace --all-features --locked --offline` passes all workspace unit, integration, conformance, governance, and documentation tests, including 91 xtask tests.
- `cargo fmt --all -- --check`, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, release build, execution-plan checksums, local Markdown links, and `git diff --check` pass.

No remote CI result or live Zed visual/keystroke smoke is claimed. A development-extension smoke belongs to ZEXT-3301 after the required query/configuration boundary is unblocked.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** no Ling error code, severity, bilingual message, typed Fact, Repair, or original UTF-8 byte-span behavior changed. Tree-sitter queries emit no Ling diagnostic.
- **Schemas and protocols:** no public schema, protocol marker, CLI contract, exit class, ABI, dependency, grammar node, or generated parser artifact changed. The three capture names remain an Experimental editor surface.
- **Language behavior:** unchanged. The compiler remains authoritative for tabs, relative-indent validity, inconsistent dedents, depth limits, and whether source reaches checked Typed Core.
- **Semantic IDs and canonical bytes:** unchanged; query captures, editor indentation, whitespace style, and recovery nodes do not enter semantic identity.
- **Determinism:** fixture names, node/capture inventories, exact coordinates, process/time/output bounds, and query ordering are fixed. Query output is normalized and compared across repeated executions; no host path, timing, hash-map order, or theme value enters expected evidence.
- **Unicode:** Unicode remains 17.0.0. Chinese and decomposed XID-shaped identifiers use the same structural ranges, and the emoji-prefix case proves bounded editor recovery without weakening compiler NFC/security policy.

## Intentionally deferred work

- `outline.scm`, text objects, runnables, overrides, injections, redactions, and live Zed configuration remain their own declared tasks and prerequisites.
- `ling fmt`, on-type formatting, and the formatter/LSP edit protocol remain formatter/IDE work; ZQ-3203 does not create a formatting API.
- Trait, impl, task, actor, node, kernel, contract, Effect-row, and Borrow syntax remain unavailable until Accepted authority expands Seed and the compiler/grammar implement it.
- Live Zed indentation behavior, file recognition, extension installation, and pinned grammar integration remain ZEXT-3301 evidence rather than inferred success from Tree-sitter CLI output.
