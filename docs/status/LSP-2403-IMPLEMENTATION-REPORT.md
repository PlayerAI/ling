# LSP-2403 implementation report

## Result

LSP-2403 is complete under Accepted RFC-0048. Implementation commit
`75c2bd0b7d0dcd6e37cdb16e0ec854529f56f97f` adds the Preview
`ling.lsp.semantic-tokens/0.1` provider and standard LSP 3.17 full/delta
transport over the RFC-0047 compiler-owned token index.

## Normative clauses covered

- RFC-0046 §5–7: client-supported canonical legends, category fallback,
  modifier projection, original-span position truth, freshness, and privacy.
- RFC-0047 §1, §5–7: exact immutable generation inputs, conservative
  whole-source fallback, line-local spans, order/non-overlap, and identity
  redaction.
- RFC-0048 §1–2: strict known-capability validation, ignored ordinary unknown
  members, full-only/delta discovery, exact methods, bounded params, lifecycle,
  and notification silence.
- RFC-0048 §3–4: one complete request snapshot, tracked-document selection,
  compiler snapshot construction, same-version recapture, UTF-8/16/32 source
  projection, selected legend indices, modifier bitsets, and standard relative
  five-integer groups.
- RFC-0048 §5–8: client-visible deterministic result identity, bounded FIFO
  history, collision rejection, canonical one-edit deltas, full fallback,
  equivalence, cancellation, token/data/frame limits, atomic failures, privacy,
  and Preview migration boundary.

## Implementation

- Initialize selects the RFC-0046 canonical subset from the client's standard
  type/modifier arrays. A provider is advertised only for `relative`, full
  support, and a nonempty type legend; delta is advertised independently.
- Full requests capture one immutable `RequestSnapshot`, build the compiler
  from its exact bytes/inputs, query `CompilerDb::semantic_token_index`, and
  project every retained line-local original span through DEC-0029.
- Encoding validates monotonic non-overlap and emits standard relative
  `deltaLine`, `deltaStart`, `length`, type-index, and modifier-mask groups.
- Result IDs are `st1-` plus BLAKE3 over length-delimited client-visible
  protocol fields and data only. No source bytes, VFS revision, path, compiler
  key, Definition ID, or Semantic ID is retained or hashed.
- The server retains at most 32 distinct results in FIFO publication order.
  Valid bases use one deterministic longest-prefix/longest-suffix replacement;
  equal results use no edits; invalid, foreign, or expired bases return full.
- Cancellation, snapshot recapture, collision, projection, token/data, and
  frame checks occur before history publication, so failures leave no partial
  state.

## Tests and verification

Seven transport integration tests plus two module unit tests cover:

- full-only, delta, partial, empty, unsupported-format, absent, malformed, and
  unknown capability cases;
- typed and lexical-fallback output, canonical type fallbacks and modifiers,
  temporary and closed documents;
- Chinese, emoji, combining text, BOM, LF/CRLF, multiline splitting, and exact
  UTF-8/16/32 line/character lengths;
- deterministic repeated full results, insertion, deletion, replacement,
  reorder and equal deltas, full/delta equivalence, invalid/foreign/expired
  bases, result-ID shape, and FIFO eviction;
- lifecycle, notifications, invalid params, missing documents, cooperative
  cancellation, token/data overflow, and post-failure recovery.

Focused commands executed successfully:

```text
cargo test -p ling-lsp --test semantic_tokens --locked --offline
cargo test -p ling-lsp --all-targets --locked --offline
cargo clippy -p ling-lsp --all-targets --locked --offline -- -D warnings
```

The final status binding records the repository-wide gate results.

## Specification gaps or conflicts

No unresolved semantic or protocol question required by LSP-2403 remains.
RFC-0048 resolves the former audit blockers without treating the lower-authority
execution plan as semantics. The standard permits multiple unordered edits;
the Preview intentionally fixes one canonical middle replacement to make output
deterministic and independently reproducible.

## Compatibility and determinism

- Adds one public Preview protocol marker, one static provider, two standard
  methods, and bounded session-local presentation state; it makes no Stable or
  persistent compatibility claim.
- No Ling diagnostic, standalone JSON schema, Semantic ID, Definition ID,
  canonical bytes, language semantics, compiler checked facts, runtime,
  bytecode, VM, ABI, package, filesystem/network, or Unicode 17.0.0 change.
- Output is determined by the exact captured snapshot, negotiated encoding,
  selected canonical legend, public token data, and FIFO publication order; no
  hash-map iteration, allocation, timing, thread schedule, host path, or debug
  value is observable.

## Intentionally deferred

LSP-2404 owns the broader semantic-token fixture corpus. Range tokens, refresh,
dynamic registration, partial/work-done results, wire cancellation,
asynchronous scheduling, configurable limits, persistent histories, optimized
multi-edit deltas, mixed checked/error-region output, editor presentation,
Stable lifecycle, and general Semantic Transactions remain future work.
