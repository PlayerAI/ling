# DEC-0037: CLI internal exit catalog boundary / CLI 内部退出码目录边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: cli-design
> Related authority/gap: `DEC-0013`, `DEC-0036`, `GAP-REGISTER`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision centralizes the already accepted exit-code constants for the
implemented CLI. It does not define output text, future command failures,
project policy, or a new public protocol.

## Question

CLI-1702 covers output and exit behavior, but its broader command/project
surface is not accepted. DEC-0013 fixes Seed run/check exit categories, and
the current implementation has the same values in `main.rs`; centralizing
those values can remove duplication without changing observable behavior.

## Decision

1. `ling-cli` may contain an internal exit catalog with the current values:
   success `0`, compile error `1`, invalid usage `2`, runtime fault `4`,
   internal error `5`, and snapshot mismatch `6`.
2. Existing command handlers import these constants; their output streams,
   diagnostic rendering, JSON schemas, and exit selection remain unchanged.
3. The catalog owns no command names, messages, retry behavior, project/lock
   policy, protocol fields, or future error categories. Missing numeric values
   remain intentionally unassigned.
4. The catalog is `pub(crate)` to the CLI binary and is not a public API or
   compatibility claim for unimplemented execution-plan commands.

## Conformance plan

- Verify every current exit constant retains its exact numeric value and that
  distinct runtime/internal/snapshot categories remain distinct.
- Run existing CLI conformance, JSON, REPL, formatter, project-check, and LSP
  launcher fixtures and compare output/exit behavior byte-for-byte.
- Verify no future command, diagnostic allocation, output schema, or retry
  policy is introduced by the catalog.

## Compatibility impact

- Adds only an internal CLI module and moves existing constants; source syntax,
  semantics, diagnostics, schemas, Semantic IDs, protocol inventory, runtime,
  bytecode, VM, and Unicode 17.0.0 behavior are unchanged.
- No command, option, dependency, lockfile, migration, or public error code is
  introduced.

## Unresolved alternatives

Unified output contracts, project/build/test failures, formatter/query/patch
errors, structured protocol responses, retry policy, and future exit mappings
require later Accepted CLI-1702/CLI-1703/CLI-1704/CLI-1705 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
