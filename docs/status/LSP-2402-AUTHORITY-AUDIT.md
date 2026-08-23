# LSP-2402 Authority Audit: Typed Semantic-Token Generation

## Outcome

`LSP-2402` is authorized and implemented. Accepted RFC-0046 fixes taxonomy
revision `ling.semantic-token-taxonomy/0.1`; Accepted RFC-0047 fixes the
snapshot-bound in-process generation revision
`ling.semantic-token-generation/0.1`. The implementation is a compiler-owned
`ling-db` index and does not advertise an LSP semantic-token provider.

The earlier `BlockedSpec` conclusion was correct before RFC-0046 and RFC-0047
were accepted. Those authorities now close the taxonomy, source-evidence,
fallback, span, ordering, privacy, and internal lifecycle questions required by
typed generation. Full/delta transport remains separately blocked under
LSP-2403.

## Normative traceability

- RFC-0046 §1–3 fixes the canonical types, modifiers, Seed source-role mapping,
  and modifier exclusivity.
- RFC-0046 §4 permits complete checked generation and only six conservative
  lexer families when checking fails; unresolved identifiers and synthetic or
  erroneous tokens emit nothing.
- RFC-0046 §6–7 fixes original UTF-8 spans, multiline line-local splitting,
  non-overlap, deterministic order, and identity/metadata redaction.
- RFC-0047 §1–4 defines one exact source/revision result, typed versus
  whole-source lexical-fallback mode, checked-identity versus checked-structure
  evidence, HIR/resolver role specialization, and exact modifier propagation.
- RFC-0047 §5 rejects parsed or mixed partial fallback and forbids fallback
  modifiers.
- RFC-0047 §6–7 defines atomic failure, source-span validation, deterministic
  cache identity, privacy, and the non-wire boundary.
- DEC-0084 through DEC-0087 supply the accepted lexical, checked-identity,
  snapshot, and source-fixture observations used by the generator.

## Implemented boundary

`CompilerDb::semantic_token_index` captures the exact source query key and
complete workspace resolve key. A successful parse → AST → HIR → resolve →
type → effect pipeline produces `typed` mode. Failure at any analysis stage
produces a new whole-source `lexical-fallback` result; it never reuses typed
entries from a failed workspace.

The abstract result retains only source identity, VFS revision, logical source
name, mode, original-byte spans, canonical token kinds/modifiers, and internal
evidence class. It retains no Definition/Binding/Reference/Semantic ID, source
text, type display, Effect/Capability metadata, URI, LSP position, document
version, legend index, result ID, path discovery, or transport state.

## Evidence

- `crates/ling-db/src/semantic_token_index.rs` owns classification, fallback,
  line splitting, conflict detection, ordering, and redaction.
- `crates/ling-db/src/lib.rs` owns exact workspace-keyed typed cache reuse and
  failure-isolated fallback construction.
- `crates/ling-db/tests/semantic_tokens.rs` covers current Seed roles,
  callable/non-callable identity, Trait and implementation members, parameters,
  fields, variants and constructor patterns, mutable assignment targets,
  builtin/Prelude identity, user shadowing, parse/resolution/type/effect
  failures, dependency invalidation, Unicode, combining text, emoji, BOM,
  CRLF, multiline splitting, deterministic reuse, and original-byte spans.

## Specification gaps and conflicts

The lower-authority execution plan requested parsed fallback in syntax-error
regions and named Effect/Capability/future token categories. The repository has
no Accepted partial-checked-AST boundary for mixed output, and current inferred
Effect/Capability facts have no exact token role. RFC-0046 and RFC-0047 therefore
require whole-source lexical fallback and exclude those categories.

This resolution changes no Ling language semantics. It narrows the plan to the
provable current Seed source roles instead of fabricating semantic success.

## Compatibility impact

- Adds internal `ling.semantic-token-generation/0.1` and no public wire version.
- Adds no provider, request, response, full/delta result, JSON schema,
  diagnostic allocation, Semantic ID, Definition ID, or canonical bytes.
- Original UTF-8 span truth and Unicode 17.0.0 remain unchanged.
- Runtime, interpreter, VM, bytecode, ABI, packages, filesystem, network, and
  language behavior are unaffected.

## Intentionally deferred

Provider discovery and client legend materialization, URI/document-version and
position projection, full/delta/result-ID transport, cancellation, limits,
freshness publication, wire fixtures, Zed presentation, and Stable lifecycle
remain LSP-2403 through LSP-2504 work. Mixed partial checked/error output also
remains deferred pending separate Accepted authority.
