# CLI-1704 Authority Audit: Project test command

## Outcome

`CLI-1704` is correctly recorded as `BlockedSpec`. The execution plan
proposes a `test` command, but the language has no accepted test declaration or
discovery convention, and no project command contract defines selection,
fixtures, isolation, output events, or exit behavior. The repository's Rust
and conformance tests are implementation evidence, not a Ling user test
protocol.

No `ling test` parser branch, annotation syntax, test discovery rule, event
schema, test runner API, or placeholder help entry was added. The current CLI
continues to advertise only implemented commands.

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
  policy, exits, and JSON output open. No protocol inventory entry authorizes a
  `ling.test` surface.
- The execution plan's stale `zero test` spelling and any proposed annotations
  cannot enter implementation without a higher-authority decision.

## Current interface evidence

The repository confirms the missing boundary:

- `crates/ling-cli/src/main.rs` has no `test` command and its help output does
  not advertise one.
- `crates/ling-cli/tests/conformance.rs` runs fixed repository fixtures through
  Rust's test harness; it has no user test discovery or project event schema.
- `tests/conformance/` contains compiler/CLI acceptance fixtures, not a
  versioned source-level test declaration or expected test ordering contract.
- `ling-project` provides graph/lock validation but no test target inventory,
  package test isolation, or failure aggregation protocol.

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

Until those decisions and fixtures are Accepted, a `test` command would invent
language syntax or silently establish a test convention that later projects
could not change compatibly.

## Evidence and compatibility

This audit was checked against `docs/RFC-0002.md`,
`docs/decisions/0003-m0-tooling.md`, `docs/decisions/0013-main-and-runtime-failures.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`,
`docs/governance/protocol-inventory.toml`, `docs/governance/gap-register.toml`,
`crates/ling-cli/src/main.rs`, `crates/ling-cli/tests/conformance.rs`,
`crates/ling-project`, and `tests/conformance`.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`CLI-1704` can begin after an accepted test convention and project CLI
contract define discovery, isolation, events, and exits. The implementation
must reuse checked compiler services, remain offline/deterministic, and exclude
stale `zero` syntax and unregistered annotations.
