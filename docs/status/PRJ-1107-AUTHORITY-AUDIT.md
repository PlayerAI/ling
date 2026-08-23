# PRJ-1107 Authority Audit: Project API and CLI Integration

## Outcome

PRJ-1107 is `Done` for the complete accepted v0.1 scope. Accepted RFC-0025
closes `GAP-PROJECT-CLI-INTERFACE-001` for manifest-selected semantic
`check`, `run`, `test`, `build`, explicit single-root workspace selection,
locked/offline behavior, exits, JSON results, and a checked semantic artifact.

RFC-0024 and the completed `PRJ-1107-CHECK` child remain the authority and
implementation for the distinct graph-only `ling project check` command.
Accepted DEC-0058/`PRJ-1107-LOAD` and DEC-0083/
`PRJ-1107-SEMANTIC-SNAPSHOT` remain the internal library boundaries reused by
the completed parent. DEC-0250's current-surface verifier is updated from a
blocker inventory to executable completion evidence; it does not replace the
RFC-0025 authority.

## Normative traceability

- RFC-0002 §§1–7 govern exact `ling.toml` selection, the local vendored
  package graph, package/graph identities, canonical `ling.lock/1`, locked and
  offline behavior, diagnostics, and profile/target inclusion in artifact
  identity.
- RFC-0024 §§1–9 continue to govern only
  `ling project check --manifest-path PATH --locked` and
  `ling.project.check/0.1` graph validation.
- RFC-0025 §§1–2 define root `ling check/run/test/build` project mode,
  mutually exclusive file/project inputs, required `--manifest-path`,
  `--locked`, and `--offline`, and the exact one-root workspace rule.
- RFC-0025 §§3–6 require the package-aware checked snapshot, semantic check,
  checked root entry execution, and the isolated one-entry smoke test.
- RFC-0025 §§7–9 define create-new canonical
  `ling.project.artifact/0.1`, SHA-256 identity, shared
  `ling.project.command/0.1` results, exits, and failure atomicity.
- DEC-0003 and DEC-0013 retain command parser and exit-class authority;
  DEC-0058 and DEC-0083 retain the locked project and semantic query
  boundaries. No plan-only `CompilerHost` placeholder was introduced.

## Implementation boundary

`ling_cli::project::compile` reads exactly the explicit manifest, calls
`load_locked_project`, and then calls
`CompilerDb::project_semantic_snapshot`. It converts user-controlled syntax,
resolution, type, and Effect failures into existing bilingual diagnostics with
logical `package:<name>/<source>` spans. Project execution calls
`ling_eval::execute_project_main`, whose interpreter now stores a
`CheckedProgram` reference and therefore supports file and project snapshots
without accepting unchecked AST.

The root CLI preserves positional file behavior and RFC-0024 graph checking.
Manifest-selected `check` performs no entry execution; `run` captures output
for JSON purity; `test` runs exactly one isolated root entry; and `build`
publishes only a canonical checked semantic artifact for `explore` /
`semantic`. Build uses exclusive create-new publication and removes a newly
created partial file if writing or syncing fails. It never replaces an
existing file or follows an existing symlink.

The explicit single-root workspace selection contract is complete for
manifest version 1: the root and its vendored dependencies are checked, while
dependencies are not selectable members. No ambient search, current-directory
default, member flag, registry, network, shell, environment, cache, or lock
update occurs.

## Compatibility

- Source syntax, type/Effect semantics, entry rules, and runtime behavior are
  unchanged; the new commands compose existing checked services.
- Positional file commands and `ling.project.check/0.1` remain compatible.
- `ling.project.command/0.1` and `ling.project.artifact/0.1` are new
  Experimental, current-writer-only boundaries.
- `L-IO-0005` is a monotonic Preview diagnostic allocation for artifact
  publication. The Diagnostic JSON schema is unchanged.
- Existing package-aware Semantic IDs are embedded by value and unchanged;
  artifact SHA-256 identity is separate and includes profile/target through
  the complete canonical bytes.
- Artifacts and reports exclude host paths, timestamps, environment data,
  unordered iteration, and allocation identity. Unicode remains 17.0.0.

## Remaining future work

The parent task is not blocked by optional future extensions. Source-level
test declarations, filters, parallelism, multi-member workspace manifests,
implicit discovery, lock update mode, output replacement/default directories,
caches, bytecode/native/Wasm backends, Native/Critical profiles, registry and
publication, signatures, and Stable 1.0 lifecycle require later Accepted
authority and are not claimed here.
