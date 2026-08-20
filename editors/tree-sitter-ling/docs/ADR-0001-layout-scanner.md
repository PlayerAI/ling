# ADR-0001: Stateful layout scanner

> Engineering status: Accepted for TS-3103; amended for TS-3107 and TS-3108
> Date: 2026-08-21
> Scope: `tree-sitter-ling` implementation only; this ADR is not Ling language authority

## Context

Accepted DEC-0006 defines relative offside indentation, dedents, delimiter-local soft newlines, comment/blank-line handling, and EOF closure. A regular Tree-sitter token cannot compare a new line's indentation with an unbounded stack or restore that stack during incremental reparsing. The TS-3102 newline-plus-space token consequently cannot distinguish siblings, nested blocks, and dedents.

Nested block comments are also non-regular and affect whether a logical line is blank for layout purposes. Keeping the non-nested TS-3102 regex would leave the scanner unable to classify accepted comment-only input consistently.

## Decision

Use one private C external scanner with these external symbols, in stable array order:

1. `_newline`;
2. `_indent`;
3. `_dedent`;
4. `_soft_newline`;
5. `_line_leading_bar`;
6. `_line_leading_pipeline`;
7. `block_comment`;
8. `_delimiter_open`;
9. `_delimiter_close`;
10. `_error_sentinel`;
11. `_root_declaration_boundary`.

The state machine emits layout tokens and two zero-width delimiter-state markers. `_line_leading_bar` consumes only the preceding same-column newline when the next operator is exactly case-leading `|`; `_line_leading_pipeline` does the same when the next operator is `|>`. Both leave punctuation to the normal lexer. The distinct tokens let the parser choose between ending a match-case body and continuing its expression only after the scanner has inspected the second character, and they prevent an ordinary final newline from being captured speculatively. `_delimiter_open` and `_delimiter_close` occur immediately inside each grammar-owned `()`, `[]`, or `{}` pair. They update scanner depth without externalizing or consuming punctuation, so newline classification remains correct even in parser states where the closing literal is not yet valid.

The same translation unit recognizes the layout-coupled `block_comment` extra so nested comments remain one CST trivia node. It does not generally scan identifiers, operators, literals, declarations, or source punctuation. The TS-3107 boundary helper conservatively recognizes only the exact ASCII root keywords `let`, `type`, `module`, and `import` when followed by trivia, a comment opener, a newline, or EOF; it therefore neither duplicates Unicode XID tables nor splits identifiers such as `type人`. The error sentinel opts out when Tree-sitter probes with every symbol valid during recovery. Blank lines can be consumed as layout trivia, while comment-leading lines are inspected without including the comment in the layout token; the comment therefore remains visible in the CST.

`_root_declaration_boundary` consumes only the newline immediately before a recognized column-zero root declaration and leaves the keyword for the normal lexer. When the grammar admits this private synchronization point, the scanner resets indentation and delimiter recovery state before emitting it. In all other states, the same single-pass newline probe still emits the applicable ordinary layout token. The grammar combines the boundary with a non-emitting sentinel aliased to `=` only for a bounded missing-body recovery branch, so malformed input retains built-in `ERROR`/`MISSING` evidence and never becomes valid Ling syntax.

The scanner stores a strictly increasing stack of indentation columns including root column zero and the current delimiter depth. It caps layout, delimiter, and comment nesting at 256, matching DEC-0006. Columns and delimiter depth are stored as `uint16_t`, so editor layout is deliberately bounded at 65,535 leading spaces; the compiler remains authoritative and diagnostic-producing for pathological wider input.

Tabs in indentation count as one recovery column so editor structure remains finite. The scanner does not issue diagnostics; the compiler continues to reject tabs and inconsistent dedents with registered bilingual codes.

## Incremental state

The complete indentation stack is serialized after every recognized token:

```text
u8 version = 2
u16 little-endian stack length
u16 little-endian delimiter depth
u16 little-endian columns[stack length]
```

At the maximum layout depth the payload is 517 bytes, below Tree-sitter's 1,024-byte serialization buffer. Deserialization first resets to indentation `[0]` and delimiter depth zero. It accepts only the exact version/length, an indentation-stack length from 1 through 256, delimiter depth at most 256, root zero, and strictly increasing columns. Truncated, unknown-version, over-depth, or non-monotonic state recovers to the reset state.

## Consequences and boundaries

- Nested dedents and EOF dedents can be zero-width tokens, but every emission pops state, so the scanner cannot loop indefinitely.
- Delimiter markers are zero-width but grammar-required; each marker advances parser state, and each accepted close marker decreases scanner depth.
- LF, CRLF, and lone CR are one logical newline each.
- Blank and line-comment-only lines do not mutate the indentation stack. Nested block comments are preserved as CST extras.
- Newlines inside parsed delimiters become `_soft_newline` without mutating layout state.
- Tree-sitter recovery remains tolerant and is not a validity decision. Inconsistent-dedent, tab, depth, and unclosed-comment diagnostics remain compiler-owned.
- A root-declaration boundary may close editor-only recovery state and retain one following complete declaration as a named descendant of an incomplete binding. Queries must match declaration nodes by kind rather than assume every recovery node is a direct `source_file` child.
- Match-case and pipeline prefixes use distinct private newline tokens; scanner-state and whole-program differential tests must cover both when either grammar continuation is viable.
- Scanner symbol order, marker placement, and serialization version are internal parser compatibility surfaces. Changing any of them requires regeneration, scanner-state tests, and an incremental-reparse test.

## Rejected alternatives

- A larger regular-expression approximation cannot compare or serialize indentation state.
- Copying compiler diagnostics into the scanner would duplicate authority and cannot expose the registered bilingual diagnostic protocol.
- Using only closing literals in `valid_symbols` as delimiter-context sentinels fails before required members, where a closing literal is intentionally not yet valid. Serialized zero-width markers provide complete context without externalizing punctuation.
- Externalizing all punctuation would unnecessarily enlarge the scanner's responsibility; private delimiter-state markers preserve the normal lexer as the punctuation owner.
- A generated finite indentation grammar would impose a formatting width and create an impractical parser.
