# DEC-0036: CLI internal command catalog boundary / CLI 内部命令目录边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: cli-design
> Related authority/gap: `DEC-0003`, `DEC-0013`, `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision extracts the already implemented command names into one internal
catalog. It does not add commands, aliases, shared option groups, project
selection, shell completion, or a new public CLI protocol.

## Question

The current hand-written CLI parser has command identity and display spelling
embedded in `main.rs`, while CLI-1701's broader unified command model is not
accepted. DEC-0003 authorizes a small parser for the current surface, so the
identity table can be centralized without advertising future commands.

## Decision

1. `ling-cli` may contain an internal `Command` catalog enumerating the current
   implemented roots `run`, `check`, `repl`, `semantic`, `audit`, `fmt`, and
   `lsp`, plus the already implemented hierarchical `project check` value.
2. Root parsing returns only those implemented root commands. `project check`
   remains selected by the existing two-token dispatch in `main.rs`; this child
   does not add `project build`, `init`, `test`, `query`, or `patch`.
3. Each catalog value has one canonical display name used by current parser
   errors and reproduction labels. The catalog owns no options, paths,
   project manifests, output bytes, exit codes, diagnostics, or execution.
4. The catalog is `pub(crate)` to the binary target and is not a public Rust,
   shell-completion, JSON, or protocol API. Existing parser validation and
   usage text remain unchanged.

## Conformance plan

- Verify every current root command parses to exactly one catalog value and
  canonical display names round-trip; unknown planned commands remain rejected.
- Verify `project check` retains its hierarchical identity without becoming a
  root alias and that no option or execution behavior moves into the catalog.
- Verify parser/conformance output, exit codes, diagnostics, and usage text are
  byte-identical for existing accepted and rejected inputs.

## Compatibility impact

- Adds only an internal CLI module and refactors existing private command
  identity; source syntax, semantics, diagnostics, schemas, Semantic IDs,
  protocol inventory, runtime, bytecode, VM, and Unicode 17.0.0 behavior are
  unchanged.
- No dependency, manifest, lockfile, command name, alias, option, or migration
  surface is introduced.

## Unresolved alternatives

The broader CLI-1701 command registry, shared option groups, project/build/test
selection, formatter/query/patch transactions, shell completion, help/version
policy, protocol lifecycle, and cross-command service ownership require later
Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
