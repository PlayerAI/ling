# DEC-0222: Internal CLI and Exit-Code Freeze boundary evidence / 内部 CLI 与退出码冻结边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: CLI protocol governance
> 相关规范/缺口：`DEC-0013` | `DEC-0036` | `DEC-0037` | `DEC-0040` | `PROTO-CLI` | `PROTO-CLI-EXIT`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `PROTO-6204-OBSERVATION`. It
freezes the truthful internal inventory of commands and assigned exit values
without promoting the Preview/Experimental CLI to Stable or defining behavior
for plan-only commands.

本决定授权 `PROTO-6204-OBSERVATION` 使用有界证据，固定真实的内部命令清单
与已分配退出值；不将 Preview/Experimental CLI 提升为 Stable，也不定义仅存在
于计划中的命令行为。

## Question

Which CLI and exit-code facts are executable today and safe to retain while a
complete 1.0 command, output, path, color, offline, and compatibility contract
is still absent?

## Decision

1. The exact implemented command inventory is nine root commands—`run`,
   `check`, `repl`, `semantic`, `audit`, `fmt`, `init`, `test`, and `lsp`—plus
   hierarchical `project check`. `project` is not a root catalog alias.
2. The command catalog must reject plan-only roots `build`, `query`, `patch`,
   `replay`, `explain`, `evidence`, `version`, and `support`. Help continues to
   advertise only implemented commands; `--version` is a flag, not a
   `version` command.
3. The exact assigned process exit values remain `0`, `1`, `2`, `4`, `5`, and
   `6` for success, compile/check failure, invalid usage, runtime/host fault,
   internal failure, and Semantic snapshot mismatch. Value `3` remains
   unassigned and carries no compatibility meaning.
4. `crates/ling-types/tests/cli_exit_freeze_evidence.rs` records sixty
   test-local command, option, output, exit, policy, lifecycle, determinism,
   and fixture boundaries with explicit ordering and duplicate rejection.
5. Opaque bytes tagged `ling.cli-exit-freeze-observation/0` are test evidence
   only. They are not a CLI schema, completion format, output contract,
   command registry, or compatibility marker.
6. No command, alias, option, default, output byte, exit meaning, color/path
   policy, offline rule, schema, diagnostic, or stability promotion is added.
   Public `PROTO-6204` remains `BlockedSpec`.

## Normative basis

- Accepted `DEC-0013` defines the current Seed process exit classes and keeps
  value `3` reserved rather than assigned.
- Accepted `DEC-0036` centralizes only the implemented internal command names;
  accepted `DEC-0037` centralizes only assigned exit constants.
- Accepted `DEC-0040` requires truthful help and rejection of unknown future
  commands without freezing help bytes or shell completion.
- Accepted `DEC-0028`, `DEC-0038`, `DEC-0039`, RFC-0004, and RFC-0024 add
  scoped formatter, init, standalone test, LSP, and project-check surfaces;
  none creates a complete Stable 1.0 command matrix.
- `PROTO-CLI` and `PROTO-CLI-EXIT` remain Preview, while project check is
  Experimental and unsupported plan-only command families remain explicit in
  the support matrix.

## Conformance plan

- Assert the exact ordered ten-value internal catalog, nine parseable roots,
  hierarchical project-check spelling, and rejection of plan-only roots.
- Assert the exact assigned exit list and explicit absence of value `3`.
- Assert all sixty local boundaries, exact order, duplicate rejection, and
  order-independent opaque bytes.
- Run CLI unit/conformance, support, protocol, governance, status, workspace,
  lint, formatting, deterministic, and offline gates.

## Compatibility impact

Existing command parsing, options, defaults, help/version output, stdout and
stderr routing, exit values, JSON/human schemas, diagnostics, project and
formatter behavior, language/runtime semantics, source spans, and Unicode
17.0.0 remain unchanged.

## Unresolved alternatives

The Stable 1.0 command matrix; plan-only commands; shared option grammar;
defaults and environment precedence; color and path policy; project
run/test/build and workspace selection; stdout/stderr byte compatibility;
human/JSON schema lifecycle; cancellation and future exits; shell completion;
locale; migration; and release policy remain open under `PROTO-6204`,
incomplete G1-G5 exits, ROADMAP-1.0, support-matrix blockers, and protocol
lifecycle work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
