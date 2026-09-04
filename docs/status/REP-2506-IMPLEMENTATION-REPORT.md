# REP-2506 Implementation Report

## Outcome

REP-2506 is complete only for the private Experimental same-binary
reconstruction baseline authorized by Accepted DEC-0284. Implementation commit
`e5f16355e02abb76680f6984427207ad96ae7b0a` adds one crate-private,
`cfg(test)` evidence module to `ling-eval`. Parent tests start fresh copies of
the current unit-test executable with an empty inherited environment; child
probes rebuild checked Task traces from fixed in-memory inputs and return exact
private canonical bytes to the parent test.

This package is not persisted-log playback, a generator/player protocol,
Program/Schema mutation refusal, cross-toolchain or cross-platform
certification, a public process harness, or a Stable Replay contract.
`GAP-DETERMINISTIC-REPLAY-001` remains Open and `PROTO-REPLAY` remains Future,
unversioned, schema-less, and unimplemented.

## Normative clauses covered

- DEC-0284 clauses 1-5 and 12: the new module is test-only; every child follows
  `SourceFile → parse → AST → HIR → resolve → typecheck → Effect check` and
  schedules only checked Task Core. The hexadecimal stdout marker transports
  existing private trace bytes only between test processes.
- Clauses 2-3 and 9: every probe is a fresh copy of the current test executable,
  clears its inherited environment, reports zero observed entries, uses fixed
  in-memory source/configuration/arguments and an empty host script, and runs
  under explicit finite scheduler bounds.
- Clause 6: three independently started LF probes produce identical complete
  private trace bytes and environment counts.
- Clause 7: independently started LF and UTF-8 BOM plus CRLF probes use distinct
  source IDs/names but reconstruct identical complete private trace bytes.
- Clause 8: changing the checked child Task body or the root Task argument
  produces three pairwise-distinct private traces. This is concrete recipe
  distinction, not serialized mutation rejection.
- Clauses 10-11: all eighteen DEC-0109 concerns occur exactly once with exact
  dispositions. Negative assertions retain the absence of production process
  acceptance APIs, a Replay CLI route, `L-REPLAY-*` diagnostics, a Replay
  schema, and an implemented `PROTO-REPLAY` record.

## Exact evidence matrix

The dedicated parent matrix contains exactly these cases:

1. `independent-process-repeatability`
2. `source-independent-process-equivalence`
3. `changed-recipe-process-distinction`
4. `empty-environment-bounded-process`
5. `deferred-cross-process-public-surface-absence`

Four ignored child probes are invoked only by the parent matrix through fresh
processes. The DEC-0109 concern inventory is complete and duplicate-free:

- same-binary child evidence: process isolation, toolchain identity, and target
  identity;
- private trace-comparison evidence: cache isolation, input snapshot, log
  generation, Program binding, observable equivalence, repeatability,
  divergence, resource limits, and offline mode;
- deferred public contracts: profile identity, Replay playback, Schema binding,
  mutation rejection, provenance, and platform boundary.

These labels retain DEC-0284's narrow definitions: toolchain/target identity
means one compiled executable, cache isolation means the probe accepts no cache
input, log generation means only the existing in-memory private Task trace, and
Program binding means only the existing private checked-recipe preflight.

## Executed verification

Commands executed locally on 2026-09-04:

- `cargo test -p ling-eval --lib replay_cross_process_execution_evidence --locked --offline`
  — passed: 5 parent cases; 4 child probes intentionally ignored by the outer
  harness and invoked through fresh child processes by the parent cases.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test --workspace --all-targets --locked --offline` — passed; the
  `ling-eval` library portion passed 92 tests with the same 4 intentional child
  probe ignores.
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
- Production Task/Actor scheduling and replay behavior: unchanged. All new
  process execution and stdout markers are contained in `cfg(test)` code.
- The result proves only bounded same-binary exact-byte reconstruction. It does
  not prove clean build caches, cross-build identity, public observable
  equivalence, Replay playback, or Program/Schema mismatch refusal.
- Unicode remains 17.0.0, and original UTF-8 spans remain authoritative sidecar
  evidence.

## Deferred work

Public generator/player and persisted log formats; Program, Semantic ID, Schema,
capability, configuration, and message bindings; mutation/corruption refusal;
privacy and integrity; reader/writer compatibility and migration; divergence
diagnostics; observable-equivalence rules; compiler/toolchain/profile/target and
dependency provenance; clean-cache tooling; environment fingerprints; signed
artifacts; cross-backend and cross-platform matrices; CI artifact schemas;
resource/security guarantees; and Stable support remain intentionally deferred
to RFC-0010 or replacement Accepted authority.
