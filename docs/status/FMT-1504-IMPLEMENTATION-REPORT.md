# FMT-1504 Implementation Report: Comment Attachment

## Outcome

FMT-1504 is complete as a compiler-CST-backed comment attachment slice. The
Author Source Format IR now exposes deterministic attachment metadata for line,
documentation, and block comments. Multiline/nested block comments are grouped
from the lexer segments that prove they belong to the same block; separate block
comments remain separate attachments. Attachments retain source order and point
to the nearest top-level CST declaration or to `Program` for a comment-only
source, so comments cannot be reassigned across definitions by the formatter.

`format_core` uses the attachment ranges as a preservation guard before
publishing a formatted candidate. The renderer still emits the original comment
token spelling and token order; no second parser or text-based syntax authority
was introduced.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans and compiler source
  identity as the attachment boundary.
- Accepted `DEC-0023` §1 requires the formatter to operate over the compiler CST
  rather than a regex/Tree-sitter authority; §2 preserves exact comment and
  documentation bytes; §4 keeps comments and blank lines attached to their
  neighboring syntactic region, in relative order, without cross-definition
  movement or merging; §6 requires safe publication; and §7 leaves broad
  incomplete-source recovery to FMT-1505.
- FMT-1504 in `03-G1-V0.1-LIVING.md` §7 covers documentation comments, line-end
  comments, nested block comments, and Chinese comments without definition
  drift.

## Implementation

- Added `CommentKind`, `CommentPlacement`, and `CommentAttachment` to
  `crates/ling-format/src/comments.rs` and exported them from `ling-format`.
- Added `FormatDocument::comment_attachments()`; `build_format_ir` constructs
  the metadata from `FormatToken` and `FormatNode` only.
- Documentation comments remain distinct from ordinary line comments.
- The attachment scan stops same-line trailing association at compiler
  `Newline`/`SoftNewline` tokens and ignores whitespace/comments/layout when
  locating neighboring significant tokens.
- A comment already inside a top-level CST declaration remains owned by that
  declaration. Comments outside a declaration attach to the next declaration,
  or the previous declaration for a same-line trailing comment. A comment-only
  source is attached to `Program` as standalone metadata.
- Multiline block comments are grouped by balanced `/*`/`*/` delimiters in the
  compiler-emitted block-comment segments. This groups nested segments without
  merging adjacent independent block comments.
- `format_core` verifies that every attached comment segment is present in the
  candidate in original order and exact spelling before compiler revalidation.

## Tests and evidence

`crates/ling-format/src/comments.rs` tests cover:

- documentation, line-end, and leading Chinese comments attached to the
  correct `LetDeclaration`;
- nested multiline block comments grouped as one attachment without crossing
  into the following declaration; and
- comment-only files retaining standalone attachment order and `Program`
  ownership.

Executed checks:

- `cargo fmt --all`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo test -p ling-format --all-targets --locked --offline`; and
- `git diff --check`.

## Compatibility impact

No language grammar, parser semantics, Typed Core, diagnostics, Semantic IDs,
Audit Source bytes, CLI/LSP fields, ABI, or Unicode 17.0.0 tables changed.
`CommentAttachment` is an in-process formatter model; it introduces no wire
protocol or automatic format-on-save behavior. Original token spans and exact
UTF-8 comment spelling remain unchanged, and attachment construction is stable
for a fixed compiler token/CST projection.

## Deferred work

FMT-1505 owns conservative formatting of incomplete/error regions. FMT-1506
owns property and semantic-equivalence evidence. FMT-1507 owns CLI/LSP
integration, and FMT-1508 owns the separation proof between Author Source
formatting and canonical Audit Source rendering.

## Next target

FMT-1505, incomplete-source recovery, is the next execution-plan target. It must
preserve this attachment model and publish no partial rewrite for ambiguous
error regions.
