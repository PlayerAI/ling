# DEC-0028: Formatter CLI contract

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role：formatter-design  
> Related authority/gap: `DEC-0003`, `DEC-0023`, `GAP-FORMATTER-CLI-PROTOCOL-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision is intentionally limited to the file/stdin formatter command. It
does not define an LSP server, Workspace Edit transaction, Semantic Transaction,
format-on-save, range formatting, or a stable 1.0 protocol.

## Question

The Author Source formatter is implemented as a compiler-owned library, but the
execution plan needs a scriptable `ling fmt` entry point. The existing CLI
decision does not define formatter input, output, check-mode, or report
behavior. A command must be deterministic and must never overwrite a source
file implicitly.

## Decision

1. The command is `ling fmt`. It accepts exactly one input operand, either a
   repository path to a `.ling` file or `-` for stdin. A path is read as bytes;
   stdin is read to EOF. The command never writes the input path.
2. Stdin requires `--stdin-name <name>`. The logical name must be non-empty,
   valid UTF-8, end in `.ling`, and contain no absolute or parent (`..`)
   component. It is used only for diagnostics and the JSON report.
3. `--format human|json` selects the output mode. Human mode writes the
   formatted Author Source bytes to stdout when not checking. JSON mode writes
   exactly one `ling.format/0.1` report object and never mixes source bytes or
   human diagnostics into stdout.
4. `--check` suppresses formatted source output and reports whether the source
   already equals the formatter result. Exit status is `0` when unchanged,
   `1` when changes are required, and `2` for invalid command usage. Invalid
   UTF-8, lexical/parse errors, and formatter rejection are reported through
   the existing bilingual diagnostic channel and exit with `1`; no candidate
   text is published.
5. A successful non-check human invocation writes the formatter result exactly,
   including its prescribed LF and final-newline behavior. A broken stdout or
   stderr is a host I/O fault with the existing runtime-fault exit class.
6. The report schema is `ling.format/0.1` and has required fields `schema`,
   `source`, `check`, `changed`, and `disposition`. `disposition` is one of
   `formatted`, `unchanged`, or `invalid`. Successful JSON reports include
   `text` only when `check` is false; invalid reports include `diagnostics` and
   never include a partial `text`. Unknown fields are rejected by the schema;
   the protocol is Preview and current-writer only.
7. Formatting consumes the existing compiler `FormatDocument`/CST boundary
   and `DEC-0023` publication rules. It must preserve original UTF-8 spelling,
   comments, diagnostics, and invalid-source bytes. The CLI adds no language
   syntax, semantic identity, Audit Source field, or evaluator input.

## Conformance plan

- Exercise a valid file, a CRLF/BOM file, Chinese and decomposed identifiers,
  stdin with a logical name, and an already formatted file.
- Exercise `--check` for unchanged and changed input, missing/invalid stdin
  names, missing files, invalid UTF-8, incomplete syntax, and unknown options.
- Compare human stdout byte-for-byte with the library formatter and require
  that JSON stdout is one schema-valid object with no source leakage in check
  mode.
- Repeat each invocation with different filesystem enumeration order and
  independent processes; compare report bytes, disposition, diagnostics, and
  exit status.
- Verify that the command never writes or renames the input path and that
  output failures remain bounded host faults.

## Compatibility impact

- Adds the Preview `ling fmt` CLI surface and `ling.format/0.1` report schema;
  existing `run`, `check`, `semantic`, `audit`, and `repl` behavior is
  unchanged.
- No source, bytecode, Semantic ID, Audit Source, diagnostic-code meaning,
  Unicode-version, ABI, or LSP transaction compatibility claim changes.
- The command is stdout-only and therefore requires no source migration. A
  future in-place writer, range edit, or incompatible report needs a new
  accepted decision/schema version.

## Unresolved alternatives

- In-place writing, backup/atomic replacement, multiple input operands, range
  formatting, format-on-save, localized keyword views, and LSP Workspace Edit
  projection remain outside this decision.
- The general Semantic Graph/Semantic Transaction lifecycle and LSP position
  preconditions remain open under their registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
