# REP-2505 Implementation Report

## Outcome

REP-2505 is complete only for the private Experimental baseline authorized by
Accepted DEC-0283. Implementation commit
`b31bd278df5f3482052c463186063b03e13329c9` adds one crate-private,
`cfg(test)` evidence module to `ling-eval` plus a complete test-only scheduler
assertion for strict-prefix and event-gap rejection.

This package does not define a production or public privacy policy, redactor,
trimmer, chunk/checksum format, retained log, key manager, decoder, corruption
recovery path, offline Replay command, schema, diagnostic, migration rule, or
cross-process guarantee. `GAP-DETERMINISTIC-REPLAY-001` remains Open and
`PROTO-REPLAY` remains Future, unversioned, schema-less, and unimplemented.

## Normative clauses covered

- DEC-0283 clauses 1-3 and 16: `replay_privacy_execution_evidence.rs` is
  test-only and compiles every fixture through Source, CST, AST, HIR,
  resolution, type checking, and Effect/Capability checking. It exercises only
  validated DEC-0267 in-memory traces and DEC-0282 strict fresh-runtime replay.
- Clauses 4-6: one repository-local Unicode sentinel appears unchanged in the
  typed Host event and private canonical trace bytes, while the logical source
  name is absent. This demonstrates raw retention and explicitly disproves any
  redaction or safe-persistence claim.
- Clauses 7 and 11: a complete scheduler assertion rejects every strict event
  prefix with an exact structural reason and rejects every non-closure event
  removal as a nonconsecutive event sequence. No dependency-preserving trim or
  partial replay is inferred.
- Clauses 8, 12, and 13: complete existing mutation assertions cover version,
  identity, closure, ready set, selected Task, step, tick, deadline, Host, and
  terminal changes. Validation/replay errors remain private evidence rather
  than checksums, authentication, recovery, or public diagnostics.
- Clause 9: equivalent checked programs reconstructed across source-name,
  `SourceId`, LF, and BOM/CRLF differences replay identically using explicit
  finite arguments, seed, limits, deadlines, and injected Host responses.
- Clauses 10, 14, and 15: all sixteen DEC-0108 concerns occur exactly once with
  bounded dispositions. Negative assertions cover public privacy/redaction/
  trimming/chunk/checksum/retention/key/decoder APIs, CLI routes, diagnostics,
  schemas, and the unchanged Future `PROTO-REPLAY` record.
- Clauses 17-18: focused, retained differential, full workspace, governance,
  status, documentation, RC0, formatting, and lint gates pass. All public
  privacy/integrity/data-tooling semantics remain deferred.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `raw-payload-retention-boundary`
2. `prefix-truncation-refusal`
3. `validated-mutation-refusal`
4. `explicit-input-offline-reconstruction`
5. `deferred-privacy-integrity-surface-absence`

The DEC-0108 disposition inventory is complete and duplicate-free:

- raw-payload risk evidence: `field-sensitivity` only, without defining a
  sensitivity taxonomy;
- private fail-closed evidence: `truncation`, `corruption`, and private
  `failure-diagnostics`;
- explicit-input hermetic test evidence: `offline-mode`;
- deferred public contracts: `field-redaction`, `secret-pii-exclusion`,
  `capability-resource-exclusion`, `authorization`, `key-handling`,
  `retention`, `dependency-closure`, `chunk-boundary`, `checksum-integrity`,
  `unknown-field`, and `migration`.

## Executed verification

Commands executed locally on 2026-09-04:

- `cargo test -p ling-eval --lib replay_privacy_execution_evidence --locked --offline`
  — passed: all five DEC-0283 cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 87
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

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, Semantic IDs,
  protocols, package/ABI versions, stored data, dependencies, bytecode, VM,
  backends, and migration: unchanged.
- Production Task/Actor scheduling and replay behavior: unchanged. The only
  scheduler addition is a complete `cfg(test)` prefix/gap rejection assertion.
- The evidence deliberately confirms that private trace payloads are raw. No
  privacy, redaction, integrity, authenticity, trimming, persistence, offline-
  tool, or compatibility guarantee is created.
- Unicode remains 17.0.0, and original UTF-8 spans remain authoritative
  sidecar evidence.

## Deferred work

Sensitivity taxonomy; default allow/deny; secret/PII/Capability/Resource
handling; redaction representation; authorization; encryption and keys;
retention/deletion; dependency-preserving trimming; chunk framing;
checksum/integrity/authenticity; corruption recovery; unknown fields; public
diagnostics; resource policy; reader/writer compatibility; migration; public
offline tools; cross-process/backend behavior; and Stable support remain
intentionally deferred. REP-2506 requires separate Accepted authority.
