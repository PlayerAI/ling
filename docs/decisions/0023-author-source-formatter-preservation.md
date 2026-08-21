# DEC-0023: Author Source formatter preservation boundary

> 状态：Accepted
> 提出日期：2026-08-21
> 决定日期：2026-08-21
> Owner role：formatter-design
> 相关 RFC/缺口：`DEC-0002`, `DEC-0006`, `DEC-0015`, `GAP-FORMATTER-AUTHOR-SOURCE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

The roadmap requires an Author Source formatter, while `DEC-0015` defines a
separate canonical Audit Source. The formatter must make safe, repeatable
textual changes without becoming a second parser, changing checked semantics,
or erasing author-controlled comments, Unicode spelling, and incomplete text.

## Decision

1. Author Source formatting is a presentation transformation over the
   authoritative compiler CST and validated source spans. It is not a parser,
   resolver, type checker, Semantic Graph writer, Audit Source renderer, or
   language-localization pass. Format IR may be introduced by FMT-1502 only
   after consuming the compiler CST; regex and Tree-sitter are not semantic
   authorities.
2. Formatting preserves the exact scalar spelling and byte content of
   identifiers, string/character literals, comments, documentation text, and
   unknown/error regions. It may normalize line endings to LF and emit exactly
   one final LF. It must not NFC-normalize, translate, or replace localized
   names or user text merely because an equivalent spelling exists.
3. Indentation follows `DEC-0006` and `SEMANTICS`: semantic indentation is
   ASCII spaces, and formatter-produced blocks use four spaces per level.
   Delimiter-internal newlines remain soft newlines. A tab or inconsistent
   dedent in an invalid source is not silently repaired as if it were valid
   syntax; the incomplete/error policy below applies.
4. Blank lines and comments remain attached to their original neighboring
   syntactic region and retain relative order. The formatter may normalize
   surrounding indentation and line endings but may not invent, delete,
   duplicate, move across definitions, or merge comments. Documentation
   comments remain distinct metadata and are not converted into Audit fields.
5. Parentheses and delimiters are changed only when the authoritative CST
   proves the rewrite is syntax-preserving under the current precedence and
   layout rules. Necessary parentheses remain. Existing optional trailing
   commas are preserved; adding or removing them is deferred until a later
   decision supplies a style rule and migration evidence.
6. For a complete valid source file, formatting is required to be idempotent:
   `format(format(source)) == format(source)`. The implementation must parse
   the formatted bytes through the existing compiler pipeline and compare the
   existing checked/public equivalence contracts before publishing an edit.
   A formatter failure publishes no partial output.
7. For incomplete or invalid source, the formatter may rewrite only complete
   regions whose CST spans and attachments are unambiguous. It must otherwise
   return the original bytes unchanged together with the existing diagnostics;
   it must not fabricate missing tokens, reinterpret error nodes, or destroy
   text needed for recovery. FMT-1505 owns the later exhaustive recovery
   policy.
8. Author Source and Audit Source remain disjoint. Author formatting never
   changes canonical Audit bytes, Semantic IDs, source spans, or evaluator
   inputs. `DEC-0015` continues to own Audit Source grammar, canonical order,
   and round-trip behavior.
9. This decision fixes preservation and safety boundaries only. It adds no
   formatter CLI/LSP command, range-format protocol, JSON schema, diagnostic
   allocation, localized-keyword view, or public stability claim.

## Conformance plan

- Run positive fixtures covering declarations, expressions, modules/imports,
  records, variants, matches, mutable places, pipelines, nested delimiters,
  CRLF, BOM handling, Unicode identifiers, and Chinese comments.
- Verify idempotence and compare parsed CST, resolved references, checked
  Typed Core, effects, Semantic IDs, and existing diagnostic projections for
  every valid fixture before and after formatting.
- Verify comment/documentation attachment, blank-line order, optional comma
  preservation, necessary-parenthesis retention, four-space layout, LF output,
  and exact scalar spelling of localized names and string/comment content.
- Exercise invalid UTF-8, tabs, inconsistent dedents, unterminated strings or
  comments, and incomplete blocks; require bounded diagnostics and either
  unchanged bytes or edits limited to complete, independently validated spans.
- Compare the canonical Audit Source produced before and after formatting and
  require byte equality whenever the checked program is equivalent. Run the
  same corpus through independent process invocations to check deterministic
  output and attachment order.

## Compatibility impact

- This is Accepted implementation authority for a future Author Source
  formatter. It does not change current compiler, CLI, LSP, Audit, diagnostic,
  schema, Semantic ID, ABI, or Unicode 17.0.0 behavior.
- A future formatter may intentionally rewrite whitespace and LF boundaries
  only within this decision; callers must opt in before replacing Author
  Source. No automatic format-on-save or range edit is implied.
- Existing `DEC-0006` four-space output and `DEC-0015` Audit Source remain
  compatible; no migration of existing source or audit artifacts is required
  by this decision.

## Unresolved alternatives

- A canonical policy for adding/removing trailing commas, aggressive redundant
  parenthesis removal, and localized keyword views requires separate evidence.
- Range formatting, editor edit transactions, format-on-save, and public JSON
  reports are deferred to FMT-1507 and the LSP transaction authority.
- Recovery heuristics for broad error regions remain deferred to FMT-1505;
  they cannot relax the no-second-parser or no-text-loss rules above.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
