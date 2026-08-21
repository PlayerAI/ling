# FMT-1502 Implementation Report: Compiler-CST Format IR

## Outcome

FMT-1502 is complete as a bounded, in-process Format IR slice. The new
`ling-format` projection consumes `ling-syntax`'s authoritative compiler CST
and publishes an immutable source snapshot, a recursively projected document
tree, and the compiler token stream with exact original spelling. It does not
render text, attach comments, expose a formatter command, or introduce a
second parser.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans as the source-position
  authority.
- Accepted `DEC-0006` keeps compiler layout tokens and delimiter-internal
  soft-newline distinctions intact.
- Accepted `DEC-0023` §1 requires Format IR to consume the compiler CST rather
  than regex or Tree-sitter; §2 requires exact scalar/byte preservation; §7
  requires invalid/incomplete input not to be fabricated into new syntax.
- `docs/ling_execution_plan/03-G1-V0.1-LIVING.md` §7 and FMT-1502 require the
  document-tree slice before formatter style, recovery, or protocol work.

## Implementation

- Added `build_format_ir(&SourceFile, &ParsedSource)` in
  `crates/ling-format/src/format_ir.rs`.
- `FormatDocument` owns the exact original UTF-8 snapshot, the BOM-free
  LF-normalized lexical snapshot, validity state, and projected compiler root.
- `FormatNode` preserves every compiler `NodeKind`, CST token range, original
  byte span, and child order. Empty recovery ranges have no fabricated span.
- `FormatToken` preserves `TokenKind`, normalized lexical span, original byte
  span, and exact original token spelling. Trivia and layout remain visible to
  later attachment/style stages.
- The builder validates source identity, original/lexical span bounds, and CST
  token ranges before publishing the document. Invalid parses are retained as
  lossless IR with `is_valid() == false`.
- `FORMAT_IR_SCHEMA` names the in-process projection only; no public wire
  protocol or CLI/LSP behavior is claimed.

## Tests and evidence

The `ling-format` unit tests cover:

- valid CST projection with BOM, CRLF, Unicode text, and Chinese comments;
- exact original token spelling and both span domains;
- invalid/incomplete source retention without fabricated tokens;
- source-identity mismatch rejection; and
- deterministic repeated projection.

Executed checks:

- `cargo test -p ling-format --all-targets --locked --offline`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo fmt --all`; and
- `git diff --check`.

## Compatibility impact

No language semantics, Typed Core, diagnostics, Semantic IDs, Audit Source
bytes, CLI/LSP fields, ABI, or Unicode 17.0.0 behavior changed. The only
dependency change adds existing workspace crates `ling-source` and
`ling-syntax` to `ling-format`; `Cargo.lock` was updated offline.

## Deferred work

FMT-1503 owns formatting style for core syntax. FMT-1504 owns comment and
blank-line attachment, FMT-1505 owns broad invalid-source recovery, FMT-1506
owns property/conformance evidence, and FMT-1507 owns CLI/LSP transactions.
This slice intentionally does not render or rewrite source.

## Next target

FMT-1503, core syntax formatting, may consume this IR only after its style
policy and preservation tests are kept within Accepted `DEC-0023`.
