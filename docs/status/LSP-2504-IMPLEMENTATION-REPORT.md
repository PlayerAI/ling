# LSP-2504 Implementation Report: Bounded Resources

## Status

Implemented under Accepted RFC-0051. Accepted DEC-0033 remains the checked
UTF-8 arithmetic foundation; this parent milestone connects it to open-overlay
accounting, adds bounded live request admission, composes existing bounded
results and Trait solver nesting, and exposes one exact Preview protocol.

## Normative clauses covered

- RFC-0051 §1: exact `ling.lsp.resource-limits/0.1` discovery and fixed hard
  limits.
- RFC-0051 §2: exact decoded UTF-8 bytes, per-document and 8 MiB aggregate
  open-overlay accounting, atomic growth/shrink, and close cleanup.
- RFC-0051 §3: 128 queued/executing live request IDs, duplicate-first
  precedence, rejection without association, and completion cleanup.
- RFC-0051 §4: composition of existing 256 completion items, diagnostic
  defaults/hard maxima, 1 MiB frame/body limits, and RFC-0005's 64 nested
  Trait-obligation bound without creating editor-only compiler semantics.
- RFC-0051 §5–§6: registered bilingual `L-LSP-0002`, exact `-32803` data,
  notification response-freedom, failure precedence, privacy, determinism, and
  no partial publication.

## Implementation

- `resource.rs` owns the Preview marker, fixed aggregate/live limits, stable
  resource Facts, and the checked DEC-0033 byte budget.
- `LspServer` reserves exact overlay bytes before open or growth, rolls back a
  failed publication, releases shrink/close bytes, and returns structured
  resource data without URI, source, allocator, request-ID, or timing facts.
- The stdio request registry admits at most 128 distinct live string/number IDs.
  Duplicate detection precedes the quota; rejected requests create no token or
  association; the existing identity-safe completion path releases capacity.
- `ling-types` exports RFC-0005's existing solver nesting constant through the
  compiler DB so discovery cannot drift from the accepted compiler value.
- Existing completion, diagnostic-control, and frame limits remain owned by
  their accepted protocols and are referenced rather than reimplemented.

## Executable evidence

- `resource_limits.rs` consumes the exact fixture and covers discovery, stable
  error shape, per-document over-limit rejection, exact aggregate fill,
  multibyte UTF-8 accounting, atomic over-limit change, close/retry cleanup,
  and a deterministic framed 129th-live-request rejection without sleeps.
- Request-registry unit tests cover the exact 128 boundary, duplicate-first
  precedence, non-admission, completion cleanup, and retry.
- Incremental-change tests retain state and version on a resource failure; the
  exact diagnostic transcript corpus includes additive discovery.
- Existing `ByteBudget`, completion, diagnostic-control, transport, and Trait
  solver tests retain their independently accepted boundaries.

Focused commands executed successfully during implementation:

```text
cargo check -p ling-types -p ling-db -p ling-diagnostics -p ling-lsp --all-targets --locked --offline
cargo test -p ling-lsp --lib --test resource_limits --test overlay --test incremental_changes --test cancellation --locked --offline --quiet
cargo test -p ling-lsp --test diagnostic_transcripts --locked --offline --quiet
```

Repository-wide verification completed successfully on 2026-08-24:

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
cargo xtask trait-performance verify
cargo fmt --all -- --check
git diff --check
manual SHA-256 verification of docs/ling_execution_plan/SHA256SUMS.txt
```

The status verifier reported 499 tracked tasks and 331 Done before this
milestone's separate status-only commit. The checksum audit reported
`execution-plan checksums OK`; no execution-package planning input was changed.

## Compatibility and determinism

- **Protocol:** adds Preview `ling.lsp.resource-limits/0.1` and structured
  `-32803` resource failures; no new method or non-diagnostic configuration.
- **Diagnostics:** allocates `L-LSP-0002`; `L-LSP-0001` and all source
  diagnostic meanings and spans are unchanged.
- **Compiler:** exports the already accepted solver nesting constant; successful
  checked facts, cache keys, ordering, and language acceptance are unchanged.
- **Schema/identity:** no standalone schema, Semantic ID, Definition ID, or
  canonical compiler byte change.
- **Language/runtime/Unicode:** no syntax, semantics, Typed Core evaluation,
  interpreter, runtime, bytecode, VM, ABI, package, host I/O, or Unicode 17.0.0
  change.
- **Determinism/privacy:** decisions use exact bytes/counts only and serialize
  no allocator, host-memory, CPU/load, timing, path, source, or Rust/debug data.

## Intentionally deferred

Allocator/RSS guarantees, OOM recovery, configurable non-diagnostic quotas,
eviction, partial results, progress, deadlines, total compiler fuel, general
workspace/dependency memory accounting, persistence, Stable lifecycle, and
Semantic Transactions remain outside RFC-0051.
