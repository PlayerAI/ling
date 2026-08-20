# TS-3106 Pattern and Type implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `7948a17a7848c32078b3893b6c9182ab7c41096b`
> Verified baseline: `main@7948a17a7848c32078b3893b6c9182ab7c41096b`

## Outcome

TS-3106 completes focused Pattern and Type coverage for the v0.0.1 Seed compiler/editor boundary. The compiler and Tree-sitter now share a 41-case validity corpus covering binding, wildcard, Unit, grouping, tuple, literal, qualified and nested constructor, nonempty record, and guarded patterns; it also covers generic declarations and variable, qualified, applied, product, tuple, parenthesized, and right-associative function types.

Tree-sitter now exposes a named `parenthesized_pattern`, preserves a qualified constructor as one coherent `qualified_name`, and keeps every Pattern/Type form shallow and query-oriented. It still does not decide whether a bare name is a binding or zero-payload constructor. The authoritative compiler now rejects singleton/trailing-separator tuple patterns and empty record patterns consistently with Accepted DEC-0005, while grouped nested constructor patterns lower without being misrepresented as tuple payloads.

The shared corpus explicitly rejects incomplete type forms and post-Seed effect-row or Borrow Author Source. No placeholder node makes an unavailable feature appear implemented.

## Normative clauses covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3106: ADT, accepted record, tuple, wildcard, literal, and guarded patterns; type application, generics, and function types; no fake nodes for unavailable syntax.
- Accepted [`DEC-0005`](../decisions/0005-seed-literals-and-delimiters.md): nonempty records, record/list separators, Unit/group/tuple delimiters, no singleton tuple, and Seed literal forms.
- Accepted [`DEC-0013`](../decisions/0013-main-and-runtime-failures.md): the Unit entry pattern.
- Accepted [`DEC-0014`](../decisions/0014-seed-prelude-option-result.md): ordinary resolved `Option`/`Result` constructors and constructor payload patterns.
- [`SEMANTICS.md`](../SEMANTICS.md) §6 and §11: Seed type representation, ordered pattern matching, bindings, and guards.
- [`RFC-0001.md`](../RFC-0001.md) §8: the Draft Seed Pattern/Type grammar baseline.

Tree-sitter remains an editor CST implementation, not a language authority. No Draft clause is promoted by this work.

## Implementation

- [`ling-syntax`](../../crates/ling-syntax/src/parser.rs) factors constructor-sequence parsing into one helper used by match, grouped/tuple, and record-field patterns. It rejects singleton/trailing tuple separators and empty records with bounded registered diagnostics.
- [`ling-hir`](../../crates/ling-hir/src/lib.rs) has explicit evidence that `Some (Ok item)` lowers as two one-argument constructors, not as a synthetic tuple.
- [`grammar.js`](../../editors/tree-sitter-ling/grammar.js) adds `parenthesized_pattern` and places the qualified-constructor alias at the field boundary so the CST exposes one coherent qualified name.
- [`pattern-types.tsv`](../../editors/tree-sitter-ling/test/fixtures/pattern-types.tsv) is the single 41-case compiler/Tree-sitter validity corpus. Independent Rust and Node runners require both parsers to agree and bound generated CST size.
- [`declarations.txt`](../../editors/tree-sitter-ling/test/corpus/declarations.txt), [`patterns.txt`](../../editors/tree-sitter-ling/test/corpus/patterns.txt), and [`errors.txt`](../../editors/tree-sitter-ling/test/corpus/errors.txt) add exact CST evidence for nested generics, type precedence, qualified/grouped/nested patterns, all Seed literal patterns, rejected delimiters, and unavailable syntax.
- Four conformance fixtures prove grouped constructor execution and stable failures for singleton tuple, trailing tuple separator, and empty record patterns.
- [`grammar-map.md`](../grammar-map.md), the Tree-sitter README/known-differences record, and the generated traceability matrix now describe the implemented boundary and its remaining owners.

The implementation follows KISS by extending the existing shallow grammar, DRY by sharing one validity corpus and one compiler pattern-term helper, YAGNI by rejecting unavailable syntax, and SRP by leaving semantic name classification and type checking in compiler layers.

## Specification gaps or conflicts

- No unresolved conflict was found in the required TS-3106 surface. Accepted decisions control delimiters and Prelude constructors; Draft documents provide only the surrounding Seed baseline.
- DEC-0005 enumerates tuple forms as `(a, b)`, `()`, and grouped `(a)`, while explicitly granting trailing separators only to semicolon-separated record/list members. The closed Seed corpus therefore rejects singleton and trailing tuple-pattern separators rather than preserving the compiler's previous permissive parse.
- A bare identifier remains deliberately syntax-neutral. Tree-sitter cannot determine whether resolution will classify it as a binding or zero-payload constructor.
- Effect rows exist in checked internal types, but no Accepted Author Source syntax exposes them. Borrow is explicitly outside Seed. Both forms remain finite error input without successful feature nodes.
- The qualified-constructor CST previously exposed repeated `constructor` fields. Moving the alias outward corrects editor structure only; it changes no Ling semantic representation.

## Tests and verification

Executed locally on 2026-08-20 against implementation commit `7948a17a7848c32078b3893b6c9182ab7c41096b`:

- Shared Pattern/Type validity differential — all 41 cases agree between `ling-syntax` and the generated Tree-sitter parser.
- Compiler diagnostics — singleton tuple and empty record failures use `L-SYNTAX-0010`, include both public languages, and point at exact original UTF-8 delimiter byte spans following multibyte names.
- HIR lowering — grouped nested constructor payloads retain one argument per constructor without forged tuple arity.
- Conformance — all 42 registered fixtures pass, including exact `7\n` execution and the three TS-3106 syntax failures.
- `npm run verify` with tree-sitter-cli 0.26.12 — 37/37 grammar cases, scanner/layout integrations, 18 Unicode differential cases, 29 precedence differential cases, 41 Pattern/Type differential cases, and the package example pass.
- Generated parser idempotence — grammar JSON, node types, C parser, and three Tree-sitter headers retain identical SHA-256 hashes after regeneration.
- `cargo test --workspace --all-features --locked --offline` — all workspace unit, integration, conformance, governance, and documentation tests pass.
- Rust formatting, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, and release build pass offline with locked dependencies.
- Governance, schema, traceability, support, CI-contract, status, and deterministic Seed-reproduction gates pass.
- All 27 execution-plan checksums match after the backlog transition; 912 local inline links across 89 active Markdown files resolve to repository targets.

## Compatibility, determinism, and Unicode impact

- **Diagnostics:** no error code, severity, Facts schema, or bilingual template changed. New negative evidence reuses registered `L-SYNTAX-0010` and checks original UTF-8 byte spans.
- **Schemas and protocols:** no public schema, protocol marker, CLI command shape, exit-code contract, or dependency changed.
- **Language behavior:** the compiler closes three accepted delimiter bugs: singleton tuple patterns, trailing tuple-pattern separators, and empty record patterns are no longer accepted. Grouping nested constructor patterns now follows their existing HIR meaning. All other checked Pattern/Type semantics are unchanged.
- **Semantic IDs and canonical bytes:** no schema or tag changed. Invalid programs no longer reach checked Semantic output; IDs for previously valid Seed programs are unchanged.
- **Source positions:** compiler spans remain offsets in original UTF-8 bytes; multibyte negative tests exercise the invariant.
- **Determinism:** shared fixture order is fixed, both parsers are checked independently, CST size is bounded, and all six generated artifacts are byte-idempotent.
- **Unicode:** Unicode tables, NFC/security behavior, and the pinned Unicode 17.0.0 version are unchanged.

## Intentionally deferred work

- Systematic malformed/incomplete edit recovery remains TS-3107, now the next Ready task.
- Whole-program compiler/Tree-sitter corpus differential and randomized edit coverage remain TS-3108.
- Binding versus zero-payload-constructor classification remains compiler name-resolution work, not Tree-sitter syntax.
- Effect-row, Borrow, Trait, Resource, Task, Actor, Node, Kernel, Contract, and other post-Seed Author Source remain unavailable unless an Accepted RFC or decision adds them.
- Interpreter/VM differential remains VM-1209 because no second checked execution engine exists.
