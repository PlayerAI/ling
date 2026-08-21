# FMT-1501 Implementation Report: Author Source Formatter Preservation

## Outcome

FMT-1501 is complete as an Accepted preservation decision. `DEC-0023` fixes
the safety boundary required before Format IR work: Author Source formatting is
an opt-in presentation transformation over the compiler CST, while Audit
Source remains a separate canonical projection governed by `DEC-0015`.

No formatter implementation or public command is claimed by this milestone.

## Normative traceability

- `SEMANTICS` §3.9 requires offside-aware formatting with four ASCII spaces and
  forbids tabs as semantic indentation.
- Accepted `DEC-0002` retains original UTF-8 byte spans and position units.
- Accepted `DEC-0006` owns layout, delimiter, and continuation boundaries.
- Accepted `DEC-0015` owns Audit Source grammar, canonical bytes, and its
  isolated reader/writer; Author formatting must not replace it.
- Accepted `DEC-0023` closes the preservation, idempotence, incomplete-source,
  comment, Unicode spelling, delimiter, and no-second-parser policy question.

## Accepted boundary

- Valid Author Source formatting must be idempotent, use the authoritative
  compiler CST, preserve checked/public equivalence, and publish no partial
  result on failure.
- Identifier, literal, comment, documentation, and unknown/error-region scalar
  spelling and bytes are preserved. LF normalization and one final LF are
  allowed; localized text is not translated or NFC-rewritten.
- Four-space semantic indentation follows the existing layout decision.
  Comments and blank lines retain attachment and relative order. Optional
  trailing commas are preserved, while new comma style and aggressive
  parenthesis removal remain deferred.
- Incomplete or invalid source receives only complete-span edits whose CST
  attachment is unambiguous; otherwise bytes remain unchanged with existing
  diagnostics. The formatter cannot fabricate tokens or become a recovery
  parser.

## Evidence plan and deferred implementation

`DEC-0023` registers positive/negative fixtures for valid syntax, comments,
documentation, blank lines, CRLF, BOM, Unicode names, delimiters, parentheses,
mutable places, tabs, inconsistent dedents, unterminated text/comments, and
incomplete blocks. Later FMT-1502 through FMT-1506 must execute these fixtures,
prove idempotence and checked equivalence, and compare Audit Source bytes before
and after formatting. FMT-1507 separately owns CLI/LSP/range/format-on-save
interfaces.

## Compatibility impact

The decision changes no current source syntax, compiler behavior, diagnostics,
schemas, Semantic IDs, Audit bytes, CLI/LSP fields, ABI, or Unicode 17.0.0
tables. It creates no migration and no public formatter protocol; it only
authorizes future opt-in implementation within the stated boundary.

## Validation

- `cargo xtask governance check-all` passed with 56 indexed documents, 26 gaps,
  31 lifecycle records, 21 protocols, and 82 diagnostic codes.
- Generated authority, lifecycle, and gap reports are current and deterministic.
- `cargo fmt --all -- --check` and `git diff --check` are required before the
  milestone commit.

## Next target

FMT-1502 (Format IR) is the next implementation row and must consume compiler
CST data under Accepted `DEC-0023`; it must not introduce a second parser.
