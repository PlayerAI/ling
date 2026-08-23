# LSP-2403 Authority Audit: Full and Delta Tokens

## Outcome

The former `BlockedSpec` condition is closed. Accepted RFC-0046 fixes the
taxonomy and client projection, RFC-0047 fixes snapshot-bound typed generation,
and RFC-0048 now fixes the bounded public full/delta transport as
`ling.lsp.semantic-tokens/0.1`. LSP-2403 may be and has been implemented.

RFC-0048 does not claim that the two broad registered LSP/Semantic Transaction
gaps are globally closed. It resolves only the semantic-token request,
snapshot, position, result-ID, base, retention, cancellation, limit, failure,
privacy, and migration questions needed by this task.

## Normative traceability

- RFC-0046 §1–7 owns the canonical 17 token types, seven possible modifiers,
  fallback projection, checked/lexical evidence precedence, original-span
  truth, line splitting, selected legend, and privacy exclusions.
- RFC-0047 §1–7 owns the complete checked-workspace generator, whole-source
  lexical fallback, immutable source/revision binding, role/modifier mapping,
  deterministic non-overlap, and identity redaction.
- RFC-0048 §1–2 owns capability validation, static discovery, full/delta
  methods, parameter bounds, lifecycle, and disabled-method behavior.
- RFC-0048 §3–4 owns exact immutable request snapshots, same-version freshness,
  source-map position projection, legend mapping, relative integer encoding,
  order, and overlap rejection.
- RFC-0048 §5–8 owns client-visible deterministic result IDs, 32-entry FIFO
  retention, base validation, canonical deltas, full fallback, cancellation,
  limits, fixed failures, atomic publication, privacy, and migration.
- RFC-0004, RFC-0023, RFC-0029, RFC-0030, DEC-0029, DEC-0031, and DEC-0071
  remain the inherited lifecycle, overlay, position, workspace, cancellation,
  and snapshot authorities.

## Former gaps and decisions

The original audit correctly stopped implementation because taxonomy,
generation, legend, result identity, full/delta encoding, base validation,
freshness, cancellation, limits, and lifecycle were missing. Those specific
questions are now resolved by RFC-0046 through RFC-0048.

The implementation deliberately does not reuse VFS revisions, compiler query
keys, source hashes, Semantic IDs, or Definition IDs as public result IDs.
RFC-0048 instead hashes only client-visible protocol inputs. Invalid, expired,
or foreign-document bases recover to a full standard response rather than an
error or guessed edit.

## Acceptance evidence

- `crates/ling-db/src/semantic_token_index.rs` supplies only RFC-0047 abstract
  tokens over original UTF-8 spans.
- `crates/ling-lsp/src/semantic_tokens.rs` owns negotiation, source-map
  projection, relative encoding, bounded history, result identity, canonical
  delta construction, freshness, cancellation, and atomic publication.
- `crates/ling-lsp/tests/semantic_tokens.rs` covers full-only and delta
  negotiation, malformed and empty legends, UTF-8/16/32, Chinese, emoji,
  combining text, BOM, CRLF, multiline spans, insertion/deletion/reordering,
  equality, invalid/foreign/expired bases, FIFO eviction, temporary/closed
  documents, conservative fallback, cancellation, lifecycle, and limits.
- `tests/protocols/lsp-semantic-tokens/README.md` records the executable public
  protocol fixture boundary.
- `PROTO-LSP-SEMANTIC-TOKENS` is registered as implemented Preview, not Stable.

## Compatibility and deferred work

No Ling syntax, semantics, Typed Core evaluation, diagnostic code, standalone
schema, Semantic ID, canonical byte, runtime, bytecode, VM, ABI, package,
filesystem/network, or Unicode 17.0.0 behavior changes.

Range requests, refresh, dynamic registration, partial/work-done results, wire
`$/cancelRequest`, asynchronous scheduling, configurable limits, persistent
history, multi-edit optimization, mixed checked/error-region output, editor
presentation, Stable lifecycle, and general Semantic Transactions remain
intentionally deferred.
