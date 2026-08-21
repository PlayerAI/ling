# CLI-1702 Authority Audit: Output and exit behavior

## Outcome

`CLI-1702` is correctly recorded as `BlockedSpec`. The accepted Seed subset
already fixes `--format human|json`, bilingual diagnostic rendering, and the
process exit classes. The execution-plan extension additionally requests
language selection, color, quiet/verbose controls, and one output contract
across future project, formatter, query, patch, and test commands; those
extensions do not have accepted semantics or fixtures.

No new flag, localization mode, color policy, quiet/verbose behavior,
diagnostic code, output schema, or exit class was added. Existing `ling`
output and exit behavior remains unchanged.

## Normative traceability

- Accepted DEC-0003 fixes the M0 `--format` option and the hand-written parser
  boundary; it does not define language, color, verbosity, or future command
  output policies.
- Accepted DEC-0013 fixes exit codes 0–6 for Seed run/check behavior and
  requires human/JSON rendering to leave the exit code unchanged. It separates
  compile errors, runtime Fault/host failures, internal errors, and snapshot
  mismatches.
- `docs/SEMANTICS.md` §26 requires registered bilingual diagnostics, stable
  codes, root-cause ordering, original UTF-8 spans, structured Facts/repairs,
  and locale-aware rendering. It does not authorize CLI-specific flags or
  wording/colour compatibility promises.
- `PROTO-CLI`, `PROTO-CLI-EXIT`, `PROTO-HUMAN-OUTPUT`, and
  `PROTO-JSON-DIAGNOSTIC` are Preview/current protocol entries for the
  implemented surface. They reject unregistered options and do not cover the
  execution-plan's future command families.
- Project command output and formatter CLI output remain gated by
  `GAP-PROJECT-CLI-INTERFACE-001` and `GAP-FORMATTER-CLI-PROTOCOL-001`.

## Current interface evidence

The current repository confirms the accepted subset:

- `crates/ling-cli/src/main.rs` parses `--format human|json`, maps the
  accepted exit classes, and keeps diagnostics on stderr while successful
  program output goes to stdout.
- The diagnostic registry and renderers already preserve bilingual stable
  codes, structured JSON fields, and original byte spans; no language-select
  flag is exposed by the current parser.
- No accepted policy defines terminal color detection/override, quiet versus
  verbose event selection, localization fallback, or output aggregation for
  project/formatter/query/patch/test commands.
- The current CLI inventory deliberately advertises only implemented commands;
  adding output switches before their command contracts would make a Preview
  protocol appear stable by accident.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. language selection, fallback, and bilingual diagnostic policy for human and
   JSON modes, including whether machine-readable fields are localized;
2. color detection, explicit enable/disable precedence, TTY/redirect behavior,
   and whether ANSI bytes may appear in any machine-readable channel;
3. quiet/verbose event levels, progress/stderr routing, deterministic ordering,
   and interaction with cancellation and partial failures;
4. per-command exit/error mapping, project/lock/formatter/query/patch failures,
   and compatibility rules for reserved values;
5. versioned report schemas and protocol-inventory updates, plus positive,
   negative, locale, TTY, redirected-output, CRLF/Unicode, and migration
   fixtures; and
6. the rule that formatting changes never alter stable diagnostic identity,
   source spans, Semantic IDs, or program Fault semantics.

Until those decisions and fixtures are Accepted, changing the parser or
renderer would either break the current Preview contract or freeze behavior
for commands that are not implemented.

## Evidence and compatibility

This audit was checked against `docs/decisions/0003-m0-tooling.md`,
`docs/decisions/0013-main-and-runtime-failures.md`, `docs/SEMANTICS.md`,
`docs/ERROR-CODES.md`, `docs/governance/protocol-inventory.toml`,
`docs/governance/gap-register.toml`, `docs/ROADMAP-1.0.md`,
`crates/ling-cli/src/main.rs`, and `crates/ling-diagnostics`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`CLI-1702` can begin after the command model and per-command contracts are
Accepted. The first implementation should preserve the existing exit classes
and machine-readable fields, add only registered options, and prove stdout,
stderr, locale, TTY, and redirected-output behavior with fixtures.
