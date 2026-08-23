# DEC-0253: Current CLI command model / 当前 CLI 命令模型

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：cli-design
> 相关 RFC/缺口：DEC-0003 | DEC-0036 | RFC-0004 | RFC-0024 | RFC-0025 | CLI-1701
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision accepts the current single parser and dispatcher as the bounded
CLI-1701 command model. It composes only commands that already have Accepted
authority and executable evidence; it does not advertise a planned command or
create a placeholder service abstraction.

本决定接受当前单一解析器与分派器作为有界的 CLI-1701 命令模型。它只组合已有
Accepted 依据和可执行证据的命令，不宣传计划中的命令，也不创建占位服务抽象。

## Question

Now that the project and formatter dependencies are complete, what exact
command-model boundary may CLI-1701 claim without pulling the separately
planned output, query/patch, completion, or future release surfaces forward?

## Decision

1. `ling` has one current command-selection path:
   `run(arguments) -> Command -> Options::parse -> execute`. The hierarchical
   `project check` spelling is selected at the same boundary and maps to one
   catalog value; it is not a root alias.
2. The exact implemented root commands are `run`, `check`, `semantic`, `audit`,
   `test`, `build`, `fmt`, `init`, `repl`, and `lsp`. The exact implemented
   hierarchical command is `project check`. Top-level `--help`/`-h` and
   `--version`/`-V` remain parser controls rather than command values.
3. The catalog owns only canonical command identity. `Options::parse` owns the
   command-specific accepted argument grammar and rejects unknown, mixed, or
   unsupported forms before dispatch. There is no alias fallback, prefix
   matching, environment-selected command, plugin command, or parent-directory
   command discovery.
4. `execute` is the sole post-parse dispatcher. Each branch delegates to the
   already accepted implementation boundary: the shared checked file pipeline,
   locked project pipeline, standalone test runner, `ling-format`, project
   graph checker, initializer, REPL session, or `ling-lsp` stdio server. A
   command MUST NOT introduce a duplicate parser, compiler semantic path,
   unchecked AST evaluator, formatter, project resolver, or LSP transport.
5. “Shared service” means reuse of those existing authoritative crate and
   checked-pipeline boundaries. CLI-1701 does not require a speculative public
   `CompilerHost`, dependency-injection framework, daemon, plugin API, or
   long-lived process when no implemented command needs one.
6. Help lists only current catalog commands. Unknown commands and the stale
   lower-authority spellings `zero` and `.zero` fail as invalid usage. Planned
   roots including `query`, `patch`, `explain`, `replay`, `evidence`,
   `version`, `support`, and `migrate` remain absent until their own Accepted
   authority and implementation exist.
7. Command-specific input, output, exit, schema, stability, and migration rules
   remain owned by their existing authorities. This decision composes command
   selection and dispatch but does not merge or silently broaden those
   protocols.
8. The current broad CLI version remains the workspace package version
   `0.0.1-dev` with Preview stability. Incompatible selection, command spelling,
   or dispatch ownership changes require new Accepted authority and migration
   evidence.

## Conformance plan

- Verify the catalog contains every implemented command exactly once, parses
  each implemented root, preserves the hierarchical `project check` identity,
  and rejects planned/stale roots.
- Verify help contains every implemented catalog spelling and no `query`,
  `patch`, `zero`, or `.zero` spelling.
- Retain command-specific parser, positive/negative integration, stdout/stderr,
  exit-code, offline, deterministic, and nonmutation suites for every composed
  surface.
- Verify file/project execution still consumes checked compiler results and
  that formatter and LSP dispatch retain their accepted parser/transport
  boundaries.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, and deterministic-diff gates.

## Compatibility impact

- **CLI:** accepts the already implemented command set and its single selection
  and dispatch ownership as the current Preview model; no command, alias,
  option, output field, or exit code is added or changed.
- **Protocols:** updates only the authority/evidence of `PROTO-CLI`; every
  command-specific protocol keeps its existing version and stability.
- **Language and compiler:** no syntax, type, Effect, runtime, Checked Core,
  diagnostic, Semantic ID, Audit, package, bytecode, VM, or ABI change.
- **Determinism and environment:** adds no filesystem search, environment
  selection, network access, plugin lookup, concurrency, clock, or map order.
- **Unicode:** remains 17.0.0; command selection requires valid Unicode and
  source spans remain original UTF-8 bytes.
- **Migration:** none, because observable accepted command behavior is
  unchanged. A future incompatible model requires explicit migration evidence.

## Unresolved alternatives

CLI-1702 output/language/color/verbosity policy; CLI-1705 Semantic Query and
Transaction commands; CLI-1706 completion/help fixtures; future explain,
replay, evidence, support, migration, plugin, daemon, and shell-integration
surfaces; and Stable command compatibility remain separately governed.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
