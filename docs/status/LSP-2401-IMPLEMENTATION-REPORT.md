# LSP-2401 implementation report

## Result

LSP-2401 is complete as the semantic-token taxonomy RFC/decision milestone.
Accepted RFC-0046 defines `ling.semantic-token-taxonomy/0.1`; the planned public
semantic-token protocol is registered as Future and remains unimplemented until
LSP-2402 typed generation and LSP-2403 transport are complete.

## Normative clauses covered

- RFC-0046 §1 fixes an 18-type canonical standard legend and six standard
  modifiers plus optional custom `mutable`.
- §2–3 define exact Seed source-role mapping and declaration, definition,
  readonly, mutable, modification, documentation, and default-library facts.
- §4 defines checked, structural, and lexical evidence precedence and the
  conservative non-identifier fallback for invalid checked analysis.
- §5 defines deterministic client-supported token-type fallback, modifier
  filtering, deduplication, and the empty-legend no-provider rule.
- §6 fixes original UTF-8 span truth, multiline line splitting, negotiated
  UTF-8/16/32 projection, order, non-overlap, and snapshot/version freshness.
- §7 prohibits identity, inferred metadata, paths, and unimplemented future
  concepts from entering semantic-token data.

## Deliverables and validation

- `docs/RFC-0046.md` was created from the governance RFC template and advanced
  through Open, Draft, Proposed, and Accepted in the lifecycle registry.
- Authority, lifecycle, protocol, support, and RC0 records are updated and
  deterministically rendered.
- `PROTO-LSP-SEMANTIC-TOKENS` accurately records a Planned public Future
  surface with no current version, fixture, schema, provider, or implementation.
- Governance self-tests validate 321 authority documents, 296 lifecycle
  records, and 46 protocols without changing current public support claims.

## Specification gaps or conflicts

The non-normative execution plan proposed custom Effect, Capability, resource,
actor, node, kernel, Semantic-ID, borrow, unsafe, and generated categories.
Current Seed sources do not supply exact public spans and privacy/migration
authority for those concepts. RFC-0046 rejects them for version 0.1 instead of
turning inferred metadata or future features into a wire promise.

Tree-sitter captures remain lexical editor fallback, not semantic truth.
RFC-0046 uses the compiler-owned lexical, checked-identity, snapshot, and
fixture observations accepted by DEC-0084 through DEC-0087.

## Compatibility and determinism

- Adds one Accepted taxonomy revision and one unimplemented Future protocol
  inventory record; no client observes a new provider or request.
- Legend projection is a pure ordered function of canonical taxonomy and client
  support. Checked identity, structural roles, and lexical fallback have a
  total precedence; no map, clock, path, allocation, or previous snapshot is
  observable.
- Original UTF-8 spans and Unicode 17.0.0 remain authoritative; no normalization
  or capitalization heuristic classifies source names.
- No diagnostic, schema, Semantic ID/canonical-byte, language/runtime, VM,
  bytecode, ABI, package, filesystem, or network behavior changes.

## Intentionally deferred

Typed generation, provider negotiation implementation, full/delta/result IDs,
request/document scope, temporary/dependency handling, caching, cancellation,
limits, stale failures, fixture schemas, Zed presentation, Stable lifecycle,
Semantic Transactions, and all excluded future categories remain later tasks.
