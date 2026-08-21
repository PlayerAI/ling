# CLI-1703 Authority Audit: Project initialization

## Outcome

`CLI-1703` is correctly recorded as `BlockedSpec`. The execution plan
proposes an `init` command that writes a minimal project, `.gitignore`, an
example `.ling` file, and optionally `.zed/tasks.json`, with a template version
in the manifest. No accepted authority defines that generated project
contract, and the plan's `zero init` spelling is stale.

No `init` command, template, filesystem writer, `.gitignore`, editor task file,
manifest field, or placeholder API was added. The current repository remains
read-only with respect to project creation.

## Normative traceability

- Accepted RFC-0002 defines the library-only manifest, package graph, module
  visibility, lock, and content/graph identity protocols. It does not define a
  CLI initializer, template files, generated metadata, or project-root
  selection.
- Accepted DEC-0003 fixes the current hand-written CLI parser and rejects
  unimplemented commands from help; it does not authorize `init`.
- Accepted DEC-0007 fixes source/module file boundaries and package graph
  semantics, not a project scaffold or template version field.
- `GAP-PROJECT-CLI-INTERFACE-001` keeps manifest-path precedence, project mode,
  lock/offline selection, and project command behavior open until an accepted
  CLI decision exists.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require deterministic logical
  names, path-free identities, source-span preservation, and no ambient
  network behavior. They do not define generated files or editor task
  metadata.
- Lower-authority execution material uses `zero`; the accepted public command
  and source names remain `ling` and `.ling`.

## Current interface evidence

The current repository confirms the missing boundary:

- `crates/ling-cli/src/main.rs` has no `init` parser branch and its help text
  advertises only implemented commands.
- `ling-project` can read and validate explicit manifests and locks, but it has
  no accepted writer for a new project root or template metadata.
- The protocol inventory has no `ling.init` or template protocol, and no
  fixture freezes generated file names, bytes, permissions, line endings, or
  editor integration.
- Writing a scaffold before the project CLI decision would choose manifest
  naming, package identity, source layout, lock policy, and optional editor
  files as de facto compatibility commitments.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. root selection, existing-directory/overwrite rules, package name/version
   defaults, and deterministic manifest/lock interaction;
2. the exact generated file set, `.ling` example semantics, `.gitignore`
   policy, line endings/encoding, and template version identity;
3. whether `.zed/tasks.json` or other editor files are generated, their schema
   ownership, opt-in behavior, and no-network guarantee;
4. command output, `--format`, localization, exit/error mapping, dry-run and
   failure-atomicity behavior;
5. protocol inventory and migration entries, plus positive, negative,
   existing-root, Unicode/CRLF, offline, and byte-deterministic fixtures; and
6. the explicit `ling`/`.ling` naming rule that excludes stale `zero` names.

Until those decisions and fixtures are Accepted, implementing `init` would
write user files under an invented contract and could silently select a
different project identity than RFC-0002 permits.

## Evidence and compatibility

This audit was checked against `docs/RFC-0002.md`,
`docs/decisions/0003-m0-tooling.md`, `docs/decisions/0007-module-and-file-boundaries.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/governance/protocol-inventory.toml`, `docs/governance/gap-register.toml`,
`crates/ling-cli/src/main.rs`, and `crates/ling-project`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`CLI-1703` can begin after the project CLI decision defines initialization and
the template/protocol fixtures exist. The implementation must be offline,
failure-atomic, deterministic, and use the accepted `ling`/`.ling` names
without introducing an unregistered editor file format.
