# PRJ-1107 Implementation Report

> Status: **Done**
> Authority: Accepted RFC-0025, RFC-0002, RFC-0024, DEC-0003, DEC-0013,
> DEC-0058, and DEC-0083

## Delivered

- Added explicit locked/offline project modes for `ling check`, `ling run`,
  and `ling test`, plus project-only `ling build`.
- Preserved positional file modes and the graph-only
  `ling project check` / `ling.project.check/0.1` command.
- Added shared `ling_cli::project` orchestration over
  `load_locked_project` and `CompilerDb::project_semantic_snapshot` without a
  duplicate package resolver or placeholder compiler host.
- Added package-aware checked entry execution through
  `ling_eval::execute_project_main`; both file and project execution now share
  an interpreter that stores only `&CheckedProgram`.
- Added deterministic `ling.project.command/0.1` check/run/test/build JSON
  results with captured run/test stdout and existing exit classes.
- Added canonical create-new `ling.project.artifact/0.1` checked semantic
  artifacts for profile `explore` and target `semantic`, plus SHA-256 identity
  over the complete bytes.
- Added registered bilingual `L-IO-0005` artifact publication failures and
  public checked-project syntax/resolution/type/Effect diagnostic projection.
- Added integration coverage for deterministic path-free checking,
  dependency-using execution, isolated testing, artifact bytes/identity,
  overwrite refusal, semantic failure spans, and invalid/mixed CLI forms.

## Normative clauses covered

- RFC-0002 §§1, 4, 6, and 7: explicit root, local graph, canonical locked
  resolution, offline operation, diagnostics, and nonmutation.
- RFC-0002 §5: artifact identity includes fixed profile and target in the
  hashed canonical bytes.
- RFC-0025 §§1–2: exact commands/options, file/project exclusivity, one-root
  workspace selection, required locked/offline behavior.
- RFC-0025 §§3–6: checked snapshot, semantic check, checked root execution,
  and one-entry isolated smoke test.
- RFC-0025 §§7–9: canonical artifact, shared result protocol, exits, and
  failure atomicity.

## Evidence executed

- `cargo check -p ling-cli -p ling-db -p ling-eval --locked --offline`
- `cargo test -p ling-cli --locked --offline`
- `cargo test -p ling-cli --test project_commands --locked --offline`

The implementation milestone's full workspace, governance, support, status,
traceability, Clippy, formatting, and diff gates are recorded in the task
status entry after the implementation commit is bound.

## Compatibility and determinism

Existing file commands, graph-check output, language semantics, source spans,
Diagnostic JSON, package and graph identities, package-aware Semantic IDs,
bytecode, VM, ABI, and Unicode 17.0.0 remain unchanged. The new result and
artifact protocols are Experimental. `L-IO-0005` is monotonic. Project
reports/artifacts are path-free and exclude timestamps, environment values,
filesystem metadata, unordered iteration, and allocation identity.

## Specification gaps or conflicts

`GAP-PROJECT-CLI-INTERFACE-001` is resolved by RFC-0024 plus RFC-0025 for the
complete PRJ-1107 v0.1 scope. The lower-authority plan's stale `zero` name and
proposed `CompilerHost::load_workspace` placeholder were not implemented;
higher authority fixes `ling`, explicit `ling.toml`, and reuse of existing
complete library services.

## Intentionally deferred

Source test declarations/filtering/parallelism, multi-member workspace
manifests, implicit discovery, lock update mode, output replacement/default
directories, caches, bytecode/native/Wasm targets, Native/Critical profiles,
registry/publication/signatures, and Stable 1.0 promotion require separate
Accepted authority and executable fixtures.
