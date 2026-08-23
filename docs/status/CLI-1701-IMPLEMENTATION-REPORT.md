# CLI-1701 implementation report

## Result

CLI-1701 is complete for the bounded current command model accepted by
DEC-0253. The repository has one command catalog, one command-selection path,
one option parser, and one post-parse dispatcher. It advertises only implemented
commands and delegates behavior to existing accepted service boundaries.

## Current model

- Root commands: `run`, `check`, `semantic`, `audit`, `test`, `build`, `fmt`,
  `init`, `repl`, and `lsp`.
- Hierarchical command: `project check`.
- Parser controls: `--help`/`-h` and `--version`/`-V`.
- Selection: exact Unicode command spelling, no prefix/alias/plugin/environment
  fallback.
- Dispatch: shared checked file/project pipelines plus the accepted formatter,
  initializer, test runner, graph checker, REPL, and LSP server boundaries.

No speculative `CompilerHost` API was added: the existing crates and checked
pipelines are the shared services required by the current commands. This keeps
the design simple and avoids an abstraction with no current consumer.

## Evidence

Existing unit and integration suites verify:

- exact, duplicate-free catalog membership and canonical display names;
- hierarchical `project check` identity and rejection of unknown subcommands;
- help coverage for every implemented command and exclusion of `query`,
  `patch`, `zero`, and `.zero`;
- command-specific option grammars, invalid usage, exits, stdout/stderr,
  deterministic output, locked/offline behavior, and nonmutation;
- checked compiler execution and separate formatter/LSP boundaries.

Repository-wide evidence is bound to the implementation commit in the task
registry after the full gates pass.

## Compatibility impact

No observable CLI behavior changes. `PROTO-CLI` remains Preview and retains the
workspace package version `0.0.1-dev`. No language semantic, diagnostic,
schema, Semantic ID, Audit, package, bytecode, VM, ABI, span, determinism, or
Unicode 17.0.0 behavior changes.

## Intentionally deferred

CLI-1702 output/language/color/verbosity, CLI-1705 query/patch, CLI-1706 shell
completion/help fixtures, future commands, plugins/daemons, and Stable
compatibility remain unimplemented and unclaimed.
