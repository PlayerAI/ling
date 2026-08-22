# PRJ-1107-CHECK Implementation Report: Locked Project Graph Check

## Status

`Done` for the bounded RFC-0024 Preview slice. The parent PRJ-1107 task stays
`BlockedSpec` for semantic project checking, run/test/build, workspace
selection, and artifact policy.

## Normative clauses covered

- RFC-0024 §1: the command is `ling project check` and requires an explicit
  manifest path plus `--locked`.
- RFC-0024 §2–§3: the parser accepts exactly one path whose final component is
  `ling.toml`, rejects positional/unknown project options, and validates only
  the explicit local root.
- RFC-0024 §4–§5: module discovery and recursive package resolution reuse
  RFC-0002 `ling-project` APIs with `LockMode::Locked`; no network, parent
  search, lock creation, or lock rewrite is performed.
- RFC-0024 §6–§7: human output is bilingual and path-free; JSON output uses
  the Experimental `ling.project.check/0.1` current-writer-only report and
  existing diagnostic JSON values.
- RFC-0024 §8–§9: exit behavior is success/compile-error/invalid-usage using
  existing CLI conventions, and the implementation does not claim semantic
  compilation, execution, test/build artifacts, or workspace behavior.

The implementation also follows RFC-0002 and DEC-0003/DEC-0013 for the
underlying project graph and existing CLI/diagnostic boundaries.

## Implementation

- `crates/ling-cli/src/main.rs` adds the nested command parser, strict project
  options, explicit manifest loading, locked graph validation, bilingual
  human rendering, and deterministic JSON rendering.
- `crates/ling-cli/Cargo.toml` depends on the existing local `ling-project`
  library; `Cargo.lock` records no external dependency change.
- `crates/ling-cli/tests/project_check.rs` covers deterministic JSON,
  path-independence, bilingual human output, lock nonmutation, missing-lock
  diagnostics, and invalid argument handling.
- `tests/protocols/project-check/README.md` records the executable protocol
  corpus and its intentionally deferred scope.
- `PROTO-PROJECT-CHECK` is registered as Public/Experimental with version
  `ling.project.check/0.1`; it has no canonical public schema.

## Verification

The focused integration suite is run offline with the locked dependency set:

```text
cargo test -p ling-cli --test project_check --locked --offline
```

Repository governance, support, status, formatting, workspace tests, and
clippy gates must pass before the completion commit is recorded. The status
entry is intentionally left tied to the verified implementation commit by the
release/evidence update that follows.

## Compatibility and determinism

The slice adds no language syntax or semantics, diagnostic allocation,
Semantic ID, source-span behavior, Unicode table, bytecode, VM, or stable
schema. Human and JSON reports do not expose filesystem paths. Graph and
report ordering comes from the existing deterministic RFC-0002 library
boundary, and `--locked` is read-only with respect to project files.

## Deferred work

The full PRJ-1107 project API remains blocked until accepted contracts define
semantic project checking, run/test/build, workspace/member selection, and
artifact behavior. No placeholder APIs or commands were added for those
features.
