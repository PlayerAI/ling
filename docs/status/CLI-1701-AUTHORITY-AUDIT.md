# CLI-1701 Authority Audit: Unified command model

## Outcome

`CLI-1701` is correctly recorded as `BlockedSpec`. The current CLI already
has one hand-written parser and routes the implemented commands through the
checked compiler pipeline, but the execution plan's unified model also names
`init`, `test`, `fmt`, `query`, `patch`, `build`, and `lsp`. Their public
selection, dispatch, output, project, and transaction behavior is not accepted
by the repository authorities.

Accepted DEC-0036 closes only the bounded `CLI-1701-CATALOG` child: the
existing private command identity is centralized without changing behavior. No
new command, alias, dispatch service, `CompilerHost` API, project mode, or
placeholder help entry was added. The current `ling` command surface remains
unchanged.

## Normative traceability

- Accepted DEC-0003 fixes the M0 parser boundary: a small hand-written parser,
  the current five commands, and one `--format` option. It explicitly defers
  shared option groups and more complex command surfaces for later review.
- Accepted DEC-0013 fixes the Seed `run`/`check` entry behavior, exit-code
  mapping, human/JSON distinction, and checked execution order. It does not
  authorize project build/test, formatter, query/patch, or LSP commands.
- Accepted RFC-0002 defines the library project graph and lock protocols but
  leaves ambient/project CLI selection and build/test integration to PRJ-1107.
- Accepted DEC-0015 and DEC-0023 govern formatter preservation and audit
  separation, not a formatter CLI contract. Their command input, check mode,
  exit status, and report surface remain `GAP-FORMATTER-CLI-PROTOCOL-001`.
- `PROTO-CLI` in `docs/governance/protocol-inventory.toml` is a Preview
  inventory of the implemented `ling` surface only; it explicitly rejects
  unknown commands and does not authorize the execution-plan extensions.
- The open project and LSP/Semantic Transaction gaps leave project selection,
  query/patch preconditions, and public transaction fields unresolved.

## Current interface evidence

The current repository confirms the boundary:

- `crates/ling-cli/src/main.rs` accepts `run`, `check`, `semantic`, `audit`,
  and `repl`, plus `--format human|json` and the REPL capability option.
- Existing commands use the shared checked path, but there is no accepted
  command registry, service interface, or project-aware dispatch layer for
  the additional execution-plan commands.
- The DEC-0036 catalog owns only current command identity; options, execution,
  diagnostics, project selection, and public help remain in existing paths.
- `ling-project` exposes library graph/lock behavior; PRJ-1107 remains
  BlockedSpec for manifest selection, locked/offline policy, project exits,
  and JSON output.
- The current protocol inventory has no versioned public schema for command
  arguments or for query/patch transactions. Adding one from the lower-level
  plan would create an unregistered compatibility surface.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. the command/alias registry, parser precedence, help/version policy, and
   the boundary between file-oriented Seed commands and project commands;
2. the shared service interface and ownership rules that prevent each command
   from constructing a different compiler, VM, formatter, or project host;
3. per-command source/project inputs, lock/offline selection, capabilities,
   formatting, human/JSON output, and stable exit/error mapping;
4. the version and lifecycle of CLI, Semantic Query, Semantic Transaction,
   formatter, and LSP surfaces, including stale-snapshot and preserve rules;
5. protocol-inventory updates and positive, negative, deterministic,
   cross-platform, and migration fixtures for every advertised command; and
6. the explicit rule that stale `zero`/`.zero` spellings from lower-authority
   planning material never enter the public interface.

Until those decisions and fixtures are Accepted, changing the CLI parser or
adding a command would publish behavior that can conflict with PRJ-1107,
FMT-1507, or the LSP transaction gate.

## Evidence and compatibility

This audit was checked against `docs/decisions/0003-m0-tooling.md`,
`docs/decisions/0013-main-and-runtime-failures.md`, `docs/decisions/0015-audit-source-format.md`,
`docs/decisions/0023-author-source-formatter-preservation.md`, `docs/RFC-0002.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/governance/protocol-inventory.toml`, `docs/governance/gap-register.toml`,
`crates/ling-cli/src/main.rs`, and `crates/ling-project`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

The parent `CLI-1701` can begin after the command registry, project/formatter/
query contracts, and LSP/Semantic Transaction lifecycle are Accepted. The
first public implementation should reuse existing checked services and update
the protocol inventory and fixtures atomically; it must not advertise an
unimplemented command. The `CLI-1701-CATALOG` child is complete only for
DEC-0036's internal identity boundary.
