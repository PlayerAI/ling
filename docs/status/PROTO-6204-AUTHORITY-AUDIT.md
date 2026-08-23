# PROTO-6204 Authority Audit

- Task: `PROTO-6204` — CLI and Exit-Code Freeze
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:136-150`
- Release: G6
- Status: `BlockedSpec`

## Decision

PROTO-6204 is `BlockedSpec`. The G6 checklist asks for a frozen syntax,
flags, defaults, exit codes, stdout/stderr behavior, human/JSON modes, color
policy, path normalization, and offline behavior for every command. It does
not define which command families are part of the 1.0 public surface, how the
Seed contracts compose with project, formatter, package, VM, support, or
future protocol commands, or which versioned schemas and compatibility rules
apply to their output.

The repository has accepted decisions for the M0 hand-written parser, Seed
runtime failures, Audit Source, REPL sessions, formatter, initialization,
standalone test-file execution, bounded project graph checking, and the LSP
launcher. Those decisions are scoped contracts, not a complete 1.0 freeze.
The active `PROTO-CLI` and `PROTO-CLI-EXIT` records remain Preview; accepted
project and formatter gaps close only bounded children, while support-matrix
entries explicitly mark several proposed commands unsupported. Freezing all
plan-listed behavior now would create automation commitments without accepted
authority.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:136-150` is a non-normative freeze checklist.
  It names the dimensions of a CLI contract but does not assign command
  ownership, schema versions, compatibility, or 1.0 stability.
- Accepted `DEC-0003` authorizes the small hand-written M0 parser, the
  scriptable REPL baseline, and the conformance runner. It does not authorize
  project selection, formatter, package, support, bytecode, backend, or
  registry commands.
- Accepted `DEC-0013` fixes the Seed entry-point/runtime-failure distinction
  and the current exit classes: `0` success, `1` compile/check failure, `2`
  invalid usage, `4` runtime/host fault, `5` internal compiler error, and `6`
  semantic snapshot mismatch; `3` remains reserved. It does not freeze every
  command's defaults, path policy, or future Result-main behavior.
- Accepted `DEC-0015` fixes the `ling audit` output boundary, its stdout
  failure handling, and the meaning of `--format`; accepted `DEC-0016` fixes
  REPL human/JSON events and scripted-session exits. Neither defines a
  repository-wide color, path-normalization, or offline policy.
- `docs/governance/protocol-inventory.toml` records `PROTO-CLI`,
  `PROTO-CLI-EXIT`, human output, and diagnostic JSON as Preview. The current
  parser surface is `--help`/`-h`, `--version`/`-V`, roots `run`, `check`,
  `repl`, `semantic`, `audit`, `fmt`, `init`, `test`, and `lsp`, plus
  hierarchical `project check`; it has no independent Stable 1.0 CLI schema.
- `GAP-PROJECT-CLI-INTERFACE-001` is Accepted through RFC-0024 only for the
  bounded explicit-manifest, locked/offline `ling project check` graph
  validation command. Project semantic check/run/test/build and workspace
  selection remain unresolved.
- `GAP-FORMATTER-CLI-PROTOCOL-001` is Accepted through DEC-0028 only for the
  Preview stdout-only formatter CLI. Incompatible report or write-in-place
  extensions remain unresolved.
- The support matrix marks `ling version --format json` and `ling support
  --format json` unsupported, and separately excludes CLI project/package
  installation, publication/registry, bytecode emission/loading, and backend
  selection. Those entries cannot be silently frozen by this task.
- Root `AGENTS.md` requires accepted authority before public protocols,
  stable claims only after ROADMAP gates and executable fixtures, bilingual
  `L-<DOMAIN>-<NUMBER>` diagnostics, deterministic/offline behavior, preserved
  UTF-8 spans and Unicode 17.0.0, accepted `ling`/`.ling` naming, and no
  stale `zero` surfaces.

## Evidence in this repository

The current CLI has conformance coverage for the implemented Seed commands,
diagnostics, JSON rendering, Audit Source, REPL sessions, and the Preview exit
mapping. The protocol inventory and support matrix explicitly distinguish
those implemented surfaces from Future or Unsupported project, formatter,
package, support, bytecode, backend, and registry commands. There is no
single accepted 1.0 command matrix containing all syntax, defaults, path,
color, stdout/stderr, offline, and compatibility rules requested by the plan.

The accepted project and formatter gap resolutions provide bounded versioned
decisions and fixtures, but they do not define project run/test/build,
workspace selection, formatter write-in-place behavior, or a universal 1.0
contract. Existing fixtures cannot be promoted to a universal freeze without
changing their authority or inventing behavior for absent commands.

## Required authority before implementation

An accepted versioned CLI decision (or coordinated accepted replacements) must
define, at minimum:

1. The complete 1.0 command matrix and ownership of each command, including
   which current Preview commands remain in scope and which project,
   formatter, package, support, VM, backend, editor, and registry commands
   remain explicitly unsupported or Future.
2. Exact grammar, option precedence, defaults, arity, environment handling,
   stdin/stdout/stderr routing, color policy, path normalization and display
   rules, locale/bilingual rendering, and offline/locked dependency behavior.
3. A stable exit-code table, including compile/check, invalid usage, runtime
   and host faults, internal failures, snapshot mismatches, cancellation,
   partial REPL sessions, and reserved values; human and JSON modes must not
   silently change the exit class.
4. Versioned JSON/diagnostic/report schemas, unknown-field and future-version
   behavior, deterministic ordering, source-span and Semantic ID handling,
   resource limits, and migration guidance for incompatible command changes.
5. Offline positive, negative, malformed-argument, path/CRLF/Unicode,
   human/JSON, color, stdout/stderr, locked-project, migration, and
   cross-process fixtures, plus generated protocol/support/status drift checks.

## Compatibility and deferred work

This audit changes no command parser, command name, option, default, exit
code, stdout/stderr routing, JSON schema, diagnostic allocation, color/path
policy, dependency, package behavior, LSP route, or support claim. It
preserves the accepted `ling` CLI and `.ling` source extension, scoped Seed
contracts, current Preview protocol records, bilingual diagnostics, original
UTF-8 spans, Unicode 17.0.0, deterministic/offline requirements, and explicit
Preview/Future/Unsupported states.

It deliberately adds no command, flag, backend selector, project/formatter
integration, JSON/report protocol, migration adapter, public API, diagnostic,
dependency, or placeholder, and introduces no stale `zero` names. Future
freeze work may proceed only after the remaining command-family authorities
and their fixtures are Accepted, the unresolved project/formatter extensions
are resolved, and the compatibility/support matrix is executable. The
implementation must keep
the checked compiler pipeline and must not expose host paths, allocation,
environment, or map-order details as CLI semantics.
