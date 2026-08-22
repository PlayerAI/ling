# CLI-1704 Authority Audit: Project test command

## Outcome

`CLI-1704` remains the parent planning row and is still `BlockedSpec` for the
full project-test surface. Accepted DEC-0039 now closes the bounded
`CLI-1704-FILE` child: an explicit, offline, standalone `.ling` file runner
with deterministic discovery, captured output, and a versioned Preview report.
The repository's Rust and conformance tests remain implementation evidence,
not a replacement for the deferred project-test convention.

The bounded child adds a `ling test` parser branch, `ling.test/0.1` report,
standalone file/directory discovery, and checked-pipeline execution. It does
not add annotations, manifest test targets, workspace selection, filtering,
assertions, snapshots, property tests, parallelism, cancellation, or a
project-test API.

## Normative traceability

- Accepted RFC-0002 defines project graph and lock inputs but does not define a
  test convention, test target model, or project test command.
- Accepted DEC-0003 identifies `crates/ling-cli/tests/conformance.rs` and
  `tests/conformance/` as repository conformance evidence; it does not make
  those fixtures a user-facing `ling test` protocol.
- Accepted DEC-0013 fixes Seed `run`/`check` exit classes and execution order,
  but it does not define test discovery or structured test events.
- `docs/SEMANTICS.md` requires deterministic execution, structured diagnostics,
  and capability/effect boundaries. It has no accepted test declaration,
  isolation, ordering, snapshot, or property-test semantics.
- `GAP-PROJECT-CLI-INTERFACE-001` leaves project test selection, locked/offline
  policy, exits, and JSON output open. DEC-0039 authorizes only the explicit
  standalone-file Preview child, not a project test surface.
- Accepted DEC-0039 defines the exact file/directory operand rules, sorted
  discovery, checked execution, captured Console output, report fields,
  diagnostics, and exit precedence implemented by `CLI-1704-FILE`.
- The execution plan's stale `zero test` spelling and any proposed annotations
  cannot enter implementation without a higher-authority decision.

## Current interface evidence

The repository confirms the remaining project boundary:

- `crates/ling-cli/src/main.rs` advertises only the bounded `ling test` file
  runner; it does not advertise project/workspace test behavior.
- `crates/ling-cli/tests/conformance.rs` runs fixed repository fixtures through
  Rust's test harness; it has no user test discovery or project event schema.
- `tests/conformance/` contains compiler/CLI acceptance fixtures, not a
  versioned source-level test declaration or expected test ordering contract.
- `ling-project` provides graph/lock validation but no test target inventory,
  package test isolation, or failure aggregation protocol.
- `crates/ling-cli/src/test_runner.rs` and `crates/ling-cli/tests/test.rs`
  provide deterministic standalone-file execution and failure evidence.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. source-level test declaration syntax or an explicit fixture-only policy,
   including module/package ownership and name/identity rules;
2. discovery, filtering, ordering, dependency/capability isolation, timeout,
   cancellation, and repeatability semantics;
3. project and file-mode selection, lock/offline behavior, build/check reuse,
   exit/error mapping, and human/JSON test event schemas;
4. failure aggregation, stdout/stderr capture, snapshot/property-test policy,
   source-span/Semantic ID projection, and bilingual diagnostics; and
5. protocol-inventory updates plus positive, negative, empty, cross-package,
   deterministic, Unicode/CRLF, offline, and migration fixtures.

Until a later decision closes the project-test boundary, only DEC-0039's
explicit standalone-file runner is authorized; it intentionally does not
invent language syntax or silently establish manifest/workspace test behavior.

## Evidence and compatibility

This audit was checked against `docs/RFC-0002.md`,
`docs/decisions/0003-m0-tooling.md`, `docs/decisions/0013-main-and-runtime-failures.md`,
`docs/decisions/0039-cli-test-file-runner.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/governance/protocol-inventory.toml`, `docs/governance/gap-register.toml`,
`crates/ling-cli/src/main.rs`, `crates/ling-cli/tests/conformance.rs`,
`crates/ling-project`, and `tests/conformance`.
The child adds only the accepted Preview command/report and `L-IO-0004` /
`L-TEST-0001`; no Semantic ID, source-span model, bytecode, VM, or Unicode
17.0.0 behavior changes.

## Intentionally deferred

The parent `CLI-1704` can continue after an accepted project-test convention
defines manifest/workspace selection, isolation, events, filtering, and
assertions. The completed child reuses checked compiler services, remains
offline/deterministic, and excludes stale names and unregistered annotations.
