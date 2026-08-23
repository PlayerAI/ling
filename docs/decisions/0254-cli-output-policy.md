# DEC-0254: CLI output policy / CLI 输出策略

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：cli-design
> 相关 RFC/缺口：DEC-0003 | DEC-0013 | DEC-0037 | DEC-0253 | CLI-1702
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines one bounded output policy for every current non-LSP Ling
command. It adds explicit human-language ordering, diagnostic color, and
quiet/verbose behavior while preserving bilingual public diagnostics,
machine-readable schemas, stdout/stderr ownership, and exit classes.

本决定为当前所有非 LSP Ling 命令定义一个有界输出策略。它增加明确的人类语言
排序、诊断颜色和 quiet/verbose 行为，同时保持公开诊断双语、机器可读 schema、
stdout/stderr 所有权与退出码类别不变。

## Question

How should CLI-1702 complete the plan's output, language, color, and verbosity
surface without localizing stable machine fields, hiding failures, corrupting
protocol output, or changing program Fault semantics?

## Decision

### 1. Policy and defaults

Every successfully parsed non-LSP command has exactly one immutable
`OutputPolicy` containing:

- format: `human` (default) or `json`;
- language: `bilingual` (default), `zh-CN`, or `en`;
- color: `auto` (default), `always`, or `never`;
- verbosity: `normal` (default), `quiet`, or `verbose`.

Each option may appear at most once. `--quiet` and `--verbose` are mutually
exclusive. Unknown values and duplicates are invalid usage with exit 2.

### 2. Language

Language selection applies only to tool-authored human diagnostics and
bilingual success labels:

- `bilingual` renders Chinese and English on the same logical message;
- `zh-CN` renders Chinese first and retains English as the second line;
- `en` renders English first and retains Chinese as the second line.

No human mode may remove either diagnostic language. Codes, severities, Facts,
repairs, spans, identifiers, paths, program output, source text, Audit Source,
Semantic Graph data, and protocol fields are never translated. Parser/usage
grammar remains canonical English under CLI-1706 ownership.

JSON always retains the existing `message_zh` and `message_en` fields and is
byte-independent of `--language`; the option is accepted so scripts can share
one invocation policy without changing schemas.

### 3. Color

Color applies only to human diagnostic rendering on stderr:

- `auto` enables ANSI only when stderr is a terminal;
- `always` enables ANSI regardless of terminal detection;
- `never` emits no ANSI.

Color never affects stdout, JSON, program output, source, artifacts, identity,
or exit status. Explicit `--color auto` or `--color always` is invalid with
`--format json`; explicit `--color never` is accepted. The implicit default is
normalized to no color in JSON mode.

### 4. Quiet and verbose

`--quiet` suppresses only auxiliary successful human summaries. It MUST NOT
suppress diagnostics, warnings, invalid usage, program Console output,
formatted source, Semantic Graph JSON, Audit Source, REPL interaction,
format-check change reports, or any failure.

`--verbose` emits exactly one deterministic bilingual event to stderr after
successful parsing and before command dispatch. The event contains only the
canonical command, format, language, color, and verbosity names; it contains no
physical path, environment, clock, process, allocation, debug, or map-order
data. Normal and quiet modes emit no such event.

Quiet and verbose are human-only and are invalid with `--format json`, because
machine report cardinality and channel behavior must not vary by verbosity.

### 5. Channels and LSP

Successful program/source/artifact/report output remains on stdout. Human and
JSON diagnostics, warnings, invalid usage, format-check change reports, and the
verbose event remain on stderr. Exit selection is independent of rendering.

`ling lsp --stdio` rejects `--format`, `--language`, `--color`, `--quiet`, and
`--verbose`: stdout remains framed protocol bytes and stderr remains transport
failure only. No terminal detection occurs for an LSP invocation.

### 6. Exit compatibility

DEC-0013 and DEC-0037 remain authoritative: 0 success, 1 source/project/test
validation failure, 2 invalid usage, 4 runtime Fault or host I/O failure, 5
internal invariant failure, and 6 semantic snapshot mismatch. Exit 3 remains
reserved. Output policy never remaps or retries an exit.

## Conformance plan

- Unit-test exact parsing, defaults, duplicates, invalid values, quiet/verbose
  exclusion, JSON restrictions, and LSP rejection for every policy option.
- Unit-test bilingual, Chinese-first, and English-first diagnostic rendering;
  explicit always/never color; JSON ANSI exclusion; and deterministic verbose
  event bytes.
- Integration-test human success-summary suppression without suppressing
  program output or failures, and verbose stderr without path leakage.
- Retain exact JSON schemas/cardinality, diagnostic codes/Facts/spans/repairs,
  command-specific output, exit-code, nonmutation, offline, and deterministic
  suites.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, and deterministic-diff gates.

## Compatibility impact

- **CLI:** adds `--language`, `--color`, `--quiet`, and `--verbose` to current
  non-LSP commands and makes human diagnostics bilingual by default.
- **Human output:** Preview wording/order may change as specified; diagnostics
  retain both languages and stable identity.
- **JSON/protocols:** no schema, field, cardinality, exit, or ANSI change.
- **Language/compiler:** no syntax, type, Effect, Checked Core, runtime,
  Semantic ID, Audit, package, bytecode, VM, ABI, or span change.
- **Determinism:** only `color=auto` observes whether stderr is a terminal; it
  observes no terminal type, locale, environment variable, or host path.
- **Unicode:** remains 17.0.0; option values are exact ASCII labels and source
  spans remain original UTF-8 bytes.
- **Migration:** scripts requiring old monolingual human diagnostics may select
  a preferred first language, but still receive both languages. JSON scripts
  require no migration.

## Unresolved alternatives

Localized usage/help grammar, locale/environment inference, configurable color
themes, progress bars, timestamps, tracing, log levels beyond quiet/verbose,
JSON event streams, retry policy, command-specific additional verbosity, shell
completion, and Stable output compatibility remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
