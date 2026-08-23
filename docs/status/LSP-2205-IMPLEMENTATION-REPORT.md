# LSP-2205 implementation report

> Status: Done
> Task: `LSP-2205`
> Authority: Accepted `RFC-0035`, `RFC-0034`, `RFC-0033`, `RFC-0032`,
> `RFC-0031`, `RFC-0004`, `DEC-0001`, `DEC-0002`, `DEC-0029`, `DEC-0034`,
> `DEC-0071`, and `DEC-0072`

## Scope

This milestone adds an internal Preview corpus of exact raw JSON-RPC diagnostic
transcripts and one conformance harness. It exercises the real framed stdio
server independently of Zed and does not add or modify a public method.

## Normative clauses covered

- RFC-0035 §§1–2: exact manifest, safe case paths, compact UTF-8/LF JSON object
  payloads, strict framing, decoding, order, count, and body-byte comparison.
- RFC-0035 §3: fresh-server repeated execution, terminal state and exit-code
  checks, path-free Ling URIs, and host-input isolation.
- RFC-0035 §4: required Unicode recovery, push/pull parity, storm-control
  recovery, and invalid-initialize transcript cases.
- RFC-0035 §5: honest separation of wire-observable behavior from focused
  stale/cancellation/internal failure-injection tests.

## Implementation and fixtures

- `diagnostic_transcripts.rs` validates exact manifest members, sorted and
  unique case IDs/files, active protocol markers, canonical payload bytes, and
  LF checkout integrity before executing any case.
- It frames each raw input body through `run_stdio`, strictly decodes every
  output frame, compares exact expected bodies, repeats the complete run, and
  validates recovery, parity, omission-summary, and error invariants.
- `push-unicode-recovery` covers BOM, CRLF, Chinese identifiers/text, a later
  emoji, a ranged edit before it, exact bilingual `L-TYPE-0001`, and versioned
  empty replacement.
- `pull-parity-recovery` proves exact push/pull diagnostic-array equality,
  stateless result IDs, source repair, push clearance, and a full empty pull.
- `storm-control-recovery` records retained `L-LEX-0004`, exact
  `L-LSP-0001` capped-count Facts, and later summary clearance.
- `invalid-control-initialize` records InvalidParams and lifecycle failure
  atomicity for a known out-of-range limit.

## Tests and evidence

- The dedicated locked/offline transcript integration test passes.
- Existing LSP adapter, push, pull, diagnostic-control, position, overlay,
  cancellation, lifecycle, and oversized-response tests retain the focused
  evidence referenced by RFC-0035 §5.
- The complete repository gate set passed against implementation commit
  `93a58e9090ce5a3be17bcfb8569d7246ce7d71ec` before status binding.

## Compatibility, determinism, and Unicode impact

- Adds only internal Preview `ling.test.lsp-diagnostic-transcripts/1` fixture
  metadata; public protocol markers and behavior are unchanged.
- Expected response bodies are byte-exact and execute twice in fresh servers;
  no update/bless mode exists.
- No diagnostic allocation, language semantics, Typed Core, runtime, bytecode,
  VM, ABI, public schema, Semantic ID, or result-ID change occurs.
- Unicode remains 17.0.0; exact source bytes and UTF-16 projections are
  preserved across LF-only fixture checkout.

## Intentionally deferred

Editor-driven snapshots, dynamic registration, observable asynchronous
cancellation, progress, partial results, background scheduling, automatic
fixture updates, fixes, Workspace Edits, Semantic Transactions, and Stable
lifecycle remain future work.
