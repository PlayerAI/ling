# PRJ-1107 Authority Audit: Project API and CLI Integration

## Outcome

PRJ-1107 is correctly recorded as `BlockedSpec`. Accepted RFC-0002 provides a
deterministic, local, offline project library protocol, but explicitly leaves
CLI integration to this task and later specifications. The execution plan asks
for `CompilerHost::load_workspace`, unified project `check/run/test/build`,
`--manifest-path`, lock/offline selection, process exits, and JSON output. The
repository has no accepted public contract for those behaviors.

No second package-resolution pipeline, project CLI command, build artifact
model, test-discovery convention, or placeholder `CompilerHost` API was added.
The current file-oriented `ling` commands remain unchanged.

## Normative traceability

- Accepted RFC-0002 §1 fixes exact `ling.toml` selection and says an explicit
  manifest path selects exactly one project root; it does not define the CLI
  flag or workspace command dispatch.
- Accepted RFC-0002 §2–§14 authorizes the manifest, deterministic local graph,
  package identities, and `ling.lock/1` library protocols. It explicitly lists
  CLI integration beyond PRJ-1107 as out of scope.
- Accepted DEC-0003 fixes the current M0 CLI parser and command baseline; it
  does not authorize project commands or a build/test artifact contract.
- Accepted DEC-0013 supplies existing `main`/runtime failure categories and
  exit semantics, but does not define project-command selection, JSON result
  fields, or build/test outcomes.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the executable name to `ling`;
  stale `zero` spellings in the execution plan are not implementation
  authority.

## Specification gap

`GAP-PROJECT-CLI-INTERFACE-001` now owns the missing contract for:

- manifest-path precedence and project-root selection;
- project `check`, `run`, `test`, and `build` semantics and artifact scope;
- `--locked`/`--offline` command behavior;
- process exit mapping and machine-readable JSON output; and
- reuse of the RFC-0002 graph without duplicated package resolution.

The gap remains open because either a versioned project CLI decision or an
explicit library-only deferral is required. The implementation must not choose
between those options through code.

## Evidence and compatibility

The audit was checked against the current `crates/ling-cli` command parser,
`crates/ling-project` public library boundary, `PROTO-CLI`,
`PROTO-PACKAGE-MANIFEST`, `PROTO-LOCKFILE`, RFC-0002, DEC-0003, and DEC-0013.
No diagnostic code, schema, Semantic ID, canonical bytes, source span, Unicode
table, CLI behavior, or protocol inventory entry changed. No runtime test was
added because there is no accepted observable project CLI contract to test.

## Intentionally deferred

PRJ-1107 can start only after `GAP-PROJECT-CLI-INTERFACE-001` is resolved by an
Accepted decision. FMT-1507 and the LSP/transaction work remain separately
blocked by their registered authority gaps; this audit does not combine their
protocol choices or create a shared placeholder service.
