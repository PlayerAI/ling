# DEC-0256: Current test command / 当前测试命令

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：cli-design
> 相关 RFC/缺口：DEC-0039 | RFC-0025 | DEC-0253 | DEC-0254 | CLI-1704
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes CLI-1704 by composing its two already Accepted modes:
the standalone file/directory runner from DEC-0039 and the explicit locked,
offline project entry smoke test from RFC-0025. It does not invent test syntax.

本决定组合两个已有 Accepted 模式以关闭 CLI-1704：DEC-0039 的独立文件/目录
运行器，以及 RFC-0025 的显式锁定、离线工程入口烟雾测试。它不发明测试语法。

## Question

With no Accepted source-level test declaration, what exact existing test
conventions constitute the complete CLI-1704 command without inferring hidden
syntax, targets, or workspace behavior?

## Decision

1. `ling test PATH` selects DEC-0039 standalone mode. `PATH` is one `.ling`
   file or recursively discovered directory; selected logical names are sorted,
   files execute independently through the checked pipeline, and results use
   `ling.test/0.1`.
2. `ling test --manifest-path PATH --locked --offline` selects RFC-0025 project
   mode. It loads exactly one RFC-0002 locked project and executes exactly one
   isolated root-entry smoke test named `<package>::<entry-module>` through the
   package-aware checked snapshot. The result uses
   `ling.project.command/0.1`.
3. The modes are mutually exclusive. There is no ambient manifest discovery,
   workspace selection, dependency test execution, or conversion of project
   modules into standalone cases.
4. Ling has no Accepted source-level test declaration. Therefore CLI-1704 MUST
   NOT infer annotations, naming conventions, assertions, snapshots, property
   tests, filters, parallelism, coverage, or hidden test targets. The explicit
   standalone-program and project-entry smoke conventions are the complete
   current test surface.
5. Both modes are sequential, offline, deterministic, and consume checked
   Typed Core. Console output is captured for reports; failures preserve
   bilingual registered diagnostics and original UTF-8 spans.
6. DEC-0254 governs human/JSON, language order, color, quiet, and verbose
   behavior. Existing report schemas, channels, and exit precedence remain
   unchanged: 0 pass, 1 compile/entry/empty-selection failure, 2 usage, 4
   runtime or host I/O, 5 internal incident, and 6 snapshot mismatch.
7. Stale `zero test` and `.zero` spellings remain invalid. Future test syntax
   or project test expansion requires Accepted language/project authority and
   a versioned protocol migration.

## Conformance plan

- Retain sorted standalone file/directory success, compile-failure,
  runtime-failure, continued execution, capture, empty-selection, and schema
  fixtures.
- Retain explicit locked/offline project selection, single root-entry identity,
  capture, compile/runtime failure, path-free output, and nonmutation fixtures.
- Verify mixed modes and incomplete project selection fail with exit 2.
- Verify DEC-0254 rendering never changes test selection, report bytes, or exit.
- Run workspace, CI, governance, support, status, RC0, traceability, Clippy,
  formatting, and deterministic-diff gates.

## Compatibility impact

No command form, schema, report field, diagnostic allocation, exit, test
selection, source syntax, Semantic ID, span, runtime, bytecode, VM, ABI, or
Unicode 17.0.0 behavior changes. This accepts the current Preview/Experimental
writers and has no migration.

## Unresolved alternatives

Test declarations, manifest targets, workspaces, filtering, assertions,
snapshots, property tests, parallel scheduling, cancellation, timeout,
coverage, benchmarks, and Stable test compatibility remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
