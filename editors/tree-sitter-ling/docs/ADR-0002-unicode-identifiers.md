# ADR-0002: Generated Unicode identifier ranges

> Engineering status: Accepted for TS-3104
> Date: 2026-08-20
> Scope: `tree-sitter-ling` implementation only; this ADR is not Ling language authority

## Context

Ling Seed fixes identifier character properties to Unicode 17.0.0. The TS-3102/TS-3103 grammar used Tree-sitter's `\p{XID_Start}` and `\p{XID_Continue}` properties, whose Unicode database was supplied by the pinned Tree-sitter CLI rather than by Ling's checksummed Unicode inputs. That shape was useful for early editor parsing but could not establish version parity.

Tree-sitter's grammar DSL supports Rust-regex character classes, ranges, and Unicode scalar escapes. It does not support lookaround. External scanners are available for non-regular tokens, but XID membership is a regular character-range problem and the existing scanner has the separate responsibility of layout and nested-comment state.

References:

- [Tree-sitter Grammar DSL](https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html)
- [Rust `regex` syntax](https://docs.rs/regex/latest/regex/#syntax)

## Decision

`tools/unicode-gen` parses `XID_Start` and `XID_Continue` directly from the committed, SHA-256-verified Unicode 17.0.0 `DerivedCoreProperties.txt`. Its default invocation emits both:

- `crates/ling-unicode/src/generated.rs` for compiler security data;
- `editors/tree-sitter-ling/src/unicode-identifiers.generated.js` for editor XID ranges.

The generated JavaScript contains only versioned range data and the pinned source checksum. The hand-written `unicode-identifiers.js` adapter merges `_` into the start set and renders explicit Rust-regex `\x{...}` ranges. `grammar.js` consumes that pattern and contains no host Unicode property escape.

All Seed keywords use Tree-sitter's global reserved-word set. Because `and` is lexically reserved but recursive binding groups are not accepted, a private `_reserved_and_error` rule keeps its terminal reachable for keyword extraction and requires the scanner's non-emitting `_error_sentinel`. This makes exact `and` finite error input without adding a successful language production; `a`, `an`, and `and_then` remain ordinary identifiers.

## Compiler authority boundary

Tree-sitter recognizes the XID token shape only. It does not normalize names, reject forbidden properties, compute Script Sets, evaluate Identifier_Status/Identifier_Type, or detect confusables. In particular, XID_Continue contains join controls and variation selectors that Ling rejects before semantic analysis. Accepting those characters into an editor CST preserves nearby structure and does not make the program valid.

The shared TSV corpus is consumed independently by a `ling-syntax` integration test and a Tree-sitter process test. The compiler side asserts token kind, NFC result, stable diagnostic code, complete original UTF-8 byte span, and bilingual JSON messages. This is the diagnostic payload boundary required by the future LSP adapter; no Tree-sitter query or scanner emits a competing diagnostic.

## Reproducibility and verification

- The generator validates sorted, non-overlapping scalar ranges, `XID_Start` inclusion in `XID_Continue`, and the Unicode underscore assumptions.
- The Node differential runner reparses the pinned UCD and requires byte-generated range equality before testing editor behavior.
- The existing exhaustive Rust conformance test compares every Unicode scalar with the same UCD.
- CI regenerates both Unicode artifacts and rejects either diff.
- Parser generation, corpus tests, all shared cases, and generated-source idempotence run offline after dependencies are locked.

## Rejected alternatives

- Keeping `\p{XID_*}` would retain an unversioned editor dependency and could silently change accepted characters when the CLI changes.
- Copying XID tables into `scanner.c` would duplicate generated data and mix a regular-token concern into the stateful layout scanner.
- Enforcing NFC or confusable/security rules in the grammar would duplicate compiler semantics and cannot emit the registered bilingual diagnostics.
- A regex-only negative lookaround for `and` is unavailable in Tree-sitter's supported Rust-regex subset; excluding the full spelling also permits prefix tokenization. Global keyword reservation prevents that split correctly.
