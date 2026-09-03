# REP-2504 Implementation Report

## Outcome

REP-2504 is complete only for the private Experimental baseline authorized by
Accepted DEC-0282. Implementation commit
`c3e05bac815e55ace2bef58779094869f342b237` adds one crate-private,
`cfg(test)` evidence module to `ling-eval` and strengthens the existing private
Task replay recipe identity so changed checked Task bodies are rejected during
preflight.

This completion does not define a production or public Replay Player,
checkpoint, persisted log reader, Effect Log, CLI command, schema, diagnostic,
privacy/integrity policy, migration rule, or cross-process guarantee.
`GAP-DETERMINISTIC-REPLAY-001` remains Open and `PROTO-REPLAY` remains Future,
unversioned, schema-less, and unimplemented.

## Normative clauses covered

- DEC-0282 clauses 1-3 and 16: `replay_player_execution_evidence.rs` is
  test-only and compiles every fixture through Source, CST, AST, HIR,
  resolution, type checking, and Effect/Capability checking. It exercises the
  existing DEC-0267 in-memory Task trace and fresh-runtime replay path without
  adding a public player, reader, restore hook, or unchecked-AST route.
- Clauses 4, 8, and 13: structural trace validation remains first. The private
  recipe identity now includes each reachable Task's accepted DEC-0012 Body ID
  and advances from `ling.task-runtime-recipe/0` to
  `ling.task-runtime-recipe/1`. Changed Task behavior, root, or arguments fail
  at private event `0` before scheduling or host observation.
- Clauses 5 and 7: equivalent checked programs reconstructed across logical
  source-name and `SourceId` changes and LF versus BOM/CRLF spelling replay to
  identical private canonical trace bytes. Physical paths and source spelling
  do not enter logical equality; original UTF-8 spans remain Fault sidecars.
- Clauses 6 and 9: exactly five case families are registered and duplicate
  checked. Complete existing mutation assertions cover structural rejection
  plus the first private divergence for selection, step, tick, ready set,
  deadline, host, and terminal fields.
- Clause 10: bounded `BrokenPipe` host-Fault and deadline-cancellation traces
  replay from fresh checked runtimes with identical terminal, cleanup, Fault,
  host-event, and canonical-trace evidence.
- Clauses 11-15: all eleven DEC-0107 concerns occur exactly once. Seven map
  only to existing private Task replay evidence; checkpoint identity, privacy,
  integrity, and migration remain deferred. Negative assertions cover public
  player/checkpoint/reader APIs, CLI routes, diagnostics, schemas, and the
  unchanged Future `PROTO-REPLAY` record.
- Clauses 17-18: focused, retained differential, full workspace, governance,
  status, documentation, RC0, formatting, and lint gates pass. Public Replay
  semantics remain blocked pending Accepted RFC-0010 or replacement authority.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `validated-trace-exact-replay`
2. `checked-recipe-preflight`
3. `first-event-divergence`
4. `fault-and-cancellation-replay`
5. `deferred-checkpoint-and-public-surface-absence`

The DEC-0107 disposition inventory is complete and duplicate-free:

- existing private Task replay evidence: `program-canonical-bytes`,
  `preflight-binding`, `event-application`, `ordering`, `divergence`, `fault`,
  and `cancellation`;
- deferred public contract: `checkpoint-identity`, `privacy`, `integrity`, and
  `migration`.

Here `event-application` means only fresh-runtime re-execution under recorded
private choices. It does not mean checkpoint restoration or decoding an
external Effect Log.

## Implementation finding

The first focused run passed four cases but exposed one existing strict-
preflight gap: changing a reachable Task literal from `+ 1` to `+ 2` retained
the previous private recipe identity and diverged only at event `7`. The
existing structural Task Core/machine canonical bytes were therefore
insufficient to bind all checked expression semantics.

The implementation now builds the accepted semantic snapshot and incorporates
the DEC-0012 Body ID for every reachable Task into the recipe identity. The
same mutation is consequently rejected at event `0`, as DEC-0282 requires.
The domain bump to `/1` prevents ambiguity with the earlier private identity.

## Executed verification

Commands executed locally on 2026-09-03:

- `cargo test -p ling-eval --lib replay_player_execution_evidence --locked --offline`
  — passed: all five DEC-0282 cases after the preflight fix.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 81
  library tests, 12 Actor-runtime tests, 13 local-scheduler tests, 20 Task-
  runtime tests, and 14 Task-scheduler tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test -p ling-vm --test execution handler --locked --offline` — passed:
  5 Handler/continuation tests.
- `cargo test -p ling-vm --test differential --locked --offline` — passed: 4
  checked-interpreter/VM differential tests.
- `cargo test -p ling-cli --test task_boundary --locked --offline` — passed: 4
  Task CLI boundary tests.
- `cargo test -p ling-cli --test actor_boundary --locked --offline` — passed:
  10 Actor CLI boundary tests.
- `cargo test --workspace --all-targets --locked --offline` — passed.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` —
  passed.
- `cargo xtask governance check-all`, `cargo xtask status verify`,
  `cargo xtask docs verify`, and `cargo xtask rc0 verify` — passed.
- `cargo fmt --all -- --check` and `git diff --check` — passed.

## Compatibility impact

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, public Semantic
  IDs, protocols, package/ABI versions, stored data, dependencies, bytecode,
  VM, backends, and migration: unchanged.
- The publish-disabled private test scheduler's opaque recipe identity advances
  from `/0` to `/1` and now includes accepted Task Body IDs. No public reader,
  persistence format, or migration obligation exists for the former in-memory
  identity.
- No production replay transition, player/checkpoint/log-reader type, restore
  path, decoder, or public API is added.
- Unicode remains 17.0.0. Source-independent logical identity excludes physical
  paths and source spelling, while original UTF-8 spans remain authoritative
  Fault sidecar evidence.

## Specification gaps and deferred work

No conflict was found inside DEC-0282's private evidence contract. The focused
test exposed and the implementation corrected the private recipe-identity gap
described above.

Checkpoint contents and identity; public Program/schema/profile/target/
toolchain/Capability/config/message-schema binding; encoded log input; event
application semantics; privacy, integrity, corruption, authorization,
diagnostics, resource policy, reader/writer compatibility, migration, partial
replay, cross-process/backend behavior, and Stable support remain intentionally
deferred. REP-2505 requires separate Accepted authority before implementation.
