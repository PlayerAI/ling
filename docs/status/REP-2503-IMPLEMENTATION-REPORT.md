# REP-2503 Implementation Report

## Outcome

REP-2503 is complete only for the private Experimental baseline authorized by
Accepted DEC-0281. Implementation commit
`8a0f05b9a38f922c73339d4111e6927ac9cb693b` adds one crate-private,
`cfg(test)` evidence module to `ling-eval`. It executes exactly five bounded
case families through the real checked interpreter and observes only
`Console.Write.write` operations that escape accepted lexical Handler dispatch
and reach an injected test adapter.

This completion does not define a production Effect recorder, public
recordability policy, Effect Log, Replay payload, privacy rule, or compatibility
contract. `GAP-DETERMINISTIC-REPLAY-001` remains Open and `PROTO-REPLAY`
remains Future, unversioned, schema-less, and unimplemented.

## Normative clauses covered

- DEC-0281 clauses 1-3 and 16: `effect_recording_evidence.rs` is test-only and
  compiles every source through Source, CST, AST, HIR, resolution, type
  checking, Effect/Capability checking, and `ProgramSnapshot` before executing
  the real checked interpreter. No production transition, public API, VM hook,
  or unchecked-AST route is added.
- Clauses 4-6: the adapter records exactly one bounded observation only after a
  checked operation escapes all applicable lexical handlers. Each observation
  contains a consecutive ordinal, constant operation identity, canonical-LF
  host text, and structured success or `HostErrorCategory` failure. A failing
  invocation stops later source operations, while the Runtime Fault remains
  sidecar evidence.
- Clauses 7-10: exactly five case families are registered and duplicate
  checked. They exercise ordered escaped success around pure computation,
  direct/transitive/resumed/nested Handler elision, clause escape, one success
  followed by `BrokenPipe`, absence of a post-failure invocation, and the
  accepted `L-RUNTIME-0001` projection at the original operation span.
- Clause 11: equivalent Unicode source reconstructed across LF and BOM/CRLF,
  different logical source names, and different `SourceId` values produces the
  same logical observation. Runtime Fault source names and original UTF-8 byte
  spans remain distinct sidecars and do not enter equality.
- Clause 12: all six DEC-0106 provisional boundaries occur exactly once.
  Clock/Random are classified as checked contracts without producers;
  ExternalInput/NetworkReceive/FileDeviceRead remain plan-only; scheduling
  nondeterminism remains separate private runtime evidence. None is aliased to
  the Console probe.
- Clauses 13-15: all fixtures are finite repository literals with explicit host
  outcomes. Negative assertions cover production evaluator/effects/semantic/
  project/bytecode/VM surfaces, CLI commands, diagnostics, schema registry, and
  the unchanged Future `PROTO-REPLAY` record.
- Clauses 17-18: focused, differential, workspace, governance, status,
  documentation, RC0, formatting, and lint gates pass. The evidence is bound to
  the implementation commit above; every public recorder, payload, privacy,
  migration, cross-process, backend, and Stable-support decision remains
  deferred.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `escaped-success-order`
2. `handled-elision-and-clause-escape`
3. `failure-stop-and-fault-sidecar`
4. `checked-reconstruction-and-source-independence`
5. `deferred-boundaries-and-public-surface-absence`

The private observation is deliberately smaller than a future Effect Log. It
stores no source identity or span, time, worker, address, allocation, retry,
checksum, profile, toolchain, redaction, or migration data and is never
serialized, hashed, persisted, or exported.

## Executed verification

Commands executed locally on 2026-09-03:

- `cargo test -p ling-eval --lib effect_recording_evidence --locked --offline`
  — passed: all five DEC-0281 cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 76 unit,
  12 Actor runtime, 13 local scheduler, 20 Task runtime, and 14 Task scheduler
  tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test -p ling-vm --test execution handler --locked --offline` — passed:
  5 Handler/continuation tests.
- `cargo test -p ling-vm --test differential --locked --offline` — passed: 4
  checked-interpreter/VM differential tests.
- `cargo test --workspace --all-targets --locked --offline` — passed after the
  Accepted-document and lifecycle counts were synchronized.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` —
  passed.
- `cargo xtask governance check-all`, `cargo xtask status verify`,
  `cargo xtask docs verify`, and `cargo xtask rc0 verify` — passed.
- `cargo fmt --all -- --check` and `git diff --check` — passed.

## Compatibility impact

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, Semantic IDs,
  protocols, package/ABI versions, stored data, dependencies, and migration:
  unchanged.
- Production interpreter, bytecode, VM, Task, Actor, scheduler, mailbox, and
  host behavior: unchanged. The new module is compiled only under `cfg(test)`
  and wraps the existing injected Console test boundary.
- Replay and determinism: no public recordable-operation set, event identity,
  payload, ordering, encoding, checksum, privacy, compatibility, or equivalence
  promise is created.
- Unicode remains 17.0.0. Canonical LF host text is compared privately, while
  original UTF-8 byte spans remain authoritative sidecar evidence.

## Specification gaps and deferred work

No conflict was found inside DEC-0281's private evidence contract.
`GAP-DETERMINISTIC-REPLAY-001` remains Open because no Accepted RFC-0010 or
replacement defines the public recorder and Replay contract.

Public recordability; Clock/Random/external-input/network/file/device producers;
Task/Actor scheduler capture; event identity and payload/result/Fault encoding;
buffering, flush, backpressure, limits, privacy, redaction, retention, integrity,
corruption and divergence behavior; diagnostics; reader/writer compatibility;
migration; checkpoints; cross-process/backend equivalence; and Stable support
remain intentionally deferred. REP-2504 is the next sequential task and
requires separate Accepted authority before implementation.
