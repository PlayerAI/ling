# DEC-0040: Truthful implemented-command help fixture / 已实现命令帮助事实夹具

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: cli-design  
> Related authority/gap: `DEC-0003`, `DEC-0013`, `DEC-0036`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `CLI-1706-HELP` child. It does not
close the parent shell-completion or future command-inventory contract;
`CLI-1706` remains `BlockedSpec` for those surfaces.

## Question

The current hand-written CLI already emits help and usage text, but the
execution plan has no accepted contract for shell completion, future command
discovery, help byte compatibility, or locale policy. A small regression
fixture can still protect the accepted `PROTO-CLI` rule that help advertises
only commands that the parser implements.

## Decision

1. The current internal `Command` catalog is the sole test source for the
   implemented-command set. The help text must mention each catalogued command,
   including the hierarchical `project check` entry, and must not advertise
   unimplemented planned names such as `build`, `query`, or `patch`.
2. `ling --help` and `ling -h` remain successful, UTF-8, human help paths with
   no diagnostic output. Their current output is tested for semantic coverage
   and equality, but the decision does not freeze byte layout, wording,
   locale, line wrapping, or a future compatibility version.
3. An unknown future command remains invalid usage: it exits with the accepted
   invalid-usage class, writes the existing bilingual usage diagnostic to
   stderr, and does not emit a partial stdout response. The fixture must not
   allocate a new diagnostic code or protocol schema.
4. The fixture is an internal process-test boundary. It does not add shell
   completion generation, aliases, option discovery, a public command
   registry, a completion script, or a new CLI protocol.

## Conformance plan

- Run `ling --help` and `ling -h` in independent processes and verify their
  successful streams expose the implemented command set and no stale planned
  command.
- Run an unknown future command such as `query` and verify exit class 2,
  empty stdout, and usage text on stderr.
- Repeat the fixture offline and with ordinary Unicode-capable process I/O;
  verify no source, Semantic ID, diagnostic registry, schema, or generated
  artifact changes.

## Compatibility impact

- Adds only an internal catalog test helper and process fixtures. Existing
  command names, options, output schemas, diagnostics, exit values, runtime,
  bytecode, and Unicode 17.0.0 behavior are unchanged.
- No public protocol, completion artifact, migration rule, or help-byte
  compatibility promise is introduced.

## Unresolved alternatives

The full CLI-1701/CLI-1702 command, option, alias, exit, and lifecycle
inventories; shell-specific completion and quoting; hidden/deprecated command
policy; locale/order guarantees; generated scripts; and help snapshots require
later Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

## Subsequent accepted authority

RFC-0027 subsequently implements and advertises `query` and `patch`. The
fixture therefore continues to reject only still-unimplemented plan names and
the stale `zero` / `.zero` spellings; this historical decision does not
override the later Accepted command authority.

RFC-0028 subsequently implements and advertises `completion` and versions the
four generated shell scripts. It preserves this decision's rule that ordinary
help wording and layout are not a canonical byte protocol.
