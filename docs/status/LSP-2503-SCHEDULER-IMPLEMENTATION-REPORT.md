# LSP-2503 Implementation Report: Deterministic Scheduling

## Status

Implemented under Accepted RFC-0050. The earlier DEC-0032 child remains the
strict internal queue foundation; this parent milestone adds bounded fairness,
public Preview discovery, compiler-aware diagnostic supersession, and concrete
server service-order integration.

## Normative clauses covered

- RFC-0050 §1: exact `ling.lsp.scheduling/0.1` initialize discovery.
- RFC-0050 §2–§3: explicit method classes, wire-order state/request execution,
  response ordering, and Analysis-before-Background service.
- RFC-0050 §4: deterministic message-boundary debounce, mutation-driven token
  rotation, cancellable diagnostic stages, stale rejection, and current-only
  atomic ledger publication.
- RFC-0050 §5: FIFO within class plus fixed 8-Interactive and
  16-non-Background fairness bounds.
- RFC-0050 §6: typed sequence/cancellation failures and non-public scheduling
  state.

## Implementation

- `scheduler.rs` retains DEC-0032 strict `pop` and adds `pop_fair`, explicit
  method classification, fixed bounds, and the Preview marker. Selection uses
  only class and monotonic local sequence.
- The stdio executor classifies each body without changing wire order. Pending
  push-diagnostic Analysis is flushed before Background work; ordinary
  response-before-caused-notification behavior remains unchanged.
- Each successful mutation cancels the prior diagnostic token and installs a
  fresh token. A ticket checks before and during source traversal, around the
  compiler, after adaptation, and before publication.
- `CompilerDb::workspace_diagnostics_with_cancellation` checks between sources,
  parse/module stages, cancellable type/Trait work, and final complete result.
  Existing `workspace_diagnostics` delegates to a never-cancelled probe.

## Executable evidence

- Scheduler unit tests cover strict priority/FIFO, duplicate IDs, clear and
  monotonic sequence behavior, method classification, and both fairness bounds.
- `scheduling.rs` consumes the exact fixture and verifies initialize discovery.
- Push-diagnostic tests prove multi-mutation coalescing, cancellation of an old
  ticket before compilation/publication, stale completed-result rejection,
  preservation of newer pending work, and atomic current publication.
- Compiler DB tests prove cancellation yields typed `QueryError::Cancelled`,
  returns no partial diagnostic collection, publishes no partial checked cache,
  and permits a later independent successful query.
- Exact diagnostic transcripts include the additive scheduling discovery and
  remain repeat-byte deterministic.

Focused commands executed successfully during implementation:

```text
cargo check -p ling-db -p ling-lsp --all-targets --locked --offline
cargo test -p ling-lsp --lib --test push_diagnostics --test scheduling --locked --offline --quiet
cargo test -p ling-db --lib --locked --offline --quiet
cargo xtask governance check-all
```

The final repository-wide gate set also passed:

```text
cargo test --workspace --all-targets --locked --offline --quiet
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask ci verify
cargo xtask governance check-all
cargo xtask lsp verify
cargo xtask support verify
cargo xtask status verify
cargo xtask rc0 verify
cargo xtask traceability verify --release v0.0.1
cargo fmt --all -- --check
git diff --check
manual SHA-256 verification of docs/ling_execution_plan/SHA256SUMS.txt
```

## Compatibility and determinism

- **Protocol:** adds Preview `ling.lsp.scheduling/0.1`; no new method or client
  configuration.
- **Compiler:** adds cooperative complete-diagnostic cancellation without
  changing successful facts, keys, spans, cache values, or output order.
- **Diagnostics/schema/identity:** no Ling code, diagnostic shape, standalone
  schema, Semantic ID, Definition ID, or canonical byte changes.
- **Language/runtime/Unicode:** no syntax, semantics, Typed Core evaluation,
  interpreter, runtime, bytecode, VM, ABI, package, host I/O, or Unicode 17.0.0
  change.
- **Determinism/privacy:** selection never observes a clock, CPU/load, thread,
  allocation, map iteration, path, source text, or persistent sequence.

## Intentionally deferred

Wall-clock/configurable debounce, deadlines, dynamic priorities, host-load
adaptation, worker pools, parallel mutable requests, response reordering,
progress, partial results, quotas, persistent scheduling/indexes, Stable
lifecycle, and Semantic Transactions remain outside RFC-0050.
