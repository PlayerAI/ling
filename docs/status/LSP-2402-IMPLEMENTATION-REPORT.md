# LSP-2402 implementation report

## Result

LSP-2402 is complete as the compiler-owned typed semantic-token generation
milestone. Accepted RFC-0047 is implemented by
`CompilerDb::semantic_token_index` at
`ling.semantic-token-generation/0.1`. Implementation commit
`899d00f56d444f43a5128da844e517ef3a85e186` adds the abstract generator and
its executable conformance suite.

The public `PROTO-LSP-SEMANTIC-TOKENS` record remains Future and unimplemented:
this milestone does not add `semanticTokensProvider`, a JSON-RPC method, a
client legend, positions, full/delta data, or result IDs.

## Normative clauses covered

- RFC-0046 §1–4: canonical taxonomy/modifiers, checked identity and structure
  precedence, conservative lexical families, identifier omissions, exact
  mutability, writes, documentation, and default-library roles.
- RFC-0046 §6–7: original-byte spans, line-local multiline segments,
  deterministic non-overlap, and identity/metadata/privacy exclusions.
- RFC-0047 §1–2: exact source/revision identity, typed/fallback modes,
  checked-identity/checked-structure/lexical-fallback evidence, and the
  complete checked-pipeline prerequisite.
- RFC-0047 §3–4: current Seed HIR/resolver role specialization, qualified type
  and Trait roles, callable classification, constructor patterns, record
  fields, import aliases, builtin/Prelude identity, shadowing, and canonical
  modifier propagation.
- RFC-0047 §5–7: whole-source fallback, no fallback modifiers, original-source
  splitting, atomic conflict/overlap rejection, workspace-keyed reuse, and
  non-retention of compiler or wire identities.

## Implementation

- `SemanticTokenKind` and `SemanticTokenModifier` encode RFC-0046 canonical
  order without LSP numeric indices.
- `SemanticTokenGenerationMode` and `SemanticTokenEvidence` preserve the
  internal distinction between complete checked output and conservative
  fallback without exposing it as a future wire category.
- `TypedClassifier` walks only successfully checked resolved HIR. Resolver
  definition, binding, pattern-constructor, and reference targets have higher
  precedence than compiler structure; lexer families are lowest.
- The cache key includes the exact source key and complete workspace resolve
  key, so dependency edits invalidate typed results even when the requested
  file is unchanged. Failed analysis does not enter the typed cache.
- The final builder validates source/UTF-8 boundaries, canonicalizes modifiers,
  rejects equal-precedence conflicts and overlap, splits CRLF/LF multiline
  tokens into nonempty original-byte spans, and publishes atomically.

## Tests and verification

The eight integration tests cover:

- modules, imports, aliases, records, variants, constructors, patterns,
  Traits, implementations, methods, parameters, variables, functions, fields,
  type syntax, literals, comments, keywords, and operators;
- mutable definitions/references and exact assignment `modification`;
- builtin/Prelude `defaultLibrary` and local user shadowing;
- parse, resolution, type, effect, and dependency failures with identifier-free
  whole-source lexical fallback;
- Unicode identifiers, emoji/combining text, BOM, CRLF, multiline comment
  splitting, nonempty original-byte slices, order, non-overlap, cache reuse,
  dependency invalidation, and restoration.

Final verification executes locked offline workspace tests, strict workspace
Clippy, CI, governance, LSP, support, status, RC0, traceability, formatting,
diff, and all execution-package checksum checks.

## Specification gaps or conflicts

The non-normative plan's parsed-error-region fallback conflicts with the
repository rule forbidding unchecked AST interpretation. RFC-0047 resolves the
conflict with atomic whole-source lexical fallback. The plan's custom Effect,
Capability, resource, actor, node, kernel, Semantic-ID, ownership, borrow,
unsafe, and generated categories remain excluded by RFC-0046 because they lack
an exact current source role and compatibility authority.

No additional specification gap was encountered during implementation.

## Compatibility and determinism

- Adds one internal generation marker and query/API; it does not implement or
  version the public LSP semantic-token protocol.
- No diagnostic code, public schema, Semantic ID/canonical-byte rule, language
  behavior, runtime, bytecode, VM, ABI, package, dependency, filesystem,
  network, or Unicode 17.0.0 table changes.
- Output depends only on exact compiler query inputs and canonical source order;
  no hash-map order, clock, allocation, thread schedule, host path, source text,
  debug output, or previous snapshot is observable.

## Intentionally deferred

LSP-2403 owns provider negotiation, client legend indices, URI and document
versions, UTF-8/16/32 positions, full/delta encoding, result IDs/base handling,
freshness, cancellation, limits, and publication. LSP-2404 owns public wire
fixtures. Mixed partial checked/error output and all excluded future categories
remain deferred pending Accepted authority.
