# LSP-2404 implementation report

## Result

LSP-2404 is complete under Accepted RFC-0046, RFC-0047, and RFC-0048.
Implementation commit `9105ff5be4aad29b471d5997594156a923f5cb56`
adds the deterministic `ling.test.lsp-semantic-tokens/1` corpus and an
independent executable reader for the existing Preview
`ling.lsp.semantic-tokens/0.1` surface.

## Normative clauses covered

- RFC-0046 §1–3: canonical legend, source-role mapping, scoped identity,
  mutable fields, variant constructors, and modifiers.
- RFC-0046 §4–7: evidence precedence, conservative recovery, position truth,
  order/non-overlap, privacy, and Effect/Capability category exclusion.
- RFC-0047 §1–4: snapshot-bound checked generation, shadowing, field and
  constructor roles, assignments, and modifier propagation.
- RFC-0047 §5–7: whole-source lexical fallback, original UTF-8 spans, atomic
  generation, unchecked-AST prohibition, and private-data exclusion.
- RFC-0048 §1–8: Preview marker, legend and position negotiation, tracked
  document versions, exact relative data, deterministic result IDs, canonical
  deltas, equivalence, bounds, freshness, cancellation, privacy, and migration.

## Fixture corpus and reader

`tests/protocols/lsp-semantic-tokens/fixtures/v1.json` contains four exact
cases:

1. UTF-16 projection of BOM/CRLF source with an emoji prefix, combining text,
   and Chinese identifier columns;
2. checked same-spelling values across scopes, a mutable field and write,
   variant constructors/patterns, and exclusion of `Console.Write` names in a
   `requires` clause;
3. failed checking with whole-source, unmodified lexical-family fallback; and
4. deterministic base full output, one exact canonical insertion delta, and
   the equivalent current full output.

The integration reader in `crates/ling-lsp/tests/semantic_tokens.rs` validates
the format and three Accepted version markers, canonical legend, unique case
names, exact complete JSON-RPC results, fresh-session determinism, result-ID
shape, source order/non-overlap, case-specific semantic invariants, and delta
reapplication. Existing transport tests cover partial legends, UTF-8/UTF-32,
invalid/foreign/expired bases, FIFO retention, temporary/closed documents,
malformed inputs, cancellation, and limits without duplicating those cases.

## Verification

The focused semantic-token suite and the repository's locked offline workspace
tests, strict Clippy, CI, governance, LSP, support, status, RC0, v0.0.1
traceability, formatting, diff, and execution-plan checksum gates pass against
the implementation commit plus its deterministic status binding.

## Specification gaps or conflicts

No unresolved LSP-2404 semantic or public-protocol decision remains. The old
authority audit was superseded by Accepted RFC-0046/RFC-0047/RFC-0048. The
fixture-format marker is deliberately test-only, so the task does not invent a
standalone public schema or migration surface.

## Compatibility, determinism, and Unicode impact

- Public semantic-token behavior remains Preview
  `ling.lsp.semantic-tokens/0.1`; the corpus freezes existing behavior and adds
  no provider, method, field, category, or Stable claim.
- Exact source, URI, version, encoding, legend, token data, and public result-ID
  inputs determine every expected value. No map order, allocation, timing,
  thread schedule, path, VFS revision, debug value, or private identity enters
  output.
- No diagnostic, Semantic ID, canonical bytes, language semantics, Typed Core,
  interpreter, runtime, bytecode, VM, ABI, package, dependency, filesystem,
  network, or Unicode 17.0.0 table changes.

## Intentionally deferred

Range tokens, refresh, dynamic registration, partial/work-done results, wire
cancellation, mixed checked/error-region generation, editor presentation,
persistent result histories, Stable lifecycle, and general Semantic
Transactions remain future work outside LSP-2404.
