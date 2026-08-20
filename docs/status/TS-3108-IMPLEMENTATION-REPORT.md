# TS-3108 Grammar differential implementation report

> Status: **Done**
> Completed: 2026-08-21
> Final verified implementation commit: `c90dc6209ab90a7b7e4c8b0056c164a13821dff0`
> Verified baseline: `main@c90dc6209ab90a7b7e4c8b0056c164a13821dff0`

## Outcome

TS-3108 synchronizes the complete current compiler conformance corpus with the Tree-sitter editor parser without changing Ling validity or semantics. A single sorted manifest covers all 42 `tests/conformance/*/case.ling` sources exactly once: the compiler parser confirms 34 syntactically valid and 8 syntactically invalid programs; every valid program has a clean Tree-sitter CST, seven invalid programs produce finite recovery trees, and one invalid numeric spelling is an explicit tolerant-editor whitelist case.

The same permanent runner applies 84 fixed-seed whole-corpus deletion/insertion edits twice, bounds parser time and output, rejects crashes, and compares normalized summaries. Forty-two normalized whole-program CST hashes plus the canonical generated `node-types.json` hash make node-mapping changes reviewable.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3108: clean CSTs for legal compiler conformance programs, finite trees for illegal programs, random-edit crash resistance, and stable CST node mapping.
- [`IMPLEMENTATION.md`](../IMPLEMENTATION.md) M2: bounded parser recovery, conformance evidence, and deterministic generated artifacts.
- Accepted [`DEC-0004`](../decisions/0004-pipeline-syntax.md): pipeline syntax remains `|>` and must not be confused with a following match-case `|`.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): relative offside layout, line continuation, recovery bounds, and compiler-owned diagnostics.
- [`grammar-map.md`](../grammar-map.md) §§2, 7, and 9: Tree-sitter remains an editor CST, built-in recovery is not successful Ling syntax, and whole-program evidence cannot override the compiler.

The execution plan is engineering guidance rather than language authority. No Draft clause is promoted and no accepted language behavior changes.

## Implementation

- [`conformance-syntax.tsv`](../../editors/tree-sitter-ling/test/fixtures/conformance-syntax.tsv) is the one sorted cross-tool classification. It separates compiler syntax validity from later negative name/type/effect/entry/runtime expectations and makes the sole Tree-sitter tolerance explicit.
- [`conformance_syntax_differential.rs`](../../crates/ling-syntax/tests/conformance_syntax_differential.rs) discovers the compiler corpus, requires exact manifest set equality, validates safe unique paths and policy combinations, and reparses original UTF-8 bytes with `ling-syntax` to confirm the 34/8 split.
- [`run-conformance-differential.js`](../../editors/tree-sitter-ling/test/run-conformance-differential.js) independently discovers the same 42 sources. Clean policies forbid both nonzero status and `ERROR`/`MISSING`; error policies require bounded recovery evidence; the tolerance must remain a clean editor CST over compiler-invalid input.
- The runner enforces a 500,000-microsecond parser timeout, 10-second whole-program process timeout, 20-second mutation-batch timeout, and 500,000-byte stdout/stderr ceilings. It parses two deterministic edits per source twice and compares path/duration-independent summaries.
- [`conformance-cst-sha256.txt`](../../editors/tree-sitter-ling/test/fixtures/conformance-cst-sha256.txt) locks all 42 normalized CSTs and canonical generated node types. Snapshot identifiers are unique and ordered; changes fail until their CST impact is reviewed.
- [`scanner.c`](../../editors/tree-sitter-ling/src/scanner.c) and [`grammar.js`](../../editors/tree-sitter-ling/grammar.js) use distinct private `_line_leading_bar` and `_line_leading_pipeline` tokens. The scanner inspects `|` versus `|>` before the grammar chooses whether to close a case body or continue its pipeline, eliminating five valid-program zero-width errors without exposing punctuation or changing the named CST model.
- [`scanner_state.c`](../../editors/tree-sitter-ling/test/scanner_state.c) directly protects the textual distinction and token boundary. The existing layout corpus protects line-leading pipeline behavior.
- `npm test` now includes the whole-program differential runner, so the existing locked offline verification and CI path cannot omit TS-3108.
- Generated `grammar.json` and `parser.c` are committed; all six generated artifacts are byte-idempotent. The named `node-types.json` surface is unchanged by the new private external token.

The design follows KISS by using one manifest and one runner, DRY by sharing classification across compiler and editor tests, YAGNI by adding no future syntax or public API, and SRP by leaving validity and diagnostics in the compiler while the editor parser owns only finite CST structure.

## Specification gaps or conflicts

- No unresolved language-specification conflict was encountered. TS-3108 tests editor-parser correspondence against the authoritative compiler corpus and does not create semantics.
- `m2-invalid-number` contains `0b102`. Tree-sitter intentionally accepts its complete token shape, while `ling-syntax` rejects the base-2 digit with registered bilingual `L-LEX-0011`. This is the sole explicit tolerance, not a claim that the source is valid Ling.
- Negative conformance polarity is not syntax polarity: most negative fixtures parse successfully and fail during later compiler phases. The shared manifest records syntax separately so those programs cannot be misclassified.
- Tree-sitter node names and hashes remain Experimental editor-parser implementation surfaces, not public Ling protocols or Semantic ID inputs.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `c90dc6209ab90a7b7e4c8b0056c164a13821dff0`:

- `npm run verify` with tree-sitter-cli 0.26.12 — 41/41 grammar cases, scanner/layout integrations, 18 Unicode cases, 29 precedence cases, 41 Pattern/Type cases, 10 static recovery cases, 9 incremental edits, 64 recovery mutations, 42 whole-program differential cases, 84 whole-corpus edits, 43 stable mappings, and the package example pass.
- Generated parser idempotence — all six generated artifacts retain identical SHA-256 hashes after a second generation.
- Governance, schema validation/compatibility/corrupt-input, traceability, support, CI-contract, deterministic Seed reproduction, and implementation-status gates pass.
- `cargo test --workspace --all-features --locked --offline` passes all workspace unit, integration, conformance, governance, and documentation tests.
- Rust formatting, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, and release build pass offline with locked dependencies.
- All 27 execution-plan checksums match after the backlog transition; 940 local inline links across 91 active Markdown files resolve to repository targets; `git diff --check` passes.

## Compatibility, determinism, and Unicode impact

- **Diagnostics:** no error code, severity, Facts schema, bilingual template, or original-byte-span behavior changed. Tree-sitter still emits no Ling diagnostic.
- **Schemas and protocols:** no public schema, protocol marker, CLI contract, exit code, package dependency, or successful named grammar node changed. The external-token list and normalized CST hashes remain private Experimental editor-parser surfaces.
- **Language behavior:** unchanged. Compiler-invalid source remains invalid and cannot reach checked Typed Core, including the one explicitly tolerated editor token.
- **Semantic IDs and canonical bytes:** unchanged; Tree-sitter CST nodes and snapshot hashes remain excluded from semantic identity inputs.
- **Scanner compatibility:** one private external symbol is inserted after `_line_leading_bar`; serialized scanner state remains version 2 with the same byte layout. Generated parser artifacts are regenerated together.
- **Determinism:** manifest/snapshot order, mutation seed, mutation count, time/output bounds, JSON normalization, and generated artifacts are fixed. Absolute temporary paths and timing values are excluded from comparisons.
- **Unicode:** generated tables, normalization, security policy, and Unicode 17.0.0 remain unchanged. Mutations operate on Unicode scalar strings, and compiler parsing continues to consume original UTF-8 bytes.

## Intentionally deferred work

- Adding or removing compiler conformance sources must update the shared classification and reviewed CST hashes; TS-3108 does not predict future corpus contents.
- Zed highlight, bracket, indent, outline, text-object, runnable, injection, and redaction queries remain ZQ-3201 onward.
- Project, VM, incremental compiler, formatter, CLI, LSP, and broader IDE work remains subject to the execution plan's interface and accepted-RFC prerequisites.
- All post-Seed Author Source remains unavailable unless an Accepted RFC or decision adds it.
