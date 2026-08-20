# TS-3107 Error recovery implementation report

> Status: **Done**
> Completed: 2026-08-21
> Final verified implementation commit: `1debda6d69796182d2b051bd5b5b03992008a1ca`
> Verified baseline: `main@1debda6d69796182d2b051bd5b5b03992008a1ca`

## Outcome

TS-3107 hardens the Tree-sitter editor parser for incomplete source without changing Ling validity or semantics. Unclosed strings, records, and tuples; missing `=`, `->`, and `with`; partial Chinese identifiers; incomplete pipelines; inconsistent indentation; and incomplete control-flow edits now terminate within explicit bounds while retaining the surrounding declaration canaries as named CST nodes.

The implementation uses only private grammar/scanner structure and Tree-sitter's built-in `ERROR`/`MISSING` evidence. It does not add a successful language production or allow Tree-sitter recovery to bypass the authoritative compiler pipeline.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3107: cover the ten required malformed/edit-state classes and retain surrounding definitions for highlighting and outline.
- [`IMPLEMENTATION.md`](../IMPLEMENTATION.md) M2: bounded parser recovery and tested recovery/nesting policy.
- Accepted [`DEC-0006`](../decisions/0006-offside-layout.md): relative offside layout, delimiter-local soft newlines, recovery bounds, and compiler-owned diagnostics.
- [`grammar-map.md`](../grammar-map.md) §2 and §7: Tree-sitter is an editor CST, built-in recovery nodes are not successful Ling AST nodes, and private recovery helpers cannot create language semantics.

The execution plan is engineering guidance rather than language authority. No Draft clause is promoted and no accepted language behavior changes.

## Implementation

- [`grammar.js`](../../editors/tree-sitter-ling/grammar.js) wraps root declarations in a private synchronization branch. Incomplete `let` and function bindings may retain exactly one following complete root declaration, after which a never-emitted scanner sentinel aliased to `=` forces a built-in missing-token marker.
- [`scanner.c`](../../editors/tree-sitter-ling/src/scanner.c) appends `_root_declaration_boundary` to the existing external-token order. A single-pass newline probe consumes only the preceding newline, recognizes exact column-zero `let`, `type`, `module`, or `import` starts, leaves the keyword to the normal lexer, and resets recovery-only indentation/delimiter state.
- Keyword synchronization requires a token separator and therefore does not split Unicode identifiers such as `type人`. Unicode XID classification remains outside the scanner.
- [`errors.txt`](../../editors/tree-sitter-ling/test/corpus/errors.txt) grows from 37 to 41 corpus cases with grouped recovery coverage.
- [`recovery-cases.json`](../../editors/tree-sitter-ling/test/fixtures/recovery-cases.json) defines ten required static malformed sources with `Before`/`After` declaration canaries.
- [`run-recovery-integration.js`](../../editors/tree-sitter-ling/test/run-recovery-integration.js) adds nine byte-addressed incremental edits and 64 deterministic mutations. It enforces a 500,000-microsecond parser timeout, 10-second process timeout, 200,000-byte output ceilings, nonzero malformed status, built-in recovery evidence, and retained canaries.
- [`scanner_state.c`](../../editors/tree-sitter-ling/test/scanner_state.c) directly protects synchronization reset, normal-final-newline fallback, and the Unicode-prefix boundary.
- Generated `grammar.json`, `node-types.json`, and `parser.c` are committed and byte-idempotent.

The design follows KISS by adding one synchronization token and one bounded recovery path, DRY by sharing the same fixture set across static and incremental assertions, YAGNI by adding no future syntax, and SRP by keeping validity and diagnostics in the compiler.

## Specification gaps or conflicts

- No unresolved language-specification conflict was encountered. TS-3107 defines editor recovery behavior only.
- Tree-sitter recovery can retain the next declaration as a named descendant of an incomplete binding. Future queries must match declaration kinds without assuming every recovery declaration is a direct `source_file` child.
- The private external symbol list and generated CST metadata are experimental editor-parser compatibility surfaces, not public Ling protocols.
- The compiler remains authoritative for incomplete-input diagnostics, original UTF-8 spans, Unicode security, and legal syntax. Tree-sitter emits no Ling diagnostic.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `1debda6d69796182d2b051bd5b5b03992008a1ca`:

- `npm run verify` with tree-sitter-cli 0.26.12 — 41/41 grammar cases, scanner/layout integrations, 18 Unicode cases, 29 precedence cases, 41 Pattern/Type cases, 10 static recovery cases, 9 incremental edits, 64 deterministic mutations, and the package example pass.
- Generated parser idempotence — all six generated artifacts retain identical SHA-256 hashes after a second generation.
- Governance, schema validation/compatibility/corrupt-input, traceability, support, CI-contract, deterministic Seed reproduction, and implementation-status gates pass.
- `cargo test --workspace --all-features --locked --offline` passes all workspace unit, integration, conformance, governance, and documentation tests.
- Rust formatting, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, and release build pass offline with locked dependencies.
- All 27 execution-plan checksums match after the backlog transition; 925 local inline links across 90 active Markdown files resolve to repository targets; `git diff --check` passes.

## Compatibility, determinism, and Unicode impact

- **Diagnostics:** no error code, severity, Facts schema, bilingual template, or original-byte-span behavior changed. Tree-sitter still emits no Ling diagnostic.
- **Schemas and protocols:** no public schema, protocol marker, CLI contract, exit code, or dependency changed. Generated Tree-sitter metadata now marks binding bodies optional during recovery and permits one named retained declaration child; this is an Experimental editor-CST compatibility change.
- **Language behavior:** unchanged. Malformed source remains compiler-invalid and cannot reach checked Typed Core.
- **Semantic IDs and canonical bytes:** unchanged; Tree-sitter CST/recovery nodes remain excluded from semantic identity inputs.
- **Scanner compatibility:** the new external symbol is appended, and the serialized scanner state remains version 2 with the same byte layout. Synchronization resets only private recovery state.
- **Determinism:** fixture order, mutation seed, time/output bounds, keyword set, and generated artifacts are fixed. No filesystem path, allocation detail, or hash-map order becomes observable.
- **Unicode:** generated tables, normalization, security policy, and Unicode 17.0.0 remain unchanged. A direct scanner test prevents ASCII keyword probes from splitting Unicode identifier continuations.

## Intentionally deferred work

- TS-3108 whole-program compiler/Tree-sitter differential is now the next Ready task. It owns legal-program `ERROR/MISSING` parity, finite invalid-program trees, randomized whole-corpus edits, and stable node mapping.
- Zed highlight, bracket, indent, outline, injection, and task queries remain ZQ-3201 onward.
- Compiler incomplete-input diagnostics and future LSP diagnostic adaptation remain compiler/LSP work; they are not duplicated in Tree-sitter.
- All post-Seed Author Source remains unavailable unless an Accepted RFC or decision adds it.
