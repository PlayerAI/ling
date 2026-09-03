# REP-2502 Implementation Report

## Outcome

REP-2502 is complete only for the private Experimental baseline authorized by
Accepted DEC-0280. Implementation commit
`ca1fcce6f435fb8419acc97fe813f3a773d5df8b` adds one crate-private,
`cfg(test)` evidence matrix to `ling-eval`. It executes exactly five bounded
case families over real validated DEC-0267 `TaskScheduleTrace` values.

This completion does not define a public Effect Log or Replay schema. It adds
no production model, encoder, decoder, checksum, privacy or migration rule,
diagnostic, CLI command, fixture, implemented protocol, or compatibility
promise. `GAP-DETERMINISTIC-REPLAY-001` remains Open.

## Normative clauses covered

- DEC-0280 clauses 1-3 and 14: `replay_structure_evidence.rs` is test-only,
  consumes existing checked Task routes, and leaves production runtime and
  public surfaces unchanged. `TaskScheduleTrace` remains an in-process test
  boundary rather than the `EffectLog` sketched by `SEMANTICS.md`.
- Clauses 4-5: the matrix contains the exact thirteen DEC-0105 concern labels
  once each. Seven concerns map to existing private trace evidence and six
  remain explicitly deferred; no label becomes a field, tag, ordinal, JSON
  key, or schema registry entry.
- Clause 6: exactly five named case families are registered, duplicate checked,
  and executed. No additional record meaning or case family is introduced.
- Clause 7: successful and host-Fault traces are produced from immutable
  Checked Core with exact arguments, explicit runtime and scheduler limits,
  one logical deadline, and deterministic host scripts. Repetition and
  Unicode/BOM/CRLF reconstruction validate and produce identical private
  canonical bytes without retaining physical source identity.
- Clause 8: the event projection checks consecutive IDs, monotonic logical
  ticks, canonical ready sets and deadline paths, every existing private event
  kind, and exactly one final closure. It makes no cross-process, Actor,
  external-Effect, or production-worker ordering claim.
- Clause 9: bounded typed `TaskValue::Text`, host success/failure outcomes,
  completed/faulted terminals, one retained Fault summary, and exactly-once
  cleanup are exercised. Source names and original UTF-8 spans remain sidecar
  evidence and do not affect canonical equality.
- Clause 10: the rejection case directly executes complete DEC-0267 validation
  and replay-mutation assertions for version, identity, ordering/closure,
  selection, step, tick, deadline, host, and terminal changes. Each of the
  seven scheduler and four runtime zero bounds fails before a run.
- Clauses 11-13: canonical-byte equality is never presented as a checksum or
  authenticity property. All collections and limits are finite, and negative
  evidence checks evaluator, effects, semantic/build, bytecode, VM, source,
  CLI, diagnostics, schema registry, and the unchanged Future
  `PROTO-REPLAY` record.
- Clauses 15-16: focused and repository-wide gates pass and evidence is bound
  to the implementation commit above. Every public schema, integrity,
  privacy, migration, corruption, divergence, resource, and cross-process
  decision remains deferred.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `validated-private-envelope-projection`
2. `event-identity-kind-order-projection`
3. `typed-payload-terminal-projection`
4. `mutation-and-limit-rejection`
5. `public-replay-schema-absence`

The complete concern disposition is:

- existing private trace evidence: `canonical-envelope`, `event-id`,
  `event-kind`, `ordering`, `identity`, `schema`, and `payload`;
- deferred public contract: `checksum`, `determinism-class`, `toolchain`,
  `profile`, `migration`, and `privacy`.

Here `schema` means only the existing private DEC-0267 trace-version check.
The canonical bytes are bounded in-memory equality evidence, not a persisted
wire format, checksum, signature, corruption detector, or migration revision.

## Executed verification

Commands executed locally on 2026-09-03:

- `cargo test -p ling-eval --lib replay_structure_evidence` — passed: all five
  DEC-0280 cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 71 unit,
  12 Actor runtime, 13 local scheduler, 20 Task runtime, and 14 Task scheduler
  tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test -p ling-cli --test task_boundary --locked --offline` — passed:
  4 tests.
- `cargo test -p ling-cli --test actor_boundary --locked --offline` — passed:
  10 tests; public Actor execution remains rejected.
- `cargo test --workspace --all-targets --locked --offline` — passed after the
  Accepted-document count was synchronized.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo clippy -p xtask --all-targets --locked --offline -- -D warnings` —
  passed.
- `cargo xtask governance check-all`, `cargo xtask docs verify`, and
  `cargo xtask rc0 verify` — passed.
- `cargo fmt --all -- --check` and `git diff --check` — passed.

## Compatibility impact

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, Semantic IDs,
  protocols, package/ABI versions, stored data, dependencies, and migration:
  unchanged.
- Production runtime: unchanged. One existing `cfg(test)` validation assertion
  becomes visible to its sibling test module; no production symbol or runtime
  transition is added.
- Replay and determinism: no public event identity, ordering, encoding,
  checksum, privacy, compatibility, or equivalence promise is created.
  `PROTO-REPLAY` remains Future, unversioned, schema-less, and unimplemented.
- Unicode remains 17.0.0. Original UTF-8 byte spans remain authoritative and
  are explicitly kept outside logical trace equality.

## Specification gaps and deferred work

No conflict was found inside DEC-0280's private evidence contract.
`GAP-DETERMINISTIC-REPLAY-001` remains Open because the execution plan's public
Replay Log Schema still lacks Accepted RFC-0010 or replacement authority.

Public envelope encoding, types/tags/optionality, event and logical-time
identity across Effect/Task/Actor boundaries, payload framing, checksum and
integrity scope, determinism/toolchain/profile metadata, privacy/redaction and
retention, corruption/divergence behavior, limits, reader/writer compatibility,
unknown-field handling, migration, fixtures, cross-process/backend behavior,
and Stable support remain intentionally deferred. REP-2503 is the next
sequential task and requires separate Accepted authority before implementation.
