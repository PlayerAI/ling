# CLI-1706 Authority Audit: Shell completion and help fixtures

## Outcome

`CLI-1706` remains correctly recorded as `BlockedSpec` for the broad
command/flag/exit inventory, help lifecycle, and optional bash/zsh/fish/
PowerShell completion contract. Accepted DEC-0040 now closes only the bounded
`CLI-1706-HELP` child: an internal process fixture protects truthful help for
the commands already implemented by the parser.

No completion generator, shell script, help snapshot, public command registry,
or placeholder completion entry was added. Existing help continues to describe
only implemented `ling` commands, and `crates/ling-cli/tests/help.rs` verifies
that boundary without freezing help bytes as a public semantic artifact.

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
- Accepted DEC-0040 authorizes only an internal help-truth regression fixture;
  it does not authorize completion generation, aliases, snapshots, locale
  policy, or a future command registry.
- The execution plan allows completion to remain Preview, but a lower-level
  plan marker does not authorize a public command or generated script.

## Current interface evidence

The repository confirms both the bounded child and the missing parent boundary:

- `crates/ling-cli/src/main.rs` renders a usage string for the implemented
  `run`, `check`, `semantic`, `audit`, `test`, `fmt`, `init`, `project check`,
  `repl`, and `lsp --stdio` commands plus their accepted options.
- The bounded fixture now covers the implemented command names, `--help`/`-h`,
  and rejection of a future `query` command; it does not cover proposed future
  project, query, patch, build, or shell-specific completion behavior.
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

Until the remaining decisions and fixtures are Accepted, completion output
could advertise rejected commands or freeze shell-specific behavior before the
CLI contracts exist. The child fixture deliberately avoids that boundary.

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

The completed `CLI-1706-HELP` child keeps help truthful using the current
internal catalog. The parent implementation can begin after CLI-1701/1702
define the command and output inventories; it should generate completion only
from the accepted registry, keep help truthful, and provide shell-specific
fixtures without making help bytes a hidden semantic authority.
