# LSP-2205 authority audit: diagnostic fixtures

## Outcome

`LSP-2205` is implementation-authorized by Accepted RFC-0035. Its prerequisites
are also Accepted and implemented: RFC-0031 defines diagnostic values,
RFC-0032 push publication, RFC-0033 pull reports, and RFC-0034 root-control and
omission summaries. The earlier missing-authority blocker no longer applies.

The authorized slice is an internal, path-free, exact-byte JSON-RPC transcript
corpus. It adds no public method, compiler diagnostic meaning, editor-specific
contract, or automatic golden update mechanism.

## Normative traceability

- RFC-0035 §§1–3 define the manifest, compact UTF-8/LF payload grammar,
  RFC-0004 framing boundary, exact output comparison, repeated fresh-server
  replay, and host-input isolation.
- RFC-0035 §4 requires Unicode incremental recovery, push/pull parity,
  diagnostic-storm control, and invalid-initialize cases.
- RFC-0035 §5 separates sequential wire evidence from stale-ticket,
  cancellation, related-file, alternate-encoding, oversized, and internal
  failure-injection tests that cannot be represented honestly as transcripts.
- RFC-0031 through RFC-0034 remain the authority for every expected LSP field,
  protocol marker, ordering rule, result ID, cap, and omission diagnostic.
- `docs/ERROR-CODES.md` remains the sole diagnostic allocation source; the
  fixture corpus only records emitted `L-TYPE-0001`, `L-LEX-0004`, and
  `L-LSP-0001` values.

## Resolved plan drift

The previous audit correctly blocked fixtures before the parent protocols were
Accepted, but its current-interface observations became stale after LSP-2201
through LSP-2204 landed. The repository now has an executable stdio host,
negotiated position projection, push notifications, pull full/unchanged
reports, stateless result IDs, deterministic caps, and recovery behavior.

RFC-0035 deliberately does not claim that a sequential transcript can race a
later cancel notification or inject completion of an old analysis ticket.
Those boundaries stay in focused Rust integration tests. This avoids freezing
fabricated messages or treating implementation-only hooks as public protocol.

## Authorized evidence

- `tests/fixtures/lsp-diagnostics-v1/manifest.json` records the four required
  cases and exact active protocol markers.
- Paired `*.input.jsonl` and `*.output.jsonl` files contain raw compact
  JSON-RPC bodies with only path-free Ling URIs.
- `crates/ling-lsp/tests/diagnostic_transcripts.rs` validates grammar and
  metadata, frames the real stdio host, compares exact bytes and order, repeats
  every case in a fresh server, and asserts semantic cross-frame invariants.
- `.gitattributes` fixes the corpus at LF across supported checkout hosts.
- Existing adapter, push, pull, control, position, overlay, cancellation, and
  lifecycle suites provide the non-transcript failure-injection matrix required
  by RFC-0035 §5.

## Compatibility and determinism

The internal fixture marker is Preview test metadata, not a public Ling
protocol. No existing diagnostic code, message, severity, Facts, repair,
Semantic ID, source span, protocol behavior, schema, or result-ID algorithm is
changed. Repeated-process output identity is executable; no host path,
filesystem discovery, environment, network, cache, clock, allocation, or
hash-map iteration enters the corpus.

Unicode behavior remains Unicode 17.0.0. The corpus specifically preserves
original BOM/CRLF/Chinese/emoji source bytes while asserting negotiated UTF-16
wire ranges.

## Intentionally deferred

Dynamic registration, observable asynchronous cancellation, progress, partial
results, background scheduling, editor launch, automatic fixture rewriting,
code-description URLs, repair application, Workspace Edits, Semantic
Transactions, and Stable lifecycle remain outside LSP-2205.
