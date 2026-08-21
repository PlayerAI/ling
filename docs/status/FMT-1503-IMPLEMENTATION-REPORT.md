# FMT-1503 Implementation Report: Core Syntax Formatting

## Outcome

FMT-1503 is complete as a conservative core-syntax formatter over the
compiler-owned Format IR. `ling-format::format_core` consumes `FormatDocument`
tokens, emits deterministic Author Source spacing and four-space layout, and
reparses the candidate with the existing compiler parser before returning it.
Invalid or incomplete input is returned byte-for-byte unchanged.

The slice does not implement comment attachment, broad recovery, a CLI/LSP
protocol, range edits, or Audit Source rendering.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans and source identity as
  compiler-owned data.
- Accepted `DEC-0006` requires four ASCII spaces for formatter output, keeps
  relative offside layout, and distinguishes delimiter-internal soft newlines.
- Accepted `DEC-0023` §1 requires formatting to consume the compiler CST and
  forbids regex/Tree-sitter authority; §2 preserves exact scalar spelling and
  bytes; §3 fixes four-space indentation and soft-newline behavior; §4 keeps
  comments and blank lines in order; §6 requires idempotence and compiler
  revalidation; §7 requires conservative invalid-source handling.
- FMT-1503 in `03-G1-V0.1-LIVING.md` §7 requires the core Seed syntax corpus:
  let/function, type, record, ADT, match, if, pipeline, module/import, and
  mutable forms.

## Implementation

- Added `crates/ling-format/src/author.rs` and exported `format_core` from
  `ling-format`.
- Significant-token spacing is deterministic for declarations, applications,
  delimiters, operators, type punctuation, records, variants, match arms,
  pipelines, and assignments. Existing parentheses and trailing commas are
  never invented or removed.
- Compiler `Indent`/`Dedent` tokens drive four-space block output. Delimiter
  `SoftNewline` tokens retain line breaks and original intra-delimiter spacing.
- Literal, identifier, Unicode, and comment token text is copied exactly;
  source BOM is retained and all emitted line endings are LF.
- Comment-only indentation and blank-line count remain in place. Rich comment
  attachment and incomplete-region policy remain owned by later tasks.
- Candidate output is reparsed through `ling-syntax::parse`. A rejected
  candidate publishes the original source snapshot rather than partial output.

## Tests and evidence

`crates/ling-format/src/author.rs` tests cover:

- core spacing and four-space layout for let/functions and if expressions;
- records, variants, match arms, and pipelines;
- module declarations, imports, mutable declarations, and place assignment;
- Unicode identifiers, BOM, CRLF, comments, blank lines, and literal spelling;
- invalid-source byte-preserving fallback; and
- idempotence of already formatted output.

Executed checks:

- `cargo test -p ling-format --all-targets --locked --offline`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo test --workspace --all-targets --locked --offline`;
- `cargo fmt --all`; and
- `git diff --check`.

## Compatibility impact

No language syntax, parser semantics, Typed Core, diagnostics, Semantic IDs,
Audit Source bytes, CLI/LSP fields, ABI, or Unicode 17.0.0 tables changed.
`format_core` is an opt-in library operation and introduces no wire protocol,
format-on-save behavior, or automatic source rewrite.

## Deferred work

FMT-1504 owns attachment of documentation, line-end, and nested block comments.
FMT-1505 owns incomplete/error-region recovery. FMT-1506 owns property and
semantic-equivalence evidence. FMT-1507 owns CLI/LSP integration, and FMT-1508
keeps Author Source separate from canonical Audit Source.

## Next target

FMT-1504, comment attachment, is the next execution-plan target and must retain
the exact comment/blank-line ordering established by Accepted `DEC-0023`.
