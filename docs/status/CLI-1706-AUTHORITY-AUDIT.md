# CLI-1706 Authority Audit: Shell completion and help fixtures

## Outcome

`CLI-1706` is correctly recorded as `BlockedSpec`. The execution plan asks for
command/flag/exit inventory, help fixtures, and optional bash/zsh/fish/
PowerShell completion. The current accepted CLI inventory covers only the
implemented Preview command surface; no accepted decision defines completion
generation, shell-specific quoting, or help compatibility for future commands.

No completion generator, shell script, help snapshot, command registry, or
placeholder completion entry was added. Existing help continues to describe
only implemented `ling` commands.

## Normative traceability

- Accepted DEC-0003 fixes the hand-written parser and states that shell
  completion is a later reconsideration when shared option groups or complex
  constraints exist. It does not define generated completion artifacts.
- Accepted DEC-0013 fixes Seed exit classes, but it does not define shell
  completion filtering, help output, or cross-shell quoting.
- `PROTO-CLI` inventories the current commands/options and explicitly requires
  help/version output to advertise no placeholder command. It has no shell
  completion protocol or independent help schema.
- `GAP-PROJECT-CLI-INTERFACE-001` and the formatter CLI gap leave future
  command/flag/exit inventories open; CLI-1701 and CLI-1702 are their upstream
  authority gates.
- The execution plan allows completion to remain Preview, but a lower-level
  plan marker does not authorize a public command or generated script.

## Current interface evidence

The repository confirms the missing boundary:

- `crates/ling-cli/src/main.rs` renders a usage string containing only
  `run|check|semantic|audit`, `repl`, `--format`, and the REPL capability.
- There is no command/flag inventory artifact, shell completion generator,
  shell quoting test, or help fixture that can cover the proposed future
  project, formatter, query, patch, build, or LSP commands.
- The current CLI protocol is Preview and rejects unknown commands/options;
  generating completion for unimplemented entries would advertise behavior
  that the parser rejects.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. the authoritative command, subcommand, option, alias, and exit inventory,
   including Preview versus Stable lifecycle and version markers;
2. help/version rendering policy, whether layout/wording is compatible, and
   deterministic ordering/locale behavior;
3. completion generation and quoting rules for bash, zsh, fish, and
   PowerShell, including installed-version/source-of-truth behavior;
4. handling of hidden/deprecated/unsupported commands, project modes,
   formatter/query/patch options, and exit/error values; and
5. positive, negative, shell-parser, Unicode/space/path, redirected-output,
   deterministic, and migration fixtures plus protocol-inventory updates.

Until those decisions and fixtures are Accepted, completion output could
advertise rejected commands or freeze shell-specific behavior before the CLI
contracts exist.

## Evidence and compatibility

This audit was checked against `docs/decisions/0003-m0-tooling.md`,
`docs/decisions/0013-main-and-runtime-failures.md`, `docs/ROADMAP-1.0.md`,
`docs/LANGUAGE.md`, `docs/governance/protocol-inventory.toml`,
`docs/governance/gap-register.toml`, `docs/ling_execution_plan/03-G1-V0.1-LIVING.md`,
and `crates/ling-cli/src/main.rs`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`CLI-1706` can begin after CLI-1701/1702 define the command and output
inventories. The implementation should generate completion only from the
accepted registry, keep help truthful, and provide shell-specific fixtures
without making help bytes a hidden semantic authority.
