# LSP-2502 Implementation Report: Request Cancellation

## Status

Implemented under Accepted RFC-0049. The previous internal child
`LSP-2502-CANCELLATION` remains the DEC-0031 token foundation; this parent
milestone adds the complete bounded Preview wire, compiler, and publication
boundary.

## Normative clauses covered

- RFC-0049 §1: initialize advertises exact
  `ling.lsp.request-cancellation/0.1` discovery.
- RFC-0049 §2–§4: the stdio reader registers exact live string/number IDs,
  signals valid `$/cancelRequest` notifications before ordered dispatch,
  rejects duplicate-live IDs, prevents late cancellation from crossing reuse,
  and retains one single-threaded mutable server executor.
- RFC-0049 §5: request tokens reach workspace symbols, rename, completion,
  completion resolve, and semantic tokens. `ling-db` exposes typed cancellable
  query entry points; `ling-types` checks between definitions and inside the
  bounded Trait obligation solver.
- RFC-0049 §6: handlers check cancellation before final success and before
  completion-resolve, workspace-index, or semantic-token-history mutation;
  rename builds its versioned Workspace Edit privately and returns no partial
  edit. Observed cancellation returns bilingual `-32800`.
- RFC-0049 §7: registry cleanup is identity-safe and session-local; no request
  ID or cancellation state enters compiler caches or result identities.

## Implementation

- `crates/ling-lsp/src/request_cancellation.rs` owns exact request keys, the
  live registry, duplicate detection, cancellation signalling, identity-safe
  cleanup, and focused lifecycle tests.
- `crates/ling-lsp/src/lib.rs` separates bounded frame reading from the single
  mutable executor with a standard channel, routes every accepted request with
  its token, handles `$/cancelRequest`, and advertises exact discovery.
- Rename and completion pipelines check their bounded planning, simulation,
  candidate-validation, projection, freshness, response-size, and publication
  boundaries. Existing workspace-symbol and semantic-token atomic caches now
  consume compiler-aware cancellation.
- `ling-db::QueryError::Cancelled` and cancellable definition/reference/alias,
  completion, metadata, and semantic-token queries check before and after
  compiler stages and before cache insertion.
- `ling_types::check_with_cancellation` and the Trait obligation solver expose
  typed cancellation without publishing partial Typed Core.
- The CLI passes a Send-capable stdout owner to the cancellation-aware stdio
  executor without changing the `ling lsp --stdio` command or stdout purity.

## Executable evidence

- `tests/protocols/lsp-request-cancellation/fixtures/v1.json` freezes discovery,
  notification response-freedom, malformed handling, and request-form error
  bytes; `crates/ling-lsp/tests/cancellation.rs` is its independent reader.
- The exact diagnostic transcript corpus records the additive initialize
  capability, and its manifest binds those successful sessions to the new
  protocol marker.
- A deterministic framed transcript blocks the executor's first response until
  the reader has registered and cancelled a later live workspace-symbol ID;
  the request returns `-32800` and the notification emits no response.
- Registry unit tests cover clone sharing, exact string/number separation,
  duplicate-live rejection, unknown/duplicate/late behavior, cleanup, and safe
  ID reuse.
- Rename and completion tests prove pre-cancelled requests publish no partial
  Workspace Edit or result and that later independent requests succeed.
- Workspace-symbol and semantic-token suites retain their cancellation and
  atomic cache/history assertions.
- `ling-db` proves typed cancellation leaves no checked-program cache entry;
  `ling-types` proves cancellation is observed between definitions, and solver
  checkpoints are wired through the same typed path.

Focused commands executed successfully before the full repository gate:

```text
cargo test -p ling-lsp --test cancellation --test completion --test rename --test workspace_symbols --test semantic_tokens --locked --offline
cargo test -p ling-db --lib --locked --offline
cargo test -p ling-types --lib --locked --offline
cargo clippy -p ling-lsp -p ling-db -p ling-types --all-targets --locked --offline -- -D warnings
cargo xtask governance check-all
cargo xtask support verify
```

The following repository-wide gates were then executed successfully after the
exact transcript and governance-count evidence were brought current:

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

- **Protocol:** adds standard `$/cancelRequest`, `-32800`, duplicate-live-ID
  rejection, and Preview `ling.lsp.request-cancellation/0.1` discovery.
- **Compiler:** adds typed cooperative cancellation but does not change any
  successful query key, checked fact, cache value, or source span.
- **Diagnostics/schema/identity:** no Ling error code, standalone schema,
  Semantic ID, Definition ID, or canonical compiler bytes change.
- **Language/runtime/Unicode:** no language, Typed Core evaluation, runtime,
  bytecode, VM, ABI, package, filesystem/network, or Unicode 17.0.0 change.
- **Determinism/privacy:** only exact input order and token state affect
  cancellation; timing, thread identity, allocation, paths, source text, and
  compiler identities do not enter the wire contract.

## Intentionally deferred

Deadlines, quotas, general priority/fairness, progress, server-initiated
cancellation, parallel compiler mutation, non-stdio transports, Stable editor
compatibility, and general Semantic Transactions remain outside RFC-0049 and
belong to later tasks.
